use core::ffi;
use alloc::string::{String, ToString};

use crate::posix::write;
use crate::posix::STDOUT_FILENO;

use super::Printable;
use super::Error;

pub struct Print;

impl Printable for Print
{
    fn print(msg: &str) -> Result<usize, Error> {
        let ret = unsafe {
            write(STDOUT_FILENO as i32, msg.as_ptr() as *const ffi::c_void, msg.len())
        };
        if ret < 0
        {
            return Err(Error::IoErr("Posix: Failed to printf".to_string()));
        }
        Ok(ret as usize)
    }
    
    fn printstr(msg: &String) -> Result<usize, Error> {
        let ret = unsafe {
            write(STDOUT_FILENO as i32, msg.as_ptr() as *const ffi::c_void, msg.len())
        };
        if ret < 0
        {
            return Err(Error::IoErr("Posix: Failed to printf".to_string()));
        }
        Ok(ret as usize)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_print(){
        let test= "Hello World\n";
        let test_str = test.to_string();
        let result = Print::printstr(&test_str);
        assert!(result.is_ok());
        let ret = result.unwrap();
        assert_eq!(ret, test_str.len());
        let result = Print::print(&test);
        assert!(result.is_ok());
        let ret = result.unwrap();
        assert_eq!(ret, test_str.len()+1);
    }
}
