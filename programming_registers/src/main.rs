#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::peripheral::Peripherals;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// CPU frequecy 12.5MHz by default
const CPU_FREQUENCY_HZ: u32 = 12_500_000;


#[entry]
fn main() -> ! {
    hprintln!("Hello, embedded Rust!");
    
    let peripherals: Peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.enable_interrupt();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(CPU_FREQUENCY_HZ); // wait for 1 second
    systick.clear_current(); // clear current value in system timer (systick)
    systick.enable_counter(); // start countdown
 
    loop {
          asm::wfi(); // put the CPU to sleep
    }
}

#[exception]
fn SysTick() {
    hprintln!("woke up!");
}