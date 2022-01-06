//! This example is for the nRF 52 DK 

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use defmt_rtt as _;
use cortex_m_rt::entry;
use defmt::*;
use defmt::panic;
use embedded_hal::blocking::spi::*;
use nrf52840_hal:: {gpio,
                    spim,
                    gpio::p0,   
                    gpio::Level,
                    Spim,
                    };

use iis3dwb::{Config as IIS3DWBConfig, Range, IIS3DWB, 
                        Accelerometer, RawAccelerometer,
                        FifoAccBatchDataRate,
                        FifoMode,
                        FifoTempBatchDataRate,
                        FifoTimestampDecimation,
                        Watermark};


use panic_probe as _;
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}


#[entry]
fn main() -> ! {
    info!("running!");

    let p = nrf52840_hal::pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.P0);
    let ncs = port0.p0_14.into_push_pull_output(Level::High);
    let spiclk = port0.p0_13.into_push_pull_output(Level::Low).degrade();
    let spimiso = port0.p0_15.into_floating_input().degrade();
    let spimosi = port0.p0_16.into_push_pull_output(Level::Low).degrade();

    let spi_pins = nrf52840_hal::spim::Pins {
        sck: spiclk,
        miso: Some(spimiso),
        mosi: Some(spimosi),
    };

    let mut spi =   Spim::new(
        p.SPIM3,
        spi_pins,
        nrf52840_hal::spim::Frequency::M1,
        nrf52840_hal::spim::MODE_3, 
        0
    );

    let mut acc_cfg = IIS3DWBConfig::default();

    acc_cfg.fifo.mode = FifoMode::FifoMode;
    acc_cfg.fifo.temperature = FifoTempBatchDataRate::BDR104Hz;
    acc_cfg.fifo.timestamp = FifoTimestampDecimation::Decimation1;
    acc_cfg.fifo.acceleration = FifoAccBatchDataRate::BDR26667Hz;
    acc_cfg.fifo.watermark = Watermark::from_bytes(0xFF);
    let mut accelerometer = IIS3DWB::new(spi, ncs, &acc_cfg).unwrap();
    let id = accelerometer.get_device_id();
    defmt::info!("The device ID is: 0x{=u8:x}", id);
    // let temp = accelerometer.read_temp_raw();
    // defmt::info!("The device temperature is: 0x{=u16:x}", temp);


    accelerometer.start();
    accelerometer.set_timestamp_en(true);
    accelerometer.accel_raw();
    loop{
        cortex_m::asm::delay(5000000);   // KISS.
        defmt::info!("{}",accelerometer.unread_data_count());
    }

    exit();
}
