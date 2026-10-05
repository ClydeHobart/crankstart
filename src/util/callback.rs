use {
    crate::{alloc::boxed::Box, define_enum_count, util::r#enum::count::EnumCount},
    bitvec::{order::Lsb0, view::BitView},
    core::{
        marker::PhantomData,
        mem::{drop, transmute},
        ops::{Deref, Fn},
    },
};

define_enum_count! {
    #[repr(u8)]
    enum PtrType {
        // The first pointer is null, because a boxed stateless function has 0x1 in its first `usize`,
        // experimentally.
        Null,
        Function,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CallbackType {
    FuncPtr,
    BoxedClosure,
    TempClosure,
}

type CallbackPtrs = [usize; PtrType::COUNT];

pub struct Callback<I = (), O = ()> {
    ptrs: CallbackPtrs,
    _input: PhantomData<I>,
    _output: PhantomData<O>,
}

impl<I, O> Callback<I, O> {
    pub fn from_func_ptr(func_ptr: fn(I) -> O) -> Self {
        let mut ptrs: CallbackPtrs = CallbackPtrs::default();

        ptrs[PtrType::Function as usize] = func_ptr as *const () as usize;

        Self {
            ptrs,
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn from_boxed_closure(boxed_closure: Box<dyn Fn(I) -> O>) -> Self {
        Self {
            ptrs: unsafe { transmute(boxed_closure) },
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn is_func_ptr(&self) -> bool {
        Self::is_func_ptr_internal(&self.ptrs)
    }

    pub fn is_closure(&self) -> bool {
        !self.is_func_ptr()
    }

    pub fn is_boxed_closure(&self) -> bool {
        self.is_closure() && !Self::get_temp_closure_bit(&self.ptrs)
    }

    pub fn is_temp_closure(&self) -> bool {
        self.is_closure() && Self::get_temp_closure_bit(&self.ptrs)
    }

    pub fn get_type(&self) -> CallbackType {
        match (self.is_func_ptr(), Self::get_temp_closure_bit(&self.ptrs)) {
            (true, _) => CallbackType::FuncPtr,
            (false, false) => CallbackType::BoxedClosure,
            (false, true) => CallbackType::TempClosure,
        }
    }

    pub fn invoke(&self, input: I) -> O {
        match self.get_type() {
            CallbackType::FuncPtr => {
                (unsafe { transmute::<usize, fn(I) -> O>(self.ptrs[PtrType::Function as usize]) })(
                    input,
                )
            }
            CallbackType::BoxedClosure => {
                (unsafe { transmute::<&CallbackPtrs, &Box<dyn Fn(I) -> O>>(&self.ptrs) })(input)
            }
            CallbackType::TempClosure => {
                let mut ptrs: CallbackPtrs = self.ptrs;

                Self::clear_temp_closure_bit(&mut ptrs);

                (unsafe { transmute::<&mut CallbackPtrs, &mut Box<dyn FnMut(I) -> O>>(&mut ptrs) })(
                    input,
                )
            }
        }
    }

    const TEMP_CLOSURE_BIT: usize = 0_usize;

    unsafe fn from_temp_closure<'t>(temp_closure: &'t mut dyn FnMut(I) -> O) -> Self {
        let mut ptrs: CallbackPtrs = unsafe { transmute(temp_closure) };

        Self::set_temp_closure_bit(&mut ptrs);

        Self {
            ptrs,
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    fn is_func_ptr_internal(ptrs: &CallbackPtrs) -> bool {
        ptrs[PtrType::Null as usize] == 0_usize
    }

    fn is_closure_internal(ptrs: &CallbackPtrs) -> bool {
        !Self::is_func_ptr_internal(ptrs)
    }

    fn get_temp_closure_bit(ptrs: &CallbackPtrs) -> bool {
        ptrs[PtrType::Function as usize].view_bits::<Lsb0>()[Self::TEMP_CLOSURE_BIT]
    }

    fn set_temp_closure_bit(ptrs: &mut CallbackPtrs) {
        Self::set_temp_closure_bit_internal(ptrs, true);
    }

    fn clear_temp_closure_bit(ptrs: &mut CallbackPtrs) {
        Self::set_temp_closure_bit_internal(ptrs, false);
    }

    fn set_temp_closure_bit_internal(ptrs: &mut CallbackPtrs, value: bool) {
        assert!(Self::is_closure_internal(ptrs));

        ptrs[PtrType::Function as usize]
            .view_bits_mut::<Lsb0>()
            .set(Self::TEMP_CLOSURE_BIT, value);
    }
}

impl<I, O> Drop for Callback<I, O> {
    fn drop(&mut self) {
        // If we have a boxed closure, we need it to be dropped normally to release any heap memory.
        if self.is_boxed_closure() {
            drop(unsafe { transmute::<CallbackPtrs, Box<dyn Fn(I) -> O>>(self.ptrs) });
        }
    }
}

impl<I, O> From<fn(I) -> O> for Callback<I, O> {
    fn from(func_ptr: fn(I) -> O) -> Self {
        Self::from_func_ptr(func_ptr)
    }
}

impl<I, O> From<Box<dyn Fn(I) -> O>> for Callback<I, O> {
    fn from(boxed_closure: Box<dyn Fn(I) -> O>) -> Self {
        Self::from_boxed_closure(boxed_closure)
    }
}

pub struct TempCallback<'t, I = (), O = ()> {
    callback: Callback<I, O>,
    _lifetime: PhantomData<&'t ()>,
}

impl<'t, I, O> TempCallback<'t, I, O> {
    pub fn from_temp_closure(temp_closure: &'t mut dyn FnMut(I) -> O) -> Self {
        Self {
            callback: unsafe { Callback::from_temp_closure(temp_closure) },
            _lifetime: PhantomData,
        }
    }
}

impl<'t, I, O> AsRef<Callback<I, O>> for TempCallback<'t, I, O> {
    fn as_ref(&self) -> &Callback<I, O> {
        self.deref()
    }
}

impl<'t, I, O> Deref for TempCallback<'t, I, O> {
    type Target = Callback<I, O>;

    fn deref(&self) -> &Self::Target {
        &self.callback
    }
}

impl<'t, I, O> From<&'t mut dyn FnMut(I) -> O> for TempCallback<'t, I, O> {
    fn from(temp_closure: &'t mut dyn FnMut(I) -> O) -> Self {
        Self::from_temp_closure(temp_closure)
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

    struct Capturable;

    impl Capturable {
        fn is_captured(&self) -> bool {
            true
        }

        fn increment_x(&mut self) {
            X.fetch_add(1_i32, ORDER);
        }
    }

    // Give `Capturable`'s `drop` a visible side effect, like freeing owned memory.
    impl Drop for Capturable {
        fn drop(&mut self) {
            X.fetch_add(-1_i32, ORDER);
        }
    }

    fn verify_x(value: i32) {
        assert_eq!(X.load(ORDER), value);
    }

    fn set_x(value: i32) {
        X.store(value, ORDER);
    }

    fn reset_x() {
        set_x(0_i32);
    }

    fn test_increment_x(increment_x: &Callback, callback_type: CallbackType) {
        reset_x();
        assert_eq!(increment_x.get_type(), callback_type);
        verify_x(0_i32);
        assert_eq!(increment_x.invoke(()), ());
        verify_x(1_i32);
        assert_eq!(increment_x.invoke(()), ());
        verify_x(2_i32);
        assert_eq!(increment_x.invoke(()), ());
        verify_x(3_i32);
    }

    fn test_add_assign_x(add_assign_x: &Callback<i32>, callback_type: CallbackType) {
        reset_x();
        assert_eq!(add_assign_x.get_type(), callback_type);
        verify_x(0_i32);
        assert_eq!(add_assign_x.invoke(1_i32), ());
        verify_x(1_i32);
        assert_eq!(add_assign_x.invoke(2_i32), ());
        verify_x(3_i32);
        assert_eq!(add_assign_x.invoke(3_i32), ());
        verify_x(6_i32);
    }

    fn test_get_x(get_x: &Callback<(), i32>, callback_type: CallbackType) {
        reset_x();
        assert_eq!(get_x.get_type(), callback_type);
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
        callback_type: CallbackType,
    ) {
        reset_x();
        assert_eq!(add_assign_and_get_x.get_type(), callback_type);
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
        callback_type: CallbackType,
    ) {
        test_increment_x(increment_x, callback_type);
        test_add_assign_x(add_assign_x, callback_type);
        test_get_x(get_x, callback_type);
        test_add_assign_and_get_x(add_assign_and_get_x, callback_type);
    }

    fn test_func_ptrs(
        increment_x: fn(()),
        add_assign_x: fn(i32),
        get_x: fn(()) -> i32,
        add_assign_and_get_x: fn(i32) -> i32,
    ) {
        reset_x();

        let increment_x: Callback = Callback::from_func_ptr(increment_x);

        verify_x(0_i32);

        let add_assign_x: Callback<i32> = Callback::from_func_ptr(add_assign_x);

        verify_x(0_i32);

        let get_x: Callback<(), i32> = Callback::from_func_ptr(get_x);

        verify_x(0_i32);

        let add_assign_and_get_x: Callback<i32, i32> =
            Callback::from_func_ptr(add_assign_and_get_x);

        verify_x(0_i32);
        test_callbacks(
            &increment_x,
            &add_assign_x,
            &get_x,
            &add_assign_and_get_x,
            CallbackType::FuncPtr,
        );
    }

    fn test_closures<
        IX: Fn(()) + 'static,
        AAX: Fn(i32) + 'static,
        GX: Fn(()) -> i32 + 'static,
        AAAGX: Fn(i32) -> i32 + 'static,
    >(
        mut increment_x: IX,
        mut add_assign_x: AAX,
        mut get_x: GX,
        mut add_assign_and_get_x: AAAGX,
    ) {
        // Temp closures
        {
            reset_x();

            let increment_x: TempCallback = TempCallback::from_temp_closure(&mut increment_x);

            verify_x(0_i32);

            let add_assign_x: TempCallback<i32> =
                TempCallback::from_temp_closure(&mut add_assign_x);

            verify_x(0_i32);

            let get_x: TempCallback<(), i32> = TempCallback::from_temp_closure(&mut get_x);

            verify_x(0_i32);

            let add_assign_and_get_x: TempCallback<i32, i32> =
                TempCallback::from_temp_closure(&mut add_assign_and_get_x);

            verify_x(0_i32);
            test_callbacks(
                &increment_x,
                &add_assign_x,
                &get_x,
                &add_assign_and_get_x,
                CallbackType::TempClosure,
            );
        }

        // Boxed closures
        {
            reset_x();

            let increment_x: Callback = Callback::from_boxed_closure(Box::new(increment_x));

            verify_x(0_i32);

            let add_assign_x: Callback<i32> = Callback::from_boxed_closure(Box::new(add_assign_x));

            verify_x(0_i32);

            let get_x: Callback<(), i32> = Callback::from_boxed_closure(Box::new(get_x));

            verify_x(0_i32);

            let add_assign_and_get_x: Callback<i32, i32> =
                Callback::from_boxed_closure(Box::new(add_assign_and_get_x));

            verify_x(0_i32);
            test_callbacks(
                &increment_x,
                &add_assign_x,
                &get_x,
                &add_assign_and_get_x,
                CallbackType::BoxedClosure,
            );
        }
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

        test_func_ptrs(increment_x, add_assign_x, get_x, add_assign_and_get_x);
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
        test_func_ptrs(increment_x, add_assign_x, get_x, add_assign_and_get_x);

        test_closures(increment_x, add_assign_x, get_x, add_assign_and_get_x);
    }

    #[test]
    fn test_capturing_boxed_closures() {
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

        test_closures(increment_x, add_assign_x, get_x, add_assign_and_get_x);
    }

    #[test]
    fn test_capturable_as_fn() {
        let _x_mutex_guard: MutexGuard<()> = X_MUTEX.lock().unwrap();
        let capturable: Capturable = Capturable;
        let capturing_closure_with_on_drop_side_effect = move |()| {
            assert!(capturable.is_captured());

            X.fetch_add(1_i32, ORDER);
        };
        let capturing_closure_with_on_drop_side_effect_callback: Callback =
            Callback::from_boxed_closure(Box::new(capturing_closure_with_on_drop_side_effect));

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

    #[test]
    fn test_capturable_as_fn_mut() {
        let _x_mutex_guard: MutexGuard<()> = X_MUTEX.lock().unwrap();
        let mut capturable: Capturable = Capturable;
        let mut capturing_closure_with_on_drop_side_effect = |()| {
            assert!(capturable.is_captured());

            capturable.increment_x();
        };

        // Temp closure
        {
            let capturing_closure_with_on_drop_side_effect_callback: TempCallback =
                TempCallback::from_temp_closure(&mut capturing_closure_with_on_drop_side_effect);

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

            // Dropping the temp callback doesn't drop the closure.
            verify_x(3_i32);
        }
    }
}
