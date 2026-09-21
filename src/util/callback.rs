use {
    crate::alloc::boxed::Box,
    core::{
        marker::PhantomData,
        mem::{drop, transmute},
        ops::Fn,
    },
};

enum PtrType {
    // The first pointer is null, because a boxed stateless function has 0x1 in its first `usize`,
    // experimentally.
    Null,
    Function,
    Count,
}

type Ptr = *const ();
type PtrArray = [Ptr; PtrType::Count as usize];

pub struct Callback<I = (), O = ()> {
    ptrs: PtrArray,
    _input: PhantomData<I>,
    _output: PhantomData<O>,
}

impl<I, O> Callback<I, O> {
    pub fn from_function(function: fn(I) -> O) -> Self {
        let mut ptrs: PtrArray = PtrArray::default();

        ptrs[PtrType::Function as usize] = function as Ptr;

        Self {
            ptrs,
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn from_closure(closure: Box<dyn Fn(I) -> O>) -> Self {
        Self {
            ptrs: unsafe { transmute(closure) },
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn is_function(&self) -> bool {
        self.ptrs[PtrType::Null as usize].is_null()
    }

    pub fn is_closure(&self) -> bool {
        !self.is_function()
    }

    pub fn invoke(&self, input: I) -> O {
        if self.is_function() {
            (unsafe { transmute::<Ptr, fn(I) -> O>(self.ptrs[PtrType::Function as usize]) })(input)
        } else {
            (unsafe { transmute::<&PtrArray, &Box<dyn Fn(I) -> O>>(&self.ptrs) })(input)
        }
    }
}

impl<I, O> Drop for Callback<I, O> {
    fn drop(&mut self) {
        // If we have a boxed closure, we need it to be dropped normally to release any heap memory.
        if self.is_closure() {
            drop(unsafe { transmute::<PtrArray, Box<dyn Fn(I) -> O>>(self.ptrs) });
        }
    }
}

impl<I, O> From<fn(I) -> O> for Callback<I, O> {
    fn from(function: fn(I) -> O) -> Self {
        Self::from_function(function)
    }
}

impl<I, O> From<Box<dyn Fn(I) -> O>> for Callback<I, O> {
    fn from(closure: Box<dyn Fn(I) -> O>) -> Self {
        Self::from_closure(closure)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::sync::{
            Mutex, MutexGuard,
            atomic::{AtomicI32, Ordering},
        },
    };

    // We're not doing enough for this to be a memory concern, and choosing this means that we don't
    // need to be specific at each callsite, while still having behavior be what's expected.
    const ORDER: Ordering = Ordering::SeqCst;

    // NOTE: This must be accessed only from serial tests, otherwise they could fail non
    // deterministically.
    static X: AtomicI32 = AtomicI32::new(0_i32);
    static X_MUTEX: Mutex<()> = Mutex::new(());

    fn verify_x(value: i32) {
        assert_eq!(X.load(ORDER), value);
    }

    fn set_x(value: i32) {
        X.store(value, ORDER);
    }

    fn reset_x() {
        set_x(0_i32);
    }

    fn test_increment_x(increment_x: &Callback, is_function: bool, is_closure: bool) {
        reset_x();
        assert_eq!(increment_x.is_function(), is_function);
        assert_eq!(increment_x.is_closure(), is_closure);
        verify_x(0_i32);
        assert_eq!(increment_x.invoke(()), ());
        verify_x(1_i32);
        assert_eq!(increment_x.invoke(()), ());
        verify_x(2_i32);
        assert_eq!(increment_x.invoke(()), ());
        verify_x(3_i32);
    }

    fn test_add_assign_x(add_assign_x: &Callback<i32>, is_function: bool, is_closure: bool) {
        reset_x();
        assert_eq!(add_assign_x.is_function(), is_function);
        assert_eq!(add_assign_x.is_closure(), is_closure);
        verify_x(0_i32);
        assert_eq!(add_assign_x.invoke(1_i32), ());
        verify_x(1_i32);
        assert_eq!(add_assign_x.invoke(2_i32), ());
        verify_x(3_i32);
        assert_eq!(add_assign_x.invoke(3_i32), ());
        verify_x(6_i32);
    }

    fn test_get_x(get_x: &Callback<(), i32>, is_function: bool, is_closure: bool) {
        reset_x();
        assert_eq!(get_x.is_function(), is_function);
        assert_eq!(get_x.is_closure(), is_closure);
        verify_x(0_i32);
        assert_eq!(get_x.invoke(()), 0_i32);
        verify_x(0_i32);
        set_x(1_i32);
        assert_eq!(get_x.invoke(()), 1_i32);
        verify_x(1_i32);
        set_x(2_i32);
        assert_eq!(get_x.invoke(()), 2_i32);
        verify_x(2_i32);
    }

    fn test_add_assign_and_get_x(
        add_assign_and_get_x: &Callback<i32, i32>,
        is_function: bool,
        is_closure: bool,
    ) {
        reset_x();
        assert_eq!(add_assign_and_get_x.is_function(), is_function);
        assert_eq!(add_assign_and_get_x.is_closure(), is_closure);
        verify_x(0_i32);
        assert_eq!(add_assign_and_get_x.invoke(1_i32), 1_i32);
        verify_x(1_i32);
        assert_eq!(add_assign_and_get_x.invoke(2_i32), 3_i32);
        verify_x(3_i32);
        assert_eq!(add_assign_and_get_x.invoke(3_i32), 6_i32);
        verify_x(6_i32);
    }

    fn test_callbacks(
        increment_x: &Callback,
        add_assign_x: &Callback<i32>,
        get_x: &Callback<(), i32>,
        add_assign_and_get_x: &Callback<i32, i32>,
        are_callbacks_functions: bool,
        are_callbacks_closures: bool,
    ) {
        test_increment_x(increment_x, are_callbacks_functions, are_callbacks_closures);
        test_add_assign_x(
            add_assign_x,
            are_callbacks_functions,
            are_callbacks_closures,
        );
        test_get_x(get_x, are_callbacks_functions, are_callbacks_closures);
        test_add_assign_and_get_x(
            add_assign_and_get_x,
            are_callbacks_functions,
            are_callbacks_closures,
        );
    }

    fn test_functions(
        increment_x: fn(()),
        add_assign_x: fn(i32),
        get_x: fn(()) -> i32,
        add_assign_and_get_x: fn(i32) -> i32,
    ) {
        reset_x();

        let increment_x: Callback = Callback::from_function(increment_x);

        verify_x(0_i32);

        let add_assign_x: Callback<i32> = Callback::from_function(add_assign_x);

        verify_x(0_i32);

        let get_x: Callback<(), i32> = Callback::from_function(get_x);

        verify_x(0_i32);

        let add_assign_and_get_x: Callback<i32, i32> =
            Callback::from_function(add_assign_and_get_x);

        verify_x(0_i32);
        test_callbacks(
            &increment_x,
            &add_assign_x,
            &get_x,
            &add_assign_and_get_x,
            true,  // are_callbacks_functions
            false, // are_callbacks_closures
        );
    }

    fn test_closures(
        increment_x: Box<dyn Fn(())>,
        add_assign_x: Box<dyn Fn(i32)>,
        get_x: Box<dyn Fn(()) -> i32>,
        add_assign_and_get_x: Box<dyn Fn(i32) -> i32>,
    ) {
        reset_x();

        let increment_x: Callback = Callback::from_closure(increment_x);

        verify_x(0_i32);

        let add_assign_x: Callback<i32> = Callback::from_closure(add_assign_x);

        verify_x(0_i32);

        let get_x: Callback<(), i32> = Callback::from_closure(get_x);

        verify_x(0_i32);

        let add_assign_and_get_x: Callback<i32, i32> = Callback::from_closure(add_assign_and_get_x);

        verify_x(0_i32);
        test_callbacks(
            &increment_x,
            &add_assign_x,
            &get_x,
            &add_assign_and_get_x,
            false, // are_callbacks_functions
            true,  // are_callbacks_closures
        );
    }

    #[test]
    fn test_standard_functions() {
        let _x_mutex_guard: MutexGuard<()> = X_MUTEX.lock().unwrap();

        fn increment_x(_: ()) {
            X.fetch_add(1_i32, ORDER);
        }

        fn add_assign_x(value: i32) {
            X.fetch_add(value, ORDER);
        }

        fn get_x(_: ()) -> i32 {
            X.load(ORDER)
        }

        fn add_assign_and_get_x(value: i32) -> i32 {
            X.fetch_add(value, ORDER);

            X.load(ORDER)
        }

        test_functions(increment_x, add_assign_x, get_x, add_assign_and_get_x);
    }

    #[test]
    fn test_non_capturing_closures() {
        let _x_mutex_guard: MutexGuard<()> = X_MUTEX.lock().unwrap();

        let increment_x = |_: ()| {
            X.fetch_add(1_i32, ORDER);
        };

        let add_assign_x = |value: i32| {
            X.fetch_add(value, ORDER);
        };

        let get_x = |_: ()| -> i32 { X.load(ORDER) };

        let add_assign_and_get_x = |value: i32| -> i32 {
            X.fetch_add(value, ORDER);

            X.load(ORDER)
        };

        // Non-capturing closures can be cast to function pointers.
        test_functions(increment_x, add_assign_x, get_x, add_assign_and_get_x);

        test_closures(
            Box::new(increment_x),
            Box::new(add_assign_x),
            Box::new(get_x),
            Box::new(add_assign_and_get_x),
        );
    }

    #[test]
    fn test_capturing_closures() {
        let _x_mutex_guard: MutexGuard<()> = X_MUTEX.lock().unwrap();
        let captured: bool = true;

        let increment_x = move |_: ()| {
            assert!(captured);

            X.fetch_add(1_i32, ORDER);
        };

        let add_assign_x = move |value: i32| {
            assert!(captured);

            X.fetch_add(value, ORDER);
        };

        let get_x = move |_: ()| -> i32 {
            assert!(captured);

            X.load(ORDER)
        };

        let add_assign_and_get_x = move |value: i32| -> i32 {
            assert!(captured);

            X.fetch_add(value, ORDER);

            X.load(ORDER)
        };

        test_closures(
            Box::new(increment_x),
            Box::new(add_assign_x),
            Box::new(get_x),
            Box::new(add_assign_and_get_x),
        );

        struct Capturable;

        impl Capturable {
            fn is_captured(&self) -> bool {
                true
            }
        }

        // Give `Capturable`'s `drop` a visible side effect, like freeing owned memory.
        impl Drop for Capturable {
            fn drop(&mut self) {
                X.fetch_add(-1_i32, ORDER);
            }
        }

        let capturable: Capturable = Capturable;
        let capturing_closure_with_on_drop_side_effect = move |()| {
            assert!(capturable.is_captured());

            X.fetch_add(1_i32, ORDER);
        };

        let capturing_closure_with_on_drop_side_effect_callback: Callback =
            Callback::from_closure(Box::new(capturing_closure_with_on_drop_side_effect));

        reset_x();
        assert_eq!(
            capturing_closure_with_on_drop_side_effect_callback.invoke(()),
            ()
        );
        verify_x(1_i32);
        assert_eq!(
            capturing_closure_with_on_drop_side_effect_callback.invoke(()),
            ()
        );
        verify_x(2_i32);
        assert_eq!(
            capturing_closure_with_on_drop_side_effect_callback.invoke(()),
            ()
        );
        verify_x(3_i32);
        drop(capturing_closure_with_on_drop_side_effect_callback);
        verify_x(2_i32);
    }
}
