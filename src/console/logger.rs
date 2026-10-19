use core::cell::RefCell;

use critical_section::Mutex;
use log::{LevelFilter, Log, SetLoggerError};

use crate::console::Framebuffer;

pub struct Logger<
    const BASE_ADDRESS: usize,
    const WIDTH: u32,
    const HEIGHT: u32,
    const STRIDE: usize,
    const FLIP_180: bool,
> {
    fb: Mutex<RefCell<Framebuffer<BASE_ADDRESS, WIDTH, HEIGHT, STRIDE, FLIP_180>>>,
}

impl<
    const BASE_ADDRESS: usize,
    const WIDTH: u32,
    const HEIGHT: u32,
    const STRIDE: usize,
    const FLIP_180: bool,
> Logger<BASE_ADDRESS, WIDTH, HEIGHT, STRIDE, FLIP_180>
{
    pub fn new(fb: Framebuffer<BASE_ADDRESS, WIDTH, HEIGHT, STRIDE, FLIP_180>) -> Self {
        Self {
            fb: Mutex::new(RefCell::new(fb)),
        }
    }

    pub fn init(&'static mut self, level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(self)?;
        log::set_max_level(level);

        Ok(())
    }
}

impl<
    const BASE_ADDRESS: usize,
    const WIDTH: u32,
    const HEIGHT: u32,
    const STRIDE: usize,
    const FLIP_180: bool,
> Log for Logger<BASE_ADDRESS, WIDTH, HEIGHT, STRIDE, FLIP_180>
{
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            critical_section::with(|cs| {
                use core::fmt::Write;

                _ = writeln!(&mut *self.fb.borrow_ref_mut(cs), "{}", *record.args());
            });
        }
    }

    fn flush(&self) {}
}
