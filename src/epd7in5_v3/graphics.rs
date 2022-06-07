use crate::color::TriColor;
use crate::epd7in5_v3::{DEFAULT_BACKGROUND_COLOR, HEIGHT, NUM_DISPLAY_BITS, WIDTH};
use crate::graphics::{DisplayRotation, TriDisplayCompact};
use embedded_graphics_core::prelude::*;

/// Full size buffer for use with the 7in5 EPD
///
/// Can also be manually constructed:
/// `buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value(); 2 * NUM_DISPLAY_BITS as usize]`
pub struct Display7in5 {
    buffer: [u8; 1 * NUM_DISPLAY_BITS as usize],
    rotation: DisplayRotation,
}

impl Default for Display7in5 {
    fn default() -> Self {
        Display7in5 {
            buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value(); 1 * NUM_DISPLAY_BITS as usize],
            rotation: DisplayRotation::default(),
        }
    }
}

impl DrawTarget for Display7in5 {
    type Color = TriColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.draw_helper_tri(
                WIDTH,
                HEIGHT,
                pixel,
                crate::graphics::DisplayColorRendering::Negative,
            )?;
        }
        Ok(())
    }
}

impl OriginDimensions for Display7in5 {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}

impl TriDisplayCompact for Display7in5 {
    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn get_mut_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }

    fn chromatic_offset(&self) -> usize {
        NUM_DISPLAY_BITS as usize
    }

    fn bw_buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn chromatic_buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn clear_bw_buffer(&mut self, background_color: TriColor) {
        for (_, elem) in self.get_mut_buffer().iter_mut().enumerate() {
            *elem = background_color.get_byte_value();
        }
    }

    fn clear_chromatic_buffer(&mut self, background_color: TriColor) {
        for (_, elem) in self.get_mut_buffer().iter_mut().enumerate() {
            // for V3, white in the BW buffer is 255. But in the chromatic buffer 255 is red.
            // This means that the chromatic buffer needs to be inverted when clearing
            *elem = background_color.get_byte_value() ^ 0xFF;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epd7in5_v3;

    // test buffer length
    #[test]
    fn graphics_size() {
        let display = Display7in5::default();
        assert_eq!(display.buffer().len(), 48000);
    }

    // test default background color on all bytes
    #[test]
    fn graphics_default() {
        let display = Display7in5::default();
        for &byte in display.buffer() {
            assert_eq!(byte, epd7in5_v3::DEFAULT_BACKGROUND_COLOR.get_byte_value());
        }
    }
}
