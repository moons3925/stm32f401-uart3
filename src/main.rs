#![no_std]
#![no_main]

use embedded_hal::prelude::_embedded_hal_serial_Write;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use stm32f4xx_hal::serial::*;
use stm32f4xx_hal::gpio::{GpioExt};
use stm32f4xx_hal::rcc::RccExt;
use stm32f4xx_hal::time::Bps;

use embedded_hal::serial::Read;
use core::fmt::Write;   // (1)write!()マクロを使えるようにする
use stm32lib::uart::ErrorDetect;    // (2)追加するトレイトを使えるようにする

#[entry]
fn main() -> ! {

    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();   // GPIOAのclockも有効にしてくれる （AHBENRレジスタ）
    let bps = Bps(115_200_u32); // (3)通信速度
    let seri_config = config::Config {  // (4)通信パラメーターの設定
        baudrate: bps,
        wordlength: config::WordLength::DataBits8,  // 実際には7ビット
        parity: config::Parity::ParityEven,
        stopbits: config::StopBits::STOP1,
        dma: config::DmaConfig::None,
    };

    let rcc = dp.RCC.constrain();
    let clks = rcc.cfgr.freeze(); // 初期値でクロックを生成するコード

    let mut serial = Serial::new(
        dp.USART2,
        (gpioa.pa2, gpioa.pa3),
        seri_config,
        &clks,
    ).unwrap(); // (5)Serial構造体の生成

    loop {
        while !serial.is_rx_not_empty() {}
        if serial.is_pe() {
            let _ = serial.read();  // 読み捨てる
            write!(serial, "\r\nParity error {}", "detected.\r\n").unwrap();
        }
        else if serial.is_fe() {
            let _ = serial.read();  // 読み捨てる
            write!(serial, "\r\nFraming error {}", "detected.\r\n").unwrap();
        }
        else if serial.is_ore() {
            let _ = serial.read();  // 読み捨てる
            write!(serial, "\r\nOver run error {}", "detected.\r\n").unwrap();
        }
        else if let Ok(c) = serial.read() {
            while !serial.is_tx_empty() {}
            serial.write(c).unwrap();
        }
    }
}

