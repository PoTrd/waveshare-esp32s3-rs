# waveshare-watch-rs

Rust starter firmware for the **Waveshare ESP32-S3-Touch-LCD-1.47**.

This repo is no longer the old AMOLED smartwatch port. It now boots a small Rust `no_std` app for your actual board:

- initializes the `JD9853` LCD over 4-wire SPI
- enables the LCD backlight on `IO46`
- initializes the `AXS5106L` touch controller over I2C
- renders a simple framebuffer UI with `embedded-graphics`
- changes theme when you tap the left / center / right side of the screen

The goal is to give you a clean base for your own project, not to clone the Waveshare factory demo.

## Verified Hardware Mapping

Pin mapping was taken from the Waveshare wiki + schematic:

- `LCD_CS = IO21`
- `LCD_SCL = IO38`
- `LCD_SDA = IO39`
- `LCD_RST = IO40`
- `LCD_DC = IO45`
- `LCD_BL = IO46`
- `TP_SCL = IO41`
- `TP_SDA = IO42`
- `TP_RST = IO47`
- `TP_INT = IO48`

Sources:

- Waveshare wiki: https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.47
- Waveshare schematic PDF: https://files.waveshare.com/wiki/ESP32-S3-Touch-LCD-1.47/ESP32-S3-Touch-LCD-1.47-Schematic.pdf

## Project Layout

```text
src/
├── main.rs               Hardware init + main loop
├── app/
│   └── mod.rs            State transitions and input handling
├── board.rs              Board dimensions and verified pin mapping
├── ui/
│   └── mod.rs            Rendering and theme definitions
└── drivers/
    ├── spi_bus.rs        SPI write helpers for command/data/pixel transfers
    ├── jd9853.rs         LCD init sequence + address window handling
    └── framebuffer.rs    172x320 RGB565 framebuffer
```

## Build

Prerequisites:

- `espup install`
- `cargo install espflash`

Then:

```bash
cargo +esp check
```

## Flash

The runner is already configured in `.cargo/config.toml`, so flashing is just:

```bash
cargo +esp run
```

That uses:

```bash
espflash flash --monitor
```

If the board does not enter download mode automatically, hold `BOOT`, plug USB in, then retry.

## What You Should See

On boot:

- the screen turns on
- a card-based starter UI is drawn
- touching the panel shows the touch marker
- tapping left / center / right changes the color theme
- pressing the `BOOT` button resets the theme

## Notes

- The display driver uses the Waveshare `JD9853` init sequence exposed by their Arduino example path.
- The touch path uses the `axs5106l` crate and `Rotation::Rotate0`.
- `cargo +esp check` passes in this repo after the migration.

## Next Good Steps

- add your own screen abstraction or app state machine
- move from polling to interrupt-driven touch with `TP_INT`
- add SD card support using the verified `IO13..IO18` mapping
- add Wi‑Fi only after the UI base is stable
