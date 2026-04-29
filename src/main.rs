#![no_std]
#![no_main]

extern crate alloc;

mod app;
mod board;
mod drivers;
mod screen;
mod ui;

use axs5106l::{Axs5106l, Rotation};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::dma_buffers;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::spi::Mode as SpiMode;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;

use crate::drivers::framebuffer::Framebuffer;
use crate::drivers::jd9853::Jd9853Display;
use crate::drivers::spi_bus::SpiDisplayBus;
use crate::app::{AppState, DEVICE_ORIENTATION};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    esp_alloc::heap_allocator!(size: 96 * 1024);

    let peripherals = esp_hal::init(
        esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::_160MHz),
    );

    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    esp_println::logger::init_logger_from_env();
    println!("=== Waveshare ESP32-S3 Touch LCD 1.47 Rust Starter ===");

    let mut delay = Delay::new();

    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_hz(board::I2C_FREQ_HZ)),
    )
    .expect("I2C init failed")
    .with_sda(peripherals.GPIO42)
    .with_scl(peripherals.GPIO41);

    let touch_reset = Output::new(peripherals.GPIO47, Level::High, OutputConfig::default());
    let _touch_irq = Input::new(peripherals.GPIO48, InputConfig::default().with_pull(Pull::Up));

    let mut touch = Axs5106l::new(
        i2c,
        touch_reset,
        board::LCD_WIDTH,
        board::LCD_HEIGHT,
        Rotation::Rotate0,
    );
    let touch_ok = touch.init(&mut delay).is_ok();
    println!("[TOUCH] {}", if touch_ok { "OK" } else { "init failed" });

    let spi_config = SpiConfig::default()
        .with_frequency(Rate::from_mhz(40))
        .with_mode(SpiMode::_0);
    let (rx_buf, rx_desc, tx_buf, tx_desc) = dma_buffers!(4096);
    let dma_rx = DmaRxBuf::new(rx_desc, rx_buf).unwrap();
    let dma_tx = DmaTxBuf::new(tx_desc, tx_buf).unwrap();
    let spi = Spi::new(peripherals.SPI2, spi_config)
        .expect("SPI init failed")
        .with_sck(peripherals.GPIO38)
        .with_mosi(peripherals.GPIO39)
        .with_dma(peripherals.DMA_CH0)
        .with_buffers(dma_rx, dma_tx);

    let dc = Output::new(peripherals.GPIO45, Level::Low, OutputConfig::default());
    let cs = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let reset = Output::new(peripherals.GPIO40, Level::High, OutputConfig::default());
    let mut backlight = Output::new(peripherals.GPIO46, Level::Low, OutputConfig::default());
    backlight.set_high();

    let mut display = Jd9853Display::new(SpiDisplayBus::new(spi, dc, cs), reset);
    display.init();
    println!("[LCD] OK");

    let mut framebuffer = Framebuffer::new(DEVICE_ORIENTATION);
    let boot_button = Input::new(peripherals.GPIO0, InputConfig::default().with_pull(Pull::Up));

    let mut state = AppState::new(DEVICE_ORIENTATION);

    ui::render(&mut framebuffer, &state);
    framebuffer.flush(&mut display);

    loop {
        let touch_now = if touch_ok {
            touch.get_touch_data().ok().flatten().and_then(|data| data.first_touch())
        } else {
            None
        };
        let boot_pressed = boot_button.is_low();
        let changed = state.update(touch_now, boot_pressed);

        if changed {
            ui::render(&mut framebuffer, &state);
            framebuffer.flush(&mut display);
        }

        Timer::after(Duration::from_millis(25)).await;
    }
}
