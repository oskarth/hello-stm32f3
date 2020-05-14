#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::dbg;
use stm32f3::stm32f303;
use stm32f303::interrupt;

// B1 PA0 for button
// PE8 for blue led

// Partly based on https://github.com/antoinevg/stm32f3-rust-examples

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f3::stm32f303::Peripherals::take().unwrap();

    // Enable GPIOA and SYSCFG clocks
    let rcc = &dp.RCC;
    // RCC AHB bus, enable GPIO A and E
    rcc.ahbenr.modify(|_, w| {
        w.iopaen().set_bit()
            .iopeen().set_bit()
    });
    rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    let gpioa = &dp.GPIOA;
    unsafe { gpioa.moder.modify(|_, w| w.moder0().bits(0x00)); // input
             gpioa.pupdr.modify(|_, w| w.pupdr0().bits(0x10))
    };

    let gpioe = &dp.GPIOE;
    gpioe.moder.modify(|_, w| w.moder8().output()); // bits(0x01)
    gpioe.odr.modify(|_, w| w.odr8().set_bit());

    // Connect EXTI0 line to PA0 pin
    let syscfg = &dp.SYSCFG_COMP_OPAMP;
    syscfg.syscfg_exticr1.modify(|_, w| unsafe { w.exti0().bits(0b000) }); // w.exti0().pa0()

    // Configure EXTI0 line (external interrupts) mode=interrupt and trigger=rising-edge
    let exti = &dp.EXTI;
    exti.imr1.modify(|_, w| w.mr0().set_bit());   // unmask interrupt
    exti.rtsr1.modify(|_, w| w.tr0().set_bit());  // trigger=rising-edge


    // Enable EXTI0 Interrupt
    let mut nvic = cp.NVIC;
    unsafe {
        cortex_m::peripheral::NVIC::unmask(stm32f3::stm32f303::Interrupt::EXTI0);

        //nvic.enable(stm32f3::stm32f303::Interrupt::EXTI0);
    }
    // dbg!("Hello world");

    loop {
        // dbg!(gpioa.idr.read().idr0().bit_is_set());
    }

}

#[interrupt]
fn EXTI0() {

    // +0x14 PR1 address offset
    unsafe {
      let exti_pr1_addr = 0x4001_0414 as *mut u32;
      let p = core::ptr::read_volatile(exti_pr1_addr);
      core::ptr::write_volatile(exti_pr1_addr, p | 1);
    }

    dbg!("Hello world");
}
