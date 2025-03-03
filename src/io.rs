use alloc::string::String;
use super::error::Error;
pub mod posix_print;

pub trait Printable
{
    fn print(msg: &str) -> Result<usize, Error>;
    fn printstr(msg: &String) -> Result<usize, Error>;
}
