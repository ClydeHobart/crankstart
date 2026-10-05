use {crate::util::r#enum::count::EnumCount, core::marker::PhantomData};

pub const fn get_for_enum<E: EnumCount>() -> usize {
    let mut bit_arr_storage: BitArrStorage = BitArrStorage::U8Variant;
    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    {
        let count: usize = E::COUNT;

        if count > u8::BITS as usize {
            bit_arr_storage = BitArrStorage::U16Variant;

            #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
            if count > u16::BITS as usize {
                bit_arr_storage = BitArrStorage::U32Variant;

                #[cfg(target_pointer_width = "64")]
                if count > u32::BITS as usize {
                    bit_arr_storage = BitArrStorage::U64Variant;
                }
            }
        }
    }

    bit_arr_storage as usize
}

pub struct BitArrStorageImplementor<const BIT_ARR_STORAGE: usize>(
    PhantomData<[(); BIT_ARR_STORAGE]>,
);

pub trait BitArrStorageTrait {
    type Storage;
}

#[macro_export]
macro_rules! enum_flags_bit_arr_storage {
    ($enum:ident) => {
        <$crate::util::r#enum::flags::bit_arr_storage::BitArrStorageImplementor<
            { $crate::util::r#enum::flags::bit_arr_storage::get_for_enum::<$enum>() },
        > as $crate::util::r#enum::flags::bit_arr_storage::BitArrStorageTrait>::Storage
    };
}

enum BitArrStorage {
    U8Variant,

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    U16Variant,

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    U32Variant,

    #[cfg(target_pointer_width = "64")]
    U64Variant,
}

impl BitArrStorage {
    const U8: usize = Self::U8Variant as usize;

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    const U16: usize = Self::U16Variant as usize;

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    const U32: usize = Self::U32Variant as usize;

    #[cfg(target_pointer_width = "64")]
    const U64: usize = Self::U64Variant as usize;
}

impl BitArrStorageTrait for BitArrStorageImplementor<{ BitArrStorage::U8 }> {
    type Storage = u8;
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
))]
impl BitArrStorageTrait for BitArrStorageImplementor<{ BitArrStorage::U16 }> {
    type Storage = u16;
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl BitArrStorageTrait for BitArrStorageImplementor<{ BitArrStorage::U32 }> {
    type Storage = u32;
}

#[cfg(target_pointer_width = "64")]
impl BitArrStorageTrait for BitArrStorageImplementor<{ BitArrStorage::U64 }> {
    type Storage = u64;
}
