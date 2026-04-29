// Simple RGB565 framebuffer for the 172x320 JD9853 panel.

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{OriginDimensions, Size};
use embedded_graphics_core::pixelcolor::raw::RawU16;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::prelude::*;
use embedded_graphics_core::primitives::Rectangle;

use crate::board;
use crate::drivers::jd9853::DisplayError;
use crate::drivers::jd9853::Jd9853Display;
use crate::screen::{logical_size, map_logical_to_physical, Orientation};

use alloc::vec;
use alloc::vec::Vec;

const WIDTH: usize = board::LCD_WIDTH as usize;
const HEIGHT: usize = board::LCD_HEIGHT as usize;
const PIXEL_COUNT: usize = WIDTH * HEIGHT;

pub struct Framebuffer {
    buf: Vec<u16>,
    orientation: Orientation,
}

impl Framebuffer {
    pub fn new(orientation: Orientation) -> Self {
        let buf = vec![0u16; PIXEL_COUNT];
        Self { buf, orientation }
    }

    pub fn clear_color(&mut self, color: Rgb565) {
        let raw = RawU16::from(color).into_inner();
        self.buf.fill(raw);
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u16) {
        let (logical_w, logical_h) = self.logical_size();
        let x_end = (x + w).min(logical_w);
        let y_end = (y + h).min(logical_h);
        for row in y..y_end {
            for col in x..x_end {
                self.set_logical_pixel(col as i32, row as i32, color);
            }
        }
    }

    pub fn flush(&self, display: &mut Jd9853Display) {
        display.set_addr_window(0, 0, WIDTH as u16, HEIGHT as u16);
        display.bus_mut().write_pixels(&self.buf);
    }
}

impl OriginDimensions for Framebuffer {
    fn size(&self) -> Size {
        let (w, h) = logical_size(self.orientation);
        Size::new(w as u32, h as u32)
    }
}

impl DrawTarget for Framebuffer {
    type Color = Rgb565;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_logical_pixel(coord.x, coord.y, RawU16::from(color).into_inner());
        }
        Ok(())
    }

    fn fill_contiguous<I>(
        &mut self,
        area: &Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let (logical_w, logical_h) = logical_size(self.orientation);
        let area = area.intersection(&Rectangle::new(Point::zero(), Size::new(logical_w as u32, logical_h as u32)));
        if area.size.width == 0 || area.size.height == 0 {
            return Ok(());
        }

        let x = area.top_left.x as usize;
        let y = area.top_left.y as usize;
        let w = area.size.width as usize;
        let mut row = y;
        let mut col = 0;

        for color in colors.into_iter() {
            if col < w && row < logical_h as usize {
                self.set_logical_pixel((x + col) as i32, row as i32, RawU16::from(color).into_inner());
            }
            col += 1;
            if col >= w {
                col = 0;
                row += 1;
            }
        }
        Ok(())
    }

    fn fill_solid(
        &mut self,
        area: &Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        let (logical_w, logical_h) = logical_size(self.orientation);
        let area = area.intersection(&Rectangle::new(Point::zero(), Size::new(logical_w as u32, logical_h as u32)));
        if area.size.width == 0 || area.size.height == 0 {
            return Ok(());
        }
        let raw = RawU16::from(color).into_inner();
        self.fill_rect(
            area.top_left.x as usize,
            area.top_left.y as usize,
            area.size.width as usize,
            area.size.height as usize,
            raw,
        );
        Ok(())
    }
}

impl Framebuffer {
    fn logical_size(&self) -> (usize, usize) {
        let (w, h) = logical_size(self.orientation);
        (w as usize, h as usize)
    }

    fn set_logical_pixel(&mut self, x: i32, y: i32, color: u16) {
        if let Some((px, py)) = map_logical_to_physical(x, y, self.orientation) {
            self.buf[py * WIDTH + px] = color;
        }
    }
}
