use alloc::string::String;
use super::error::Error;

#[cfg(feature = "posix")]
pub mod posix_print;

pub trait Print
{
    fn print(msg: &str) -> Result<usize, Error>;
    fn printstr(msg: &String) -> Result<usize, Error>;
}
