use alloc::boxed::Box;

use crate::error::Error;

#[cfg(feature = "posix")]
pub mod posix_thread;

#[cfg(feature = "posix")]
pub mod posix_process;

pub mod mutex;

#[allow(dead_code)]
type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

/// Waits for the process to finish and returns its output
/// 
/// # Returns
/// 
/// * `Ok(Box<Option<T>>)` - A boxed option containing the process output if successful
/// * `Err(Error)` - If the process failed to join or encountered an error
/// 
/// # Errors
/// 
/// This function will return an error if the process fails to join or encounters
/// any runtime errors during execution
#[allow(dead_code)]
trait Join<T>
{
    /// Joins the thread with the current thread
    fn join(&mut self) -> Result<Box<Option<T>>, Error>;
}
