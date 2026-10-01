use {
    crate::{ensure, q},
    anyhow::{Error, Result},
    arrayvec::{ArrayString, CapacityError},
    core::{
        cmp::Ordering,
        error::Error as CoreError,
        ffi::c_char,
        fmt::{Debug, Display, Formatter, Result as FmtResult},
        result::Result as CoreResult,
        slice::from_raw_parts,
        str::from_utf8,
    },
};

/// The capacity of alias ShortTempString`].
pub const SHORT_TEMP_STRING_CAPACITY: usize = 64_usize;

/// The capacity of alias [`TempString`].
pub const TEMP_STRING_CAPACITY: usize = 256_usize;

/// The capacity of alias [`LongTempString`].
pub const LONG_TEMP_STRING_CAPACITY: usize = 1024_usize;

const NULL_CHAR: char = 0 as char;
const NULL_C_CHAR: c_char = NULL_CHAR as c_char;

pub type ShortTempString = ArrayString<SHORT_TEMP_STRING_CAPACITY>;
pub type TempString = ArrayString<TEMP_STRING_CAPACITY>;
pub type LongTempString = ArrayString<LONG_TEMP_STRING_CAPACITY>;

pub trait ArrayStringTrait
where
    Self: Sized,
{
    /// Clones a string, truncating as necessary to fit.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// type TinyArrayString = ArrayString<4_usize>;
    ///
    /// assert_eq!(
    ///     TinyArrayString::clone_truncating("abcdefgh"),
    ///     TinyArrayString::from("abcd").unwrap()
    /// );
    /// assert_eq!(
    ///     TinyArrayString::clone_truncating("abcd"),
    ///     TinyArrayString::from("abcd").unwrap()
    /// );
    /// assert_eq!(
    ///     TinyArrayString::clone_truncating("ab"),
    ///     TinyArrayString::from("ab").unwrap()
    /// );
    /// ```
    fn clone_truncating(string: &str) -> Self;

    /// Clones a string, truncating as necessary to fit, including a null terminating char.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// type TinyArrayString = ArrayString<4_usize>;
    ///
    /// assert_eq!(
    ///     TinyArrayString::clone_null_terminated_truncating("abcdefgh"),
    ///     TinyArrayString::from("abc\0").unwrap()
    /// );
    /// assert_eq!(
    ///     TinyArrayString::clone_null_terminated_truncating("abcd"),
    ///     TinyArrayString::from("abc\0").unwrap()
    /// );
    /// assert_eq!(
    ///     TinyArrayString::clone_null_terminated_truncating("ab"),
    ///     TinyArrayString::from("ab\0").unwrap()
    /// );
    /// ```
    fn clone_null_terminated_truncating(string: &str) -> Self;

    /// Attempts to clone a C string.
    ///
    /// Returns `Err` if the string isn't valid UTF-8 or can't fit.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// const NULL_C_STR: *const c_char = null();
    /// const INVALID_UTF8_C_STR: *const c_char = b"\xC0\0".as_ptr() as *const c_char;
    /// const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;
    ///
    /// type TinyArrayString = ArrayString<4_usize>;
    /// type TeenyTinyArrayString = ArrayString<2_usize>;
    ///
    /// // Null returns `Err`.
    /// assert!(TinyArrayString::try_clone_c_str(NULL_C_STR).is_err());
    ///
    /// // Invalid UTF-8 returns `Err`.
    /// assert!(TinyArrayString::try_clone_c_str(INVALID_UTF8_C_STR).is_err());
    ///
    /// // Insufficient capacity returns `Err`.
    /// assert!(TeenyTinyArrayString::try_clone_c_str(ABCD_C_STR).is_err());
    ///
    /// assert_eq!(
    ///     TinyArrayString::try_clone_c_str(ABCD_C_STR).ok(),
    ///     Some(TinyArrayString::from("abcd").unwrap())
    /// );
    /// ```
    fn try_clone_c_str(string: *const c_char) -> Result<Self>;

    /// Attempts to clone a C-string, truncating as necessary to fit.
    ///
    /// Returns `Err` if the string isn't valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// const NULL_C_STR: *const c_char = null();
    /// const INVALID_UTF8_C_STR: *const c_char = b"\xC0\0".as_ptr() as *const c_char;
    /// const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;
    ///
    /// type TinyArrayString = ArrayString<4_usize>;
    /// type TeenyTinyArrayString = ArrayString<2_usize>;
    ///
    /// // Null returns `Err`.
    /// assert!(TinyArrayString::try_clone_c_str_truncating(NULL_C_STR).is_err());
    ///
    /// // Invalid UTF-8 returns `Err`.
    /// assert!(TinyArrayString::try_clone_c_str_truncating(INVALID_UTF8_C_STR).is_err());
    ///
    /// // Insufficient capacity truncates.
    /// assert_eq!(
    ///     TeenyTinyArrayString::try_clone_c_str_truncating(ABCD_C_STR).ok(),
    ///     Some(TeenyTinyArrayString::from("ab").unwrap())
    /// );
    ///
    /// assert_eq!(
    ///     TinyArrayString::try_clone_c_str_truncating(ABCD_C_STR).ok(),
    ///     Some(TinyArrayString::from("abcd").unwrap())
    /// );
    /// ```
    fn try_clone_c_str_truncating(string: *const c_char) -> Result<Self>;

    /// Appends a null terminating character, replacing the last char if necessary.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// type TinyArrayString = ArrayString<4_usize>;
    ///
    /// let mut array_string: TinyArrayString = TinyArrayString::from("abc").unwrap();
    ///
    /// // If there's sufficient capacity, null is simply appended.
    /// array_string.null_terminate_truncating();
    /// assert_eq!(array_string.as_str(), "abc\0");
    ///
    /// // A string that's already null-terminated isn't mutated.
    /// array_string.null_terminate_truncating();
    /// assert_eq!(array_string.as_str(), "abc\0");
    ///
    /// array_string = TinyArrayString::from("abcd").unwrap();
    ///
    /// // If there's insufficient capacity, a character is popped, and null is appended.
    /// array_string.null_terminate_truncating();
    /// assert_eq!(array_string.as_str(), "abc\0");
    ///
    /// // The 4-byte cheeseburger emoji.
    /// array_string = TinyArrayString::from("\u{1F354}").unwrap();
    ///
    /// // Not all chars are 1 byte.
    /// array_string.null_terminate_truncating();
    /// assert_eq!(array_string.as_str(), "\0");
    /// ```
    fn null_terminate_truncating(&mut self);

    /// Like [`ArrayString::try_push_str`], but instead of panicking if the string can't fit, that
    /// which can fit is pushed, and that which can't fit is returned as in `Err`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// type TinyArrayString = ArrayString<4_usize>;
    ///
    /// let mut array_string: TinyArrayString = TinyArrayString::new();
    ///
    /// array_string.push_str("ab");
    /// assert_eq!(array_string.try_push_str_truncating("cd"), Ok(()));
    /// assert_eq!(array_string.as_str(), "abcd");
    ///
    /// array_string.clear();
    /// array_string.push_str("ab");
    /// assert_eq!(
    ///     array_string.try_push_str_truncating("cdef"),
    ///     Err(CapacityError::new("ef"))
    /// );
    /// assert_eq!(array_string.as_str(), "abcd");
    /// ```
    fn try_push_str_truncating<'s>(
        &mut self,
        string: &'s str,
    ) -> CoreResult<(), CapacityError<&'s str>>;

    /// Attempts to clone a C string into `self`.
    ///
    /// Returns `Err` if the string is null, isn't valid UTF-8, or can't fit. In all these cases,
    /// `self` is not mutated.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// const NULL_C_STR: *const c_char = null();
    /// const INVALID_UTF8_C_STR: *const c_char = b"\xC0\0".as_ptr() as *const c_char;
    /// const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;
    ///
    /// type TinyArrayString = ArrayString<4_usize>;
    /// type TeenyTinyArrayString = ArrayString<2_usize>;
    ///
    /// let mut array_string: TinyArrayString = TinyArrayString::from("xyz").unwrap();
    ///
    /// // Null returns `Err` and doesn't mutate.
    /// assert!(array_string.try_set_c_str(NULL_C_STR).is_err());
    /// assert_eq!(array_string.as_str(), "xyz");
    ///
    /// // Invalid UTF-8 returns `Err` and doesn't mutate.
    /// assert!(array_string.try_set_c_str(INVALID_UTF8_C_STR).is_err());
    /// assert_eq!(array_string.as_str(), "xyz");
    ///
    /// // Valid in-capacity UTF-8 returns `Ok` and mutates.
    /// assert!(array_string.try_set_c_str(ABCD_C_STR).is_ok());
    /// assert_eq!(array_string.as_str(), "abcd");
    ///
    /// let mut teeny_tiny_array_string: TeenyTinyArrayString =
    ///     TeenyTinyArrayString::from("x").unwrap();
    ///
    /// // Insufficient capacity returns `Err` and doesn't mutate.
    /// assert!(teeny_tiny_array_string.try_set_c_str(ABCD_C_STR).is_err());
    /// assert_eq!(teeny_tiny_array_string.as_str(), "x");
    /// ```
    fn try_set_c_str(&mut self, string: *const c_char) -> Result<()>;

    /// Attempts to clone a C string into `self`.
    ///
    /// Returns `Err` if the string is null or isn't valid UTF-8. In both these cases, `self` is not
    /// mutated.
    ///
    /// Returns `Err` if the string can't fit, in which case `self` contains that which can fit.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// const NULL_C_STR: *const c_char = null();
    /// const INVALID_UTF8_C_STR: *const c_char = b"\xC0\0".as_ptr() as *const c_char;
    /// const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;
    ///
    /// type TinyArrayString = ArrayString<4_usize>;
    /// type TeenyTinyArrayString = ArrayString<2_usize>;
    ///
    /// let mut array_string: TinyArrayString = TinyArrayString::from("xyz").unwrap();
    ///
    /// // Null returns `Err` and doesn't mutate.
    /// assert!(array_string.try_set_c_str_truncating(NULL_C_STR).is_err());
    /// assert_eq!(array_string.as_str(), "xyz");
    ///
    /// // Invalid UTF-8 returns `Err` and doesn't mutate.
    /// assert!(
    ///     array_string
    ///         .try_set_c_str_truncating(INVALID_UTF8_C_STR)
    ///         .is_err()
    /// );
    /// assert_eq!(array_string.as_str(), "xyz");
    ///
    /// // Valid in-capacity UTF-8 returns `Ok` and mutates.
    /// assert!(array_string.try_set_c_str_truncating(ABCD_C_STR).is_ok());
    /// assert_eq!(array_string.as_str(), "abcd");
    ///
    /// let mut teeny_tiny_array_string: TeenyTinyArrayString =
    ///     TeenyTinyArrayString::from("x").unwrap();
    ///
    /// // Insufficient capacity returns `Err` and mutates.
    /// assert!(
    ///     teeny_tiny_array_string
    ///         .try_set_c_str_truncating(ABCD_C_STR)
    ///         .is_err()
    /// );
    /// assert_eq!(teeny_tiny_array_string.as_str(), "ab");
    /// ```
    fn try_set_c_str_truncating(&mut self, string: *const c_char) -> Result<()>;
}

impl<const CAP: usize> ArrayStringTrait for ArrayString<CAP> {
    fn clone_truncating(string: &str) -> Self {
        let mut array_string: Self = Self::new();

        // Swallow this error, since this function is explicitly truncating.
        array_string.try_push_str_truncating(string).ok();

        array_string
    }

    fn clone_null_terminated_truncating(string: &str) -> Self {
        let mut array_string: Self = Self::clone_truncating(string);

        array_string.null_terminate_truncating();

        array_string
    }

    fn try_clone_c_str(string: *const c_char) -> Result<Self> {
        let mut array_string: Self = Self::new();

        array_string.try_set_c_str(string).map(|_| array_string)
    }

    fn try_clone_c_str_truncating(string: *const c_char) -> Result<Self> {
        let string: &str = unsafe { try_str_from_c_str(&string)? };

        let mut array_string: Self = Self::new();

        // Swallow this error, since it'd be better to return a truncated string than to return an
        // error from this function.
        array_string.try_push_str_truncating(string).ok();

        Ok(array_string)
    }

    fn null_terminate_truncating(&mut self) {
        if !self.ends_with(NULL_CHAR) {
            if self.is_full() {
                self.pop();
            }

            self.push(NULL_CHAR);
        }
    }

    fn try_push_str_truncating<'s>(
        &mut self,
        string: &'s str,
    ) -> CoreResult<(), CapacityError<&'s str>> {
        let pushable_len: usize = string.floor_char_boundary(CAP - self.len());

        self.push_str(&string[..pushable_len]);

        match pushable_len.cmp(&string.len()) {
            Ordering::Less => Err(CapacityError::new(&string[pushable_len..])),
            Ordering::Equal => Ok(()),
            Ordering::Greater => unreachable!(),
        }
    }

    fn try_set_c_str(&mut self, string: *const c_char) -> Result<()> {
        let string: &str = unsafe { try_str_from_c_str(&string)? };

        ensure!(string.len() <= CAP);

        self.clear();
        self.push_str(string);

        Ok(())
    }

    fn try_set_c_str_truncating(&mut self, string: *const c_char) -> Result<()> {
        let string: &str = unsafe { try_str_from_c_str(&string)? };

        self.clear();

        q!(self.try_push_str_truncating(string));

        Ok(())
    }
}

pub struct ErrorArrayString<const CAP: usize>(ArrayString<CAP>);

impl<const CAP: usize> CoreError for ErrorArrayString<CAP> {}

impl<const CAP: usize> Debug for ErrorArrayString<CAP> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl<const CAP: usize> Display for ErrorArrayString<CAP> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<const CAP: usize> From<ArrayString<CAP>> for ErrorArrayString<CAP> {
    fn from(value: ArrayString<CAP>) -> Self {
        Self(value)
    }
}

/// C's `strlen` ported to Rust, returning the index of the first null byte, or 0 if `value` is
/// null.
///
/// This is unsafe on account of reading bytes starting at `value`, regardless of whether or not
/// those bytes are safe to read from.
///
/// # Examples
///
/// ```ignore
/// const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;
/// assert_eq!(unsafe { strlen(null()) }, 0_usize);
/// assert_eq!(unsafe { strlen(ABCD_C_STR) }, 4_usize);
/// ```
pub const unsafe fn strlen(value: *const c_char) -> usize {
    unsafe {
        let mut strlen: usize = 0_usize;

        if !value.is_null() {
            let mut cursor: *const c_char = value;

            while cursor.read() != NULL_C_CHAR {
                cursor = cursor.add(1_usize);
                strlen += 1_usize;
            }
        }

        strlen
    }
}

/// Try to convert a C-string to a `str`, using a lifetime bound to (hopefully) the stack frame of
/// `value`.
///
/// This is unsafe on account of reading bytes starting at `value`, regardless of whether or not
/// those bytes are safe to read from.
///
/// Returns `Err` if `value` is null or isn't valid UTF-8.
///
/// # Examples
///
/// ```ignore
/// const NULL_C_STR: *const c_char = null();
/// const INVALID_UTF8_C_STR: *const c_char = b"\xC0\0".as_ptr() as *const c_char;
/// const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;
///
/// // Null returns `Err`.
/// assert!(unsafe { try_str_from_c_str(&NULL_C_STR).is_err() });
///
/// // Invalid UTF-8 returns `Err`.
/// assert!(unsafe { try_str_from_c_str(&INVALID_UTF8_C_STR).is_err() });
///
/// // Valid UTF-8 returns `Ok`.
/// assert_eq!(
///     unsafe { try_str_from_c_str(&ABCD_C_STR) }.ok(),
///     Some("abcd")
/// );
/// ```
pub unsafe fn try_str_from_c_str<'v>(value: &'v *const c_char) -> Result<&'v str> {
    ensure!(!value.is_null());

    let len: usize = unsafe { strlen(*value) };
    let bytes: &[u8] = unsafe { from_raw_parts(*value as *const u8, len) };

    from_utf8(bytes).map_err(Error::new)
}

/// Like [`core::write`], guaranteeing that the string is null-terminated, truncating if necessary.
/// This only works for [`ArrayString`] instances.
///
/// # Examples
///
/// ```ignore
/// let mut string_a: ArrayString<4_usize> = ArrayString::new();
///
/// // String formatting functions as normal.
/// assert!(write0!(&mut string_a, "{}", "ab").is_ok());
/// assert_eq!(string_a.as_str(), "ab\0");
/// string_a.pop();
///
/// // It will truncate as necessary to append the null terminating byte.
/// assert!(write0!(&mut string_a, "cd").is_ok());
/// assert_eq!(string_a.as_str(), "abc\0");
/// string_a.clear();
///
/// // `ArrayString`'s implementation of `Write` still returns `Err` if that which we're attempting
/// // to write can't fit.
/// assert!(write0!(&mut string_a, "abcde").is_err());
/// ```
#[macro_export]
macro_rules! write0 {
    ($string:expr, $($arg:tt)*) => {
        {
            use {::core::fmt::Write, $crate::util::string::ArrayStringTrait};

            let result = write!($string, $($arg)*);

            arrayvec::ArrayString::<_>::null_terminate_truncating($string);

            result
        }
    };
}

#[cfg(test)]
mod tests {
    use {super::*, core::ptr::null};

    const NULL_C_STR: *const c_char = null();
    const INVALID_UTF8_C_STR: *const c_char = b"\xC0\0".as_ptr() as *const c_char;
    const ABCD_C_STR: *const c_char = b"abcd\0".as_ptr() as *const c_char;

    mod array_string_trait {
        use super::*;

        type TinyArrayString = ArrayString<4_usize>;
        type TeenyTinyArrayString = ArrayString<2_usize>;

        #[test]
        fn test_clone_truncating() {
            assert_eq!(
                TinyArrayString::clone_truncating("abcdefgh"),
                TinyArrayString::from("abcd").unwrap()
            );
            assert_eq!(
                TinyArrayString::clone_truncating("abcd"),
                TinyArrayString::from("abcd").unwrap()
            );
            assert_eq!(
                TinyArrayString::clone_truncating("ab"),
                TinyArrayString::from("ab").unwrap()
            );
        }

        #[test]
        fn test_clone_null_terminated_truncating() {
            assert_eq!(
                TinyArrayString::clone_null_terminated_truncating("abcdefgh"),
                TinyArrayString::from("abc\0").unwrap()
            );
            assert_eq!(
                TinyArrayString::clone_null_terminated_truncating("abcd"),
                TinyArrayString::from("abc\0").unwrap()
            );
            assert_eq!(
                TinyArrayString::clone_null_terminated_truncating("ab"),
                TinyArrayString::from("ab\0").unwrap()
            );
        }

        #[test]
        fn test_try_clone_c_str() {
            // Null returns `Err`.
            assert!(TinyArrayString::try_clone_c_str(NULL_C_STR).is_err());

            // Invalid UTF-8 returns `Err`.
            assert!(TinyArrayString::try_clone_c_str(INVALID_UTF8_C_STR).is_err());

            // Insufficient capacity returns `Err`.
            assert!(TeenyTinyArrayString::try_clone_c_str(ABCD_C_STR).is_err());

            assert_eq!(
                TinyArrayString::try_clone_c_str(ABCD_C_STR).ok(),
                Some(TinyArrayString::from("abcd").unwrap())
            );
        }

        #[test]
        fn test_try_clone_c_str_truncating() {
            // Null returns `Err`.
            assert!(TinyArrayString::try_clone_c_str_truncating(NULL_C_STR).is_err());

            // Invalid UTF-8 returns `Err`.
            assert!(TinyArrayString::try_clone_c_str_truncating(INVALID_UTF8_C_STR).is_err());

            // Insufficient capacity truncates.
            assert_eq!(
                TeenyTinyArrayString::try_clone_c_str_truncating(ABCD_C_STR).ok(),
                Some(TeenyTinyArrayString::from("ab").unwrap())
            );

            assert_eq!(
                TinyArrayString::try_clone_c_str_truncating(ABCD_C_STR).ok(),
                Some(TinyArrayString::from("abcd").unwrap())
            );
        }

        #[test]
        fn test_null_terminate_truncating() {
            let mut array_string: TinyArrayString = TinyArrayString::from("abc").unwrap();

            // If there's sufficient capacity, null is simply appended.
            array_string.null_terminate_truncating();
            assert_eq!(array_string.as_str(), "abc\0");

            // A string that's already null-terminated isn't mutated.
            array_string.null_terminate_truncating();
            assert_eq!(array_string.as_str(), "abc\0");

            array_string = TinyArrayString::from("abcd").unwrap();

            // If there's insufficient capacity, a character is popped, and null is appended.
            array_string.null_terminate_truncating();
            assert_eq!(array_string.as_str(), "abc\0");

            // The 4-byte cheeseburger emoji.
            array_string = TinyArrayString::from("\u{1F354}").unwrap();

            // Not all chars are 1 byte.
            array_string.null_terminate_truncating();
            assert_eq!(array_string.as_str(), "\0");
        }

        #[test]
        fn test_try_push_str_truncating() {
            let mut array_string: TinyArrayString = TinyArrayString::new();

            array_string.push_str("ab");
            assert_eq!(array_string.try_push_str_truncating("cd"), Ok(()));
            assert_eq!(array_string.as_str(), "abcd");

            array_string.clear();
            array_string.push_str("ab");
            assert_eq!(
                array_string.try_push_str_truncating("cdef"),
                Err(CapacityError::new("ef"))
            );
            assert_eq!(array_string.as_str(), "abcd");
        }

        #[test]
        fn test_try_set_c_str() {
            let mut array_string: TinyArrayString = TinyArrayString::from("xyz").unwrap();

            // Null returns `Err` and doesn't mutate.
            assert!(array_string.try_set_c_str(NULL_C_STR).is_err());
            assert_eq!(array_string.as_str(), "xyz");

            // Invalid UTF-8 returns `Err` and doesn't mutate.
            assert!(array_string.try_set_c_str(INVALID_UTF8_C_STR).is_err());
            assert_eq!(array_string.as_str(), "xyz");

            // Valid in-capacity UTF-8 returns `Ok` and mutates.
            assert!(array_string.try_set_c_str(ABCD_C_STR).is_ok());
            assert_eq!(array_string.as_str(), "abcd");

            let mut teeny_tiny_array_string: TeenyTinyArrayString =
                TeenyTinyArrayString::from("x").unwrap();

            // Insufficient capacity returns `Err` and doesn't mutate.
            assert!(teeny_tiny_array_string.try_set_c_str(ABCD_C_STR).is_err());
            assert_eq!(teeny_tiny_array_string.as_str(), "x");
        }

        #[test]
        fn test_try_set_c_str_truncating() {
            let mut array_string: TinyArrayString = TinyArrayString::from("xyz").unwrap();

            // Null returns `Err` and doesn't mutate.
            assert!(array_string.try_set_c_str_truncating(NULL_C_STR).is_err());
            assert_eq!(array_string.as_str(), "xyz");

            // Invalid UTF-8 returns `Err` and doesn't mutate.
            assert!(
                array_string
                    .try_set_c_str_truncating(INVALID_UTF8_C_STR)
                    .is_err()
            );
            assert_eq!(array_string.as_str(), "xyz");

            // Valid in-capacity UTF-8 returns `Ok` and mutates.
            assert!(array_string.try_set_c_str_truncating(ABCD_C_STR).is_ok());
            assert_eq!(array_string.as_str(), "abcd");

            let mut teeny_tiny_array_string: TeenyTinyArrayString =
                TeenyTinyArrayString::from("x").unwrap();

            // Insufficient capacity returns `Err` and mutates.
            assert!(
                teeny_tiny_array_string
                    .try_set_c_str_truncating(ABCD_C_STR)
                    .is_err()
            );
            assert_eq!(teeny_tiny_array_string.as_str(), "ab");
        }
    }

    #[test]
    fn test_strlen() {
        assert_eq!(unsafe { strlen(null()) }, 0_usize);
        assert_eq!(unsafe { strlen(ABCD_C_STR) }, 4_usize);
    }

    #[test]
    fn test_try_str_from_c_str() {
        // Null returns `Err`.
        assert!(unsafe { try_str_from_c_str(&NULL_C_STR).is_err() });

        // Invalid UTF-8 returns `Err`.
        assert!(unsafe { try_str_from_c_str(&INVALID_UTF8_C_STR).is_err() });

        // Valid UTF-8 returns `Ok`.
        assert_eq!(
            unsafe { try_str_from_c_str(&ABCD_C_STR) }.ok(),
            Some("abcd")
        );
    }

    #[test]
    fn test_write0() {
        let mut string_a: ArrayString<4_usize> = ArrayString::new();

        // String formatting functions as normal.
        assert!(write0!(&mut string_a, "{}", "ab").is_ok());
        assert_eq!(string_a.as_str(), "ab\0");
        string_a.pop();

        // It will truncate as necessary to append the null terminating byte.
        assert!(write0!(&mut string_a, "cd").is_ok());
        assert_eq!(string_a.as_str(), "abc\0");
        string_a.clear();

        // `ArrayString`'s implementation of `Write` still returns `Err` if that which we're
        // attempting to write can't fit.
        assert!(write0!(&mut string_a, "abcde").is_err());
    }
}
