pub trait EnumCount: Sized {
    const COUNT: usize;

    type Integer;

    fn try_from_variant_index(variant_index: usize) -> Option<Self>;
}

#[macro_export]
macro_rules! define_enum_count {
    {
        #[repr($integer:ident)]
        $(#[$attr:meta])*
        $pub:vis enum $enum:ident {
            $( $variant:ident ),* $(,)?
        }
    } => {
        #[repr($integer)]
        $( #[$attr] )*
        $pub enum $enum {
            $( $variant, )*
        }

        impl $crate::util::r#enum::count::EnumCount for $enum {
            const COUNT: usize = {
                let mut count: usize = 0_usize;

                $(
                    let _: $enum = $enum::$variant;

                    count += 1_usize;
                )*

                count
            };

            type Integer = $integer;

            fn try_from_variant_index(variant_index: usize) -> Option<Self> {
                use core::mem::transmute;

                (variant_index < Self::COUNT).then(
                    || unsafe { transmute::<Self::Integer, $enum>(variant_index as Self::Integer) })
            }
        }
    };
}

#[cfg(test)]
pub mod tests {
    use {super::*, crate::util::mem::do_types_have_eq_size_and_align};

    define_enum_count! {
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum TestEnumWithCount1 {
            Foo,
            Bar,
            Baz,
        }
    }

    define_enum_count! {
        #[repr(u16)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum TestEnumWithCount2 {
            Foo,
            Bar,
            Baz,
            Qux,
        }
    }

    #[test]
    fn test_enum_with_count_size_and_align() {
        assert!(do_types_have_eq_size_and_align::<TestEnumWithCount1, u8>());
        assert!(do_types_have_eq_size_and_align::<TestEnumWithCount2, u16>());
    }

    #[test]
    fn test_enum_with_count_count() {
        assert_eq!(TestEnumWithCount1::COUNT, 3_usize);
        assert_eq!(TestEnumWithCount2::COUNT, 4_usize);
    }

    #[test]
    fn test_enum_with_count_try_from_variant_index() {
        assert_eq!(
            TestEnumWithCount1::try_from_variant_index(0_usize),
            Some(TestEnumWithCount1::Foo)
        );
        assert_eq!(
            TestEnumWithCount1::try_from_variant_index(1_usize),
            Some(TestEnumWithCount1::Bar)
        );
        assert_eq!(
            TestEnumWithCount1::try_from_variant_index(2_usize),
            Some(TestEnumWithCount1::Baz)
        );
        assert_eq!(TestEnumWithCount1::try_from_variant_index(3_usize), None);
        assert_eq!(
            TestEnumWithCount2::try_from_variant_index(0_usize),
            Some(TestEnumWithCount2::Foo)
        );
        assert_eq!(
            TestEnumWithCount2::try_from_variant_index(1_usize),
            Some(TestEnumWithCount2::Bar)
        );
        assert_eq!(
            TestEnumWithCount2::try_from_variant_index(2_usize),
            Some(TestEnumWithCount2::Baz)
        );
        assert_eq!(
            TestEnumWithCount2::try_from_variant_index(3_usize),
            Some(TestEnumWithCount2::Qux)
        );
        assert_eq!(TestEnumWithCount2::try_from_variant_index(4_usize), None);
    }
}
