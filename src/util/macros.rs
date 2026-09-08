#[macro_export]
macro_rules! str_lit {
    ($str_lit:literal) => {
        #[cfg(debug_assertions)]
        {
            $str_lit
        }

        #[cfg(not(debug_assertions))]
        {
            ""
        }
    };
}

#[macro_export]
macro_rules! ensure {
    ($expr:expr) => {
        if $expr {
            Ok(())
        } else {
            Err(::anyhow::Error::msg(
                #[cfg(debug_assertions)]
                ::core::concat!(
                    ::core::file!(),
                    ":",
                    ::core::line!(),
                    ": \"",
                    ::core::stringify!($expr),
                    "\" was false"
                ),
                #[cfg(not(debug_assertions))]
                "",
            ))
        }?
    };
}

#[macro_export]
macro_rules! q {
    ($expr:expr) => {
        ($expr).map_err(|_| {
            ::anyhow::Error::msg(
                #[cfg(debug_assertions)]
                ::core::concat!(
                    ::core::file!(),
                    ":",
                    ::core::line!(),
                    ": \"",
                    ::core::stringify!($expr),
                    "\" was an error"
                ),
                #[cfg(not(debug_assertions))]
                "",
            )
        })?
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::sys::SysAPI::print_internal(
            |temp_string| {
                $crate::write0!(temp_string, $($arg)*).ok();
            },
            $crate::sys::SysAPI::log_to_console
        );
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::sys::SysAPI::print_internal(
            |temp_string| {
                $crate::write0!(temp_string, $($arg)*).ok();
            },
            $crate::sys::SysAPI::error
        );
    }
}

#[macro_export]
macro_rules! breakpoint_nop {
    () => {
        #[cfg(debug_assertions)]
        {
            ::core::hint::black_box(());
            ::core::intrinsics::breakpoint();
        }
    };
}
