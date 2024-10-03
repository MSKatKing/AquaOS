use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;
use kernel_proc::interrupt;
use crate::drivers::ports::Port;

const PIT_FREQUENCY: u32 = 1193182;
const DESIRED_FREQUENCY: u32 = 1000;

pub fn configure_pit() {
    let divisor = PIT_FREQUENCY / DESIRED_FREQUENCY;

    let command_port = Port::new(0x43);
    let channel_0_data_port = Port::new(0x40);

    command_port.write(0x36u8);

    channel_0_data_port.write((divisor & 0xFF) as u8);
    channel_0_data_port.write((divisor >> 8) as u8);
}

static TICKS: AtomicU64 = AtomicU64::new(0);

#[interrupt]
pub fn pit_interrupt_handler() {
    TICKS.fetch_add(1, Ordering::Relaxed);

    Port::new(0x20).write(0x20u8);
}

pub fn sleep(time: Duration) {
    TICKS.store(0, Ordering::Relaxed);

    while TICKS.load(Ordering::Relaxed) < time.as_millis() as u64 {
        unsafe { asm!("hlt") }
    }
}

fn read_rtc_register(register: u8) -> u8 {
        let port_70 = Port::new(0x70);
        let port_71 = Port::new(0x71);

        port_70.write(0x80 | register);

        port_71.read()
}

fn bcd_to_binary(value: u8) -> u8 {
    (value & 0x0F) + ((value / 16) * 10)
}

fn is_rtc_updating() -> bool {
    read_rtc_register(0x0A) & 0x80 != 0
}

pub fn current_time() -> (u8, u8, u8, u8, u8, u16) {
    while is_rtc_updating() {}

    let mut seconds = read_rtc_register(0x00);
    let mut minutes = read_rtc_register(0x02);
    let mut hours = read_rtc_register(0x04);
    let mut days = read_rtc_register(0x07);
    let mut months = read_rtc_register(0x08);
    let mut years = read_rtc_register(0x09);

    let status_b = read_rtc_register(0x0B);

    if status_b & 0x04 == 0 {
        seconds = bcd_to_binary(seconds);
        minutes = bcd_to_binary(minutes);
        hours = bcd_to_binary(hours);
        days = bcd_to_binary(days);
        months = bcd_to_binary(months);
        years = bcd_to_binary(years);
    }

    (seconds, minutes, hours, days, months, 2000 + (years as u16))
}

#[macro_export]
macro_rules! timeout {
    ($func:expr, $duration:expr) => ({
        use crate::drivers::timing::sleep;
        use core::time::Duration;

        fn test(_a: impl Fn() -> bool, _b: Duration) {}

        test($func, $duration);

        let mut timeout = $duration.as_millis() as u64;

        while timeout > 0 {
            if $func() {
                break;
            }

            sleep(Duration::from_millis(1));
            timeout -= 1;
        }

        if timeout == 0 { panic!("Expression timed out"); }
    });
}