#![allow(dead_code, unused)]
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::{Spawner, task};
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::otg_fs::{Usb, asynch};
use esp_hal::peripherals::{GPIO19, GPIO20, USB0};
use esp_hal::time::Instant;
use esp_hal::timer::timg::TimerGroup;

use defmtusb::run;
use static_cell::ConstStaticCell;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static USB_RX_BUF: ConstStaticCell<[u8; 1024]> = ConstStaticCell::new([0u8; 1024]);

#[task]
async fn start_logger(usb0: USB0<'static>, dp: GPIO20<'static>, dm: GPIO19<'static>) {
    let usb = Usb::new(usb0, dp, dm);
    let tmp_buffer = USB_RX_BUF.take();
    let embassy_conf = {
        let mut c = embassy_usb::Config::new(0x303A, 0x3001);
        c.serial_number = Some("highjeans");
        c.max_packet_size_0 = 64;
        c.composite_with_iads = true;
        c.device_class = 0xEF;
        c.device_sub_class = 0x02;
        c.device_protocol = 0x01;
        c
    };
    let driver = asynch::Driver::new(usb, tmp_buffer, asynch::Config::default());
    defmtusb::run(driver, 64, embassy_conf).await;
}

#[task]
async fn test_other_task() {
    loop {
        info!("Hello world from the other task!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);
    info!("Embassy initialized!");

    spawner.must_spawn(start_logger(
        peripherals.USB0,
        peripherals.GPIO20,
        peripherals.GPIO19,
    ));
    spawner.must_spawn(test_other_task());

    loop {
        let seconds = Instant::now().duration_since_epoch().as_secs();
        info!("Hello world! {=u64:ts}", seconds);
        Timer::after(Duration::from_secs(1)).await;
    }
}
