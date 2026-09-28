use crate::util::enum_with_count::EnumWithCount;

pub trait EnumFlags {
    type Enum: EnumWithCount;
    type InnerInner;
    type Inner;
}

#[macro_export]
macro_rules! define_enum_flags {
    {
        $(#[$attr:meta])*
        $pub:vis struct $flags:ident = $enum:ident in $inner_inner:ident;
    } => {
        $( #[$attr] )*
        $pub struct $flags(<$flags as $crate::util::enum_flags::EnumFlags>::Inner);

        impl $crate::util::enum_flags::EnumFlags for $flags {
            type Enum = $enum;
            type InnerInner = $inner_inner;
            type Inner = bitvec::BitArr!(
                for <$enum as $crate::util::enum_with_count::EnumWithCount>::COUNT,
                in $inner_inner
            );
        }

        #[allow(dead_code)]
        impl $flags {
            $pub fn new() -> Self {
                Self(<Self as $crate::util::enum_flags::EnumFlags>::Inner::new(Default::default()))
            }

            $pub fn all() -> Self {
                let mut flags: Self = Self::new();

                type Enum = <$flags as $crate::util::enum_flags::EnumFlags>::Enum;

                const COUNT: usize = <Enum as $crate::util::enum_with_count::EnumWithCount>::COUNT;

                flags.0[..COUNT].fill(true);

                flags
            }

            $pub fn from_iterator<I: IntoIterator<Item = $enum>>(iter: I) -> Self {
                iter
                    .into_iter()
                    .fold(Self::new(), |mut flags, variant| { flags.set(variant, true); flags })
            }

            $pub fn get(&self, variant: $enum) -> bool {
                self.0[variant as usize]
            }

            $pub fn set(&mut self, variant: $enum, value: bool) {
                self.0.set(variant as usize, value);
            }

            $pub fn iterate(&self) -> impl Iterator<Item = $enum> {
                struct FlagsIterator {
                    flags: $flags,
                    variant_index: usize,
                }

                impl FlagsIterator {
                    fn find_next_variant_index(&self) -> usize {
                        use $crate::util::enum_with_count::EnumWithCount;

                        let mut variant_index: usize = self.variant_index;

                        while $enum::try_from_variant_index(variant_index)
                            .map_or(false, |variant| !self.flags.get(variant))
                        {
                            variant_index += 1_usize;
                        }

                        variant_index
                    }

                    fn update_variant_index(&mut self) {
                        self.variant_index = self.find_next_variant_index();
                    }
                }

                impl Iterator for FlagsIterator {
                    type Item = $enum;

                    fn next(&mut self) -> Option<Self::Item> {
                        use $crate::util::enum_with_count::EnumWithCount;

                        $enum::try_from_variant_index(self.variant_index).map(
                            |next_variant| {
                                assert!(self.flags.get(next_variant));

                                self.variant_index += 1_usize;
                                self.update_variant_index();

                                next_variant
                            })
                    }
                }

                let mut flags_iterator: FlagsIterator = FlagsIterator {
                    flags: $flags(self.0.clone()),
                    variant_index: 0_usize,
                };

                flags_iterator.update_variant_index();

                flags_iterator
            }

            fn into_inner(self) -> $inner_inner {
                unsafe { ::core::mem::transmute::<$flags, $inner_inner>(self) }
            }
        }

        impl core::ops::BitAnd for $flags {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = self;

                lhs &= rhs;

                lhs
            }
        }

        impl core::ops::BitAndAssign for $flags {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl core::ops::BitOr for $flags {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = self;

                lhs |= rhs;

                lhs
            }
        }

        impl core::ops::BitOrAssign for $flags {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl core::ops::BitXor for $flags {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut lhs: Self = self;

                lhs ^= rhs;

                lhs
            }
        }

        impl core::ops::BitXorAssign for $flags {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use {crate::define_enum_with_count, TestEnumWithCount::*, std::mem::transmute};

    define_enum_with_count! {
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum TestEnumWithCount {
            Foo,
            Bar,
            Baz,
        }
    }

    define_enum_flags! {
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct TestEnumWithCountFlags = TestEnumWithCount in u8;
    }

    impl TestEnumWithCountFlags {
        fn verify_inner(self, value: u8) {
            assert_eq!(unsafe { transmute::<Self, u8>(self) }, value);
        }
    }

    #[test]
    fn test_flags_new() {
        TestEnumWithCountFlags::new().verify_inner(0b00000000_u8);
    }

    #[test]
    fn test_flags_set() {
        let mut flags: TestEnumWithCountFlags = TestEnumWithCountFlags::new();

        flags.verify_inner(0b00000000_u8);
        flags.set(Foo, true);
        flags.verify_inner(0b00000001_u8);
        flags.set(Bar, true);
        flags.verify_inner(0b00000011_u8);
        flags.set(Baz, true);
        flags.verify_inner(0b00000111_u8);
        flags.set(Foo, false);
        flags.verify_inner(0b00000110_u8);
        flags.set(Bar, false);
        flags.verify_inner(0b00000100_u8);
        flags.set(Baz, false);
        flags.verify_inner(0b00000000_u8);
    }

    #[test]
    fn test_flags_get() {
        let mut flags: TestEnumWithCountFlags = TestEnumWithCountFlags::new();

        assert!(!flags.get(Foo));
        assert!(!flags.get(Bar));
        assert!(!flags.get(Baz));
        flags.set(Foo, true);
        assert!(flags.get(Foo));
        assert!(!flags.get(Bar));
        assert!(!flags.get(Baz));
        flags.set(Bar, true);
        assert!(flags.get(Foo));
        assert!(flags.get(Bar));
        assert!(!flags.get(Baz));
        flags.set(Baz, true);
        assert!(flags.get(Foo));
        assert!(flags.get(Bar));
        assert!(flags.get(Baz));
        flags.set(Foo, false);
        assert!(!flags.get(Foo));
        assert!(flags.get(Bar));
        assert!(flags.get(Baz));
        flags.set(Bar, false);
        assert!(!flags.get(Foo));
        assert!(!flags.get(Bar));
        assert!(flags.get(Baz));
        flags.set(Baz, false);
        assert!(!flags.get(Foo));
        assert!(!flags.get(Bar));
        assert!(!flags.get(Baz));
    }

    #[test]
    fn test_flags_iterate() {
        let mut flags: TestEnumWithCountFlags = TestEnumWithCountFlags::new();

        assert_eq!(flags.iterate().collect::<Vec<TestEnumWithCount>>(), vec![]);
        flags.set(Foo, true);
        assert_eq!(
            flags.iterate().collect::<Vec<TestEnumWithCount>>(),
            vec![Foo]
        );
        flags.set(Bar, true);
        assert_eq!(
            flags.iterate().collect::<Vec<TestEnumWithCount>>(),
            vec![Foo, Bar]
        );
        flags.set(Baz, true);
        assert_eq!(
            flags.iterate().collect::<Vec<TestEnumWithCount>>(),
            vec![Foo, Bar, Baz]
        );
        flags.set(Foo, false);
        assert_eq!(
            flags.iterate().collect::<Vec<TestEnumWithCount>>(),
            vec![Bar, Baz]
        );
        flags.set(Bar, false);
        assert_eq!(
            flags.iterate().collect::<Vec<TestEnumWithCount>>(),
            vec![Baz]
        );
        flags.set(Baz, false);
        assert_eq!(flags.iterate().collect::<Vec<TestEnumWithCount>>(), vec![]);
    }

    #[test]
    fn test_from_iterator() {
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![]
        );
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([Foo])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![Foo]
        );
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([Foo, Bar])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![Foo, Bar]
        );
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([Foo, Bar, Baz])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![Foo, Bar, Baz]
        );
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([Bar, Baz])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![Bar, Baz]
        );
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([Baz])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![Baz]
        );
        assert_eq!(
            TestEnumWithCountFlags::from_iterator([Baz, Bar, Foo])
                .iterate()
                .collect::<Vec<TestEnumWithCount>>(),
            vec![Foo, Bar, Baz]
        );
    }
}
