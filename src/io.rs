use crate::sbi::sbi_call;
use core::fmt::Write;

pub(crate) struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            putchar(ch);
        }

        Ok(())
    }
}

impl Stdout {
    pub fn new() -> Self {
        Self {}
    }
}

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
        use crate::io::Stdout;
        use core::fmt::Write;
		let _ = write!(Stdout::new(), $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

pub fn putchar(ch: char) {
    sbi_call(ch as i64, 0, 0, 0, 0, 0, 0, 1);
}
