use core::mem::{align_of, size_of};

pub const fn do_types_have_eq_size<T, U>() -> bool {
    size_of::<T>() == size_of::<U>()
}

pub const fn do_types_have_eq_align<T, U>() -> bool {
    align_of::<T>() == align_of::<U>()
}

pub const fn do_types_have_eq_size_and_align<T, U>() -> bool {
    do_types_have_eq_size::<T, U>() && do_types_have_eq_align::<T, U>()
}
