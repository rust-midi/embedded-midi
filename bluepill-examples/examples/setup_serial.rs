#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::time::U32Ext;
use stm32f1xx_hal::serial::Serial;

#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Configure pins for serial rx/tx
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let tx = gpiob.pb3.into_alternate_open_drain(&mut gpiob.crl);
    let rx = gpiob.pb2.into_alternate_open_drain(&mut gpiob.crl);

    // Configure serial
    let uart = Serial::usart2(dp.USART2, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    // Configure Midi
    // <TODO>
    loop {
       // Wait for note on and print to semihosting output
    }
}
