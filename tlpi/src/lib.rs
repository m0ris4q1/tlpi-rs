mod errors;
mod error_functions;
mod get_num;

pub use error_functions::*;
pub use get_num::*;

#[macro_export]
macro_rules! va_fn {
    ($fn_name:ident) => {
        va_fn!(@inner ($) $fn_name);
    };

    (@inner ($dol:tt) $fn_name:ident) => {
        #[macro_export]
        macro_rules! $fn_name {
            ($dol($dol e:expr),*) => {
                tlpi::$fn_name(format_args!($dol($dol e),*)).unwrap()
            }
        }
    };
}

va_fn!{err_msg}
va_fn!{err_exit3}
va_fn!{err_exit2}
va_fn!{err_exit_en}
va_fn!{fatal}
va_fn!{usage_err}
va_fn!{cmd_line_err}
