use crate::console::{Image, font};

pub struct Framebuffer<
    const BASE_ADDRESS: usize,
    const WIDTH: u32,
    const HEIGHT: u32,
    const STRIDE: usize,
    const FLIP_180: bool,
> {
    cursor_x: u32,
    cursor_y: u32,
}

impl<
    const BASE_ADDRESS: usize,
    const WIDTH: u32,
    const HEIGHT: u32,
    const STRIDE: usize,
    const FLIP_180: bool,
> Framebuffer<BASE_ADDRESS, WIDTH, HEIGHT, STRIDE, FLIP_180>
{
    const ROWS: u32 = HEIGHT / font::GLYPH_HEIGHT;
    const COLS: u32 = WIDTH / font::GLYPH_WIDTH;

    pub unsafe fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn cursor_x(&self) -> u32 {
        self.cursor_x
    }

    pub fn cursor_y(&self) -> u32 {
        self.cursor_y
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    #[inline]
    const fn transform_coords(x: u32, y: u32) -> (u32, u32) {
        if FLIP_180 {
            (WIDTH - 1 - x, HEIGHT - 1 - y)
        } else {
            (x, y)
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let (px, py) = Self::transform_coords(x, y);

        unsafe {
            core::ptr::write_volatile(
                (BASE_ADDRESS + (py as usize) * STRIDE + (px as usize) * 4) as *mut u32,
                color,
            );
        }
    }

    pub fn blit(&mut self, offset_x: u32, offset_y: u32, w: u32, h: u32, data: &[u32]) {
        for y in 0..h {
            for x in 0..w {
                self.set_pixel(x + offset_x, y + offset_y, data[(x + y * w) as usize]);
            }
        }
    }

    pub fn blit_image<const W: u32, const H: u32>(
        &mut self,
        offset_x: u32,
        offset_y: u32,
        img: &Image<{ W }, { H }>,
    ) {
        self.blit(offset_x, offset_y, W, H, img.data)
    }

    fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= WIDTH || y >= HEIGHT {
            return 0;
        }

        let (px, py) = Self::transform_coords(x, y);

        unsafe {
            core::ptr::read_volatile(
                (BASE_ADDRESS + (py as usize) * STRIDE + (px as usize) * 4) as *const u32,
            )
        }
    }

    fn scroll(&mut self) {
        let shift_rows = font::GLYPH_HEIGHT;

        for y in 0..(HEIGHT - shift_rows) {
            for x in 0..WIDTH {
                let color = self.get_pixel(x, y + shift_rows);
                self.set_pixel(x, y, color);
            }
        }

        // blank the last glyph row.
        for y in (HEIGHT - shift_rows)..HEIGHT {
            for x in 0..WIDTH {
                self.set_pixel(x, y, 0x0);
            }
        }
    }

    fn newline(&mut self) {
        let mut y = self.cursor_y() + 1;
        if y > Self::ROWS {
            self.scroll();
            y = Self::ROWS - 1;
        }

        self.set_cursor(0, y);
    }

    fn draw_glyph(&mut self, glyph: &[u8; 8], x: u32, y: u32, color: u32) {
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..font::GLYPH_WIDTH {
                let on = (bits >> col) & 1 != 0;
                let color = if on { color } else { continue };
                self.set_pixel(x + col, y + (row as u32), color);
            }
        }
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.set_cursor(0, self.cursor_y()),
            c => {
                let code = c as usize;
                let glyph = if code < 128 {
                    &font::FONT8X8_BASIC[code]
                } else {
                    &font::FONT8X8_BASIC[b'?' as usize]
                };

                if self.cursor_x() >= Self::COLS {
                    self.newline();
                }

                let px = self.cursor_x() * font::GLYPH_WIDTH;
                let py = self.cursor_y() * font::GLYPH_HEIGHT;
                self.draw_glyph(glyph, px, py, 0xFFFFFF);

                self.set_cursor(self.cursor_x() + 1, self.cursor_y());
            }
        }
    }
}

impl<
    const BASE_ADDRESS: usize,
    const WIDTH: u32,
    const HEIGHT: u32,
    const STRIDE: usize,
    const FLIP_180: bool,
> core::fmt::Write for Framebuffer<BASE_ADDRESS, WIDTH, HEIGHT, STRIDE, FLIP_180>
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
