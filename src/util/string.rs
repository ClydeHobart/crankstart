use {crate::breakpoint_nop, arrayvec::ArrayString};

const NULL_TERMINATING_BYTE: char = 0 as char;
const TEMP_STRING_SIZE: usize = 256_usize;
const LONG_TEMP_STRING_SIZE: usize = 1024_usize;

pub type TempString = ArrayString<TEMP_STRING_SIZE>;
pub type LongTempString = ArrayString<LONG_TEMP_STRING_SIZE>;

pub trait ArrayStringTrait
where
    Self: Sized,
{
    fn clone_null_terminated(string: &str) -> Self;

    fn null_terminate(&mut self);
}

impl<const CAP: usize> ArrayStringTrait for ArrayString<CAP> {
    fn clone_null_terminated(string: &str) -> Self {
        let mut array_string: Self = Self::new();

        array_string.push_str(&string[..string.floor_char_boundary(CAP)]);
        array_string.null_terminate();

        array_string
    }

    fn null_terminate(&mut self) {
        if self.try_push(NULL_TERMINATING_BYTE).is_ok() {
            breakpoint_nop!();
        } else {
            self.pop();
            self.push(NULL_TERMINATING_BYTE);
        }
    }
}

#[macro_export]
macro_rules! write0 {
    ($string:expr, $($arg:tt)*) => {
        {
            use {::core::fmt::Write, $crate::util::string::ArrayStringTrait};

            let result = write!($string, $($arg)*);

            arrayvec::ArrayString::<_>::null_terminate($string);

            result
        }
    };
}
