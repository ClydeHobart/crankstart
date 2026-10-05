pub trait EnumStrings {
    const STRINGS: &'static [&'static str];
}

#[macro_export]
macro_rules! define_enum_strings {
    {
        #[repr($integer:ident)]
        $(#[$attr:meta])*
        $pub:vis enum $enum:ident {
            $( $variant:ident ),* $(,)?
        }
    } => {
        $crate::define_enum_count!{
            #[repr($integer)]
            $(#[$attr])*
            $pub enum $enum {
                $( $variant, )*
            }
        }

        impl $crate::util::r#enum::strings::EnumStrings for $enum {
            const STRINGS: &'static [&'static str] = &[
                $( ::core::stringify!($variant), )*
            ];
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::util::{
        r#enum::{count::EnumCount, strings::EnumStrings},
        mem::do_types_have_eq_size_and_align,
    };

    define_enum_strings! {
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum TestEnumWithCountAndStrings1 {
            Foo,
            Bar,
            Baz,
        }
    }

    define_enum_strings! {
        #[repr(u16)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum TestEnumWithCountAndStrings2 {
            Foo,
            Bar,
            Baz,
            Qux,
        }
    }

    #[test]
    fn test_enum_count_size_and_align() {
        assert!(do_types_have_eq_size_and_align::<
            TestEnumWithCountAndStrings1,
            u8,
        >());
        assert!(do_types_have_eq_size_and_align::<
            TestEnumWithCountAndStrings2,
            u16,
        >());
    }

    #[test]
    fn test_enum_count_count() {
        assert_eq!(TestEnumWithCountAndStrings1::COUNT, 3_usize);
        assert_eq!(TestEnumWithCountAndStrings2::COUNT, 4_usize);
    }

    #[test]
    fn test_enum_count_try_from_variant_index() {
        assert_eq!(
            TestEnumWithCountAndStrings1::try_from_variant_index(0_usize),
            Some(TestEnumWithCountAndStrings1::Foo)
        );
        assert_eq!(
            TestEnumWithCountAndStrings1::try_from_variant_index(1_usize),
            Some(TestEnumWithCountAndStrings1::Bar)
        );
        assert_eq!(
            TestEnumWithCountAndStrings1::try_from_variant_index(2_usize),
            Some(TestEnumWithCountAndStrings1::Baz)
        );
        assert_eq!(
            TestEnumWithCountAndStrings1::try_from_variant_index(3_usize),
            None
        );
        assert_eq!(
            TestEnumWithCountAndStrings2::try_from_variant_index(0_usize),
            Some(TestEnumWithCountAndStrings2::Foo)
        );
        assert_eq!(
            TestEnumWithCountAndStrings2::try_from_variant_index(1_usize),
            Some(TestEnumWithCountAndStrings2::Bar)
        );
        assert_eq!(
            TestEnumWithCountAndStrings2::try_from_variant_index(2_usize),
            Some(TestEnumWithCountAndStrings2::Baz)
        );
        assert_eq!(
            TestEnumWithCountAndStrings2::try_from_variant_index(3_usize),
            Some(TestEnumWithCountAndStrings2::Qux)
        );
        assert_eq!(
            TestEnumWithCountAndStrings2::try_from_variant_index(4_usize),
            None
        );
    }

    #[test]
    fn test_enum_strings_strings() {
        assert_eq!(
            TestEnumWithCountAndStrings1::STRINGS,
            &["Foo", "Bar", "Baz"]
        );
        assert_eq!(
            TestEnumWithCountAndStrings2::STRINGS,
            &["Foo", "Bar", "Baz", "Qux"]
        );
    }
}
