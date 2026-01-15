#![no_std]
#![no_main]



// Import necessary crates and modules
use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// systick register - contol and status register
const SYST_CSR: u32 = 0xE000E010;
// systick reload value register
const SYST_RVR: u32 = 0xE000E014;
// systick current value register
const SYST_CVR: u32 = 0xE000E018;

// Run-mode Clock Configuration (RCC) register addresses
const RCC: u32 = 0x400FE060;
// prescaler value register
const SYSCTL_SYSDIV_16: u32 = 0xF;
const SYSCTL_SYSDIV_12: u32 = 0xB;

// CPU frequecy 12.5MHz by default
const CPU_FREQUENCY_HZ: u32 = 12_500_000;


#[entry]
fn main() -> ! {
    hprintln!("Hello, embedded Rust!");

    // Set the prescaler value
    unsafe {
        let sysdiv: u32 = SYSCTL_SYSDIV_12 << 23;
        let orig: u32 = *(RCC as *const u32);
        let mask: u32 = !0b1111 << 23;
        let rcc: u32 = (orig & mask) | sysdiv;
        *(RCC as *mut u32) = rcc;
    }

    let sleep_dur: u32 = CPU_FREQUENCY_HZ; // sleep duration in seconds

    // unsafe block to access hardware registers directly
    // Configure SysTick timer
    unsafe {
        // Set the timer duration
        *(SYST_RVR as *mut u32) = sleep_dur;
        // Clear current value register // *mut because we are writing to it
        *(SYST_CVR as *mut u32) = 0;
        // Enable the timer, enable interrupt, use processor clock 
        *(SYST_CSR as *mut u32) = 0b111;    // 0b means binary literal , set last three bits to 1

    }


    loop {
          asm::wfi(); // put the CPU to sleep
    }
}

#[exception]
fn SysTick() {
    hprintln!("woke up!");
}