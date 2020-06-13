#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_midi::{MidiEvent, MidiOut};
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
    let (mut tx, mut rx) = usart.split();
    let mut midi_out = MidiOut::new(tx);

    loop {
        let event = MidiEvent::note_on(0u8.into(), 50u8.into(), 0x40u8.into());
        hprintln!("on {:?}", event);
        midi_out.write(event);

        // block!(usart.write(0x90u8));
        // block!(usart.write(65));
        // block!(usart.write(64));

        let event = MidiEvent::note_off(0u8.into(), 50u8.into(), 0x40u8.into());
        hprintln!("off {:?}", event);
        midi_out.write(event);

        // block!(usart.write(0x80u8));
        // block!(usart.write(65));
        // block!(usart.write(64));
    }
}
