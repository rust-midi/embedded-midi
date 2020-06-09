#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use nb::block;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};

#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    // let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Configure pins for serial rx/tx
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;

    // Configure serial
    let mut usart = Serial::usart2(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(31250.bps()).parity_none(),
        clocks,
        &mut rcc.apb1,
    );

    // Configure Midi
    // <TODO>

    loop {
        //let rec = block!(usart.read());
        match block!(usart.read()) {
            Ok(byte) => hprintln!("byte {:?}", byte),
            Err(ex) => hprintln!("error {:?}", ex),
        };
    }
}
