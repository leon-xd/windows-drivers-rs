use core::fmt;
use core::{ffi::{CStr, FromBytesUntilNulError}, str::Utf8Error};

const DEFAULT_WDK_FORMAT_BUFFER_SIZE: usize = 512;

/// A fixed-size formatting buffer that implements [`fmt::Write`].
///
/// Designed for constrained Windows driver environments, this buffer is
/// zero-initialized and stores up to `T` bytes (default 512).
///
/// Use `write!`/`format_args!` to append into the buffer, then consume via
/// [`WdkFormatBuffer::to_str`] or [`WdkFormatBuffer::to_cstr`].
///
/// # Examples
/// ```
/// use core::fmt::Write;
/// use wdk::fmt::WdkFormatBuffer;
///
/// let mut buf = WdkFormatBuffer::<16>::new();
/// write!(&mut buf, "hello {}", 42).unwrap();
///
/// let s = buf.to_str().unwrap();
/// assert!(s.starts_with("hello 42"));
///
/// let c = buf.to_cstr().unwrap();
/// assert_eq!(c.to_bytes(), b"hello 42");
/// ```
pub struct WdkFormatBuffer<const T: usize = DEFAULT_WDK_FORMAT_BUFFER_SIZE> {
    buffer: [u8; T],
    used: usize,
}

impl<const T: usize> WdkFormatBuffer<T> {
    /// Creates a zeroed formatting buffer with capacity `T`.
    ///
    /// The buffer starts empty (`used == 0`) and is ready for `fmt::Write`.
    pub fn new() -> Self {
        Self {
            buffer: [0; T],
            used: 0,
        }
    }

    pub fn used(&self) -> usize {
        self.used
    }

    /// Returns a UTF-8 view over the underlying buffer.
    ///
    /// This interprets the full buffer (including unused zeroed bytes) as UTF-8.
    /// Trailing `NUL` bytes are valid UTF-8 and remain in the returned `&str`.
    ///
    /// Errors if any byte in the buffer is not valid UTF-8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(&self.buffer)
    }

    /// Returns a C string view up to the first `NUL` byte.
    ///
    /// Fails if no `NUL` terminator is present in the buffer.
    pub fn to_cstr(&self) -> Result<&CStr, FromBytesUntilNulError> {
        CStr::from_bytes_until_nul(&self.buffer)
    }
}

impl<const T: usize> fmt::Write for WdkFormatBuffer<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() + self.used >= T {
            self.buffer[self.used..T - 1].copy_from_slice(&s.as_bytes()[..T - self.used - 1]);
            self.used = T - 1;
            return Err(fmt::Error);
        }
        self.buffer[self.used..s.len()].copy_from_slice(s.as_bytes());
        self.used += s.len();
        Ok(())
    }
}

