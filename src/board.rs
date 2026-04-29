#![allow(dead_code)]

// Board pin definitions for Waveshare ESP32-S3-Touch-LCD-1.47
// Reference:
// - Waveshare wiki: https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.47
// - Waveshare schematic PDF: ESP32-S3-Touch-LCD-1.47-Schematic.pdf

// === LCD (JD9853 over 4-wire SPI) ===
pub const LCD_SCLK: u8 = 38;
pub const LCD_MOSI: u8 = 39;
pub const LCD_CS: u8 = 21;
pub const LCD_DC: u8 = 45;
pub const LCD_RESET: u8 = 40;
pub const LCD_BL: u8 = 46;
pub const LCD_WIDTH: u16 = 172;
pub const LCD_HEIGHT: u16 = 320;
pub const LCD_COL_OFFSET: u16 = 34;
pub const LCD_ROW_OFFSET: u16 = 0;

// === Touch (AXS5106L) ===
pub const I2C_SDA: u8 = 42;
pub const I2C_SCL: u8 = 41;
pub const I2C_FREQ_HZ: u32 = 400_000;
pub const TP_RESET: u8 = 47;
pub const TP_INT: u8 = 48;

// === Buttons ===
pub const BOOT_BUTTON: u8 = 0;
