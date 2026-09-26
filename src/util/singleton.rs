use core::cell::{Ref, RefCell, RefMut};

pub trait Singleton: Sized + 'static {
    fn try_get() -> Option<Ref<'static, Self>> {
        get_storage::<Self>().map(RefCell::borrow)
    }

    fn get() -> Ref<'static, Self> {
        Self::try_get().unwrap()
    }

    fn try_get_mut() -> Option<RefMut<'static, Self>> {
        get_storage::<Self>().map(RefCell::borrow_mut)
    }

    fn get_mut() -> RefMut<'static, Self> {
        Self::try_get_mut().unwrap()
    }

    fn set(value: Self) {
        // SAFETY: We don't have many synchronization options available to us due to hardware
        // restrictions, but we'll do our best to ensure nobody is actively holding onto a
        // reference.
        let storage_mut: &mut Option<RefCell<Self>> = unsafe { Self::get_storage_mut_unchecked() };

        assert!(
            storage_mut
                .as_ref()
                .map_or(true, |ref_cell| ref_cell.try_borrow_mut().is_ok())
        );

        *storage_mut = Some(RefCell::new(value));
    }

    unsafe fn get_storage_mut_unchecked() -> &'static mut Option<RefCell<Self>>;
}

fn get_storage<S: Singleton>() -> Option<&'static RefCell<S>> {
    unsafe { S::get_storage_mut_unchecked() }.as_ref()
}

#[macro_export]
macro_rules! impl_singleton {
    ($ty:ty) => {
        impl $crate::util::singleton::Singleton for $ty {
            #[allow(static_mut_refs)]
            unsafe fn get_storage_mut_unchecked() -> &'static mut Option<
                ::core::cell::RefCell<Self>
            > {
                unsafe {
                    static mut STORAGE: Option<::core::cell::RefCell<$ty>> = None;

                    &mut STORAGE
                }
            }
        }
    };
}
