//! STM32 SVD to bindings for Drone, an Embedded Operating System.

#![feature(generators)]
#![feature(generator_trait)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::pedantic)]

pub mod adc;
pub mod dma;
pub mod dmamux;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod pwr;
pub mod rcc;
pub mod rtc;
pub mod spi;
pub mod tim;
pub mod uart;

pub use anyhow::{bail, Result};

use drone_svd::Device;
use std::{env, fs::File, path::Path};

const REG_EXCLUDE: &[&str] = &[
    "FPU",
    "FPU_CPACR",
    "ITM",
    "MPU",
    "NVIC",
    "SCB",
    "STK",
    "TPIU",
];

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut regs = File::create(out_dir.join("svd_regs.rs"))?;
    dev.generate_regs(&mut regs, REG_EXCLUDE, pool_number, pool_size)
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut reg_tokens = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
    dev.generate_rest(
        &mut reg_tokens,
        &mut interrupts,
        REG_EXCLUDE,
        "stm32_reg_tokens",
    )
}

fn svd_deserialize() -> Result<Device> {
    drone_svd::rerun_if_env_changed();
    match env::var("CARGO_CFG_STM32_MCU")?.as_ref() {
        "stm32f100" => parse_svd("STM32F100.svd"),
        "stm32f101" => parse_svd("STM32F101.svd"),
        "stm32f102" => patch_stm32f102(parse_svd("STM32F102.svd")?),
        "stm32f103" => parse_svd("STM32F103.svd"),
        "stm32f107" => parse_svd("STM32F107.svd"),
        "stm32f401" => patch_stm32f401(parse_svd("STM32F401.svd")?),
        "stm32f405" => patch_stm32f405(parse_svd("STM32F405.svd")?),
        "stm32f407" => patch_stm32f407(parse_svd("STM32F407.svd")?),
        "stm32f410" => patch_stm32f410(parse_svd("STM32F410.svd")?),
        "stm32f411" => patch_stm32f411(parse_svd("STM32F411.svd")?),
        "stm32f412" => patch_stm32f412(parse_svd("STM32F412.svd")?),
        "stm32f413" => patch_stm32f413(parse_svd("STM32F413.svd")?),
        "stm32f427" => patch_stm32f427(parse_svd("STM32F427.svd")?),
        "stm32f429" => patch_stm32f429(parse_svd("STM32F429.svd")?),
        "stm32f446" => patch_stm32f446(parse_svd("STM32F446.svd")?),
        "stm32f469" => patch_stm32f469(parse_svd("STM32F469.svd")?),
        "stm32l4x1" => patch_stm32l4x1(parse_svd("STM32L4x1.svd")?),
        "stm32l4x2" => patch_stm32l4x2(parse_svd("STM32L4x2.svd")?),
        "stm32l4x3" => patch_stm32l4x3(parse_svd("STM32L4x3.svd")?),
        "stm32l4x5" => patch_stm32l4x5(parse_svd("STM32L4x5.svd")?),
        "stm32l4x6" => patch_stm32l4x6(parse_svd("STM32L4x6.svd")?),
        "stm32l4r5" => patch_stm32l4plus(parse_svd("STM32L4R5.svd")?),
        "stm32l4r7" => patch_stm32l4plus(parse_svd("STM32L4R7.svd")?),
        "stm32l4r9" => patch_stm32l4plus(parse_svd("STM32L4R9.svd")?),
        "stm32l4s5" => patch_stm32l4plus(parse_svd("STM32L4S5.svd")?),
        "stm32l4s7" => patch_stm32l4plus(parse_svd("STM32L4S7.svd")?),
        "stm32l4s9" => patch_stm32l4plus(parse_svd("STM32L4S9.svd")?),
        _ => bail!("invalid `stm32_mcu` cfg flag"),
    }
}

fn patch_stm32f102(mut dev: Device) -> Result<Device> {
    spi::fix_spi2_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f401(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim10_2(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    tim::fix_tim11_2(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f405(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    rcc::fix_3(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    dma::fix_dma2_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim10_2(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    tim::fix_tim11_2(&mut dev)?;
    adc::fix_adc_com(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f407(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    rcc::fix_3(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    dma::fix_dma2_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim10_2(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    tim::fix_tim11_2(&mut dev)?;
    adc::fix_adc_com(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f410(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim5_2(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f411(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim10_2(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    tim::fix_tim11_2(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f412(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim6(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f413(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    dma::fix_dma1(&mut dev)?;
    exti::fix_exti_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim7(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f427(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    rcc::fix_3(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    dma::fix_dma2_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim10_2(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    tim::fix_tim11_2(&mut dev)?;
    adc::fix_adc_com(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f429(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    rcc::fix_3(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    dma::fix_dma2_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim10_2(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    tim::fix_tim11_2(&mut dev)?;
    adc::fix_adc_com(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f446(mut dev: Device) -> Result<Device> {
    rcc::fix_2(&mut dev)?;
    dma::fix_dma2_1(&mut dev)?;
    dma::fix_dma2_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    adc::fix_adc_com(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32f469(mut dev: Device) -> Result<Device> {
    dma::fix_dma2_1(&mut dev)?;
    dma::fix_dma2_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim2_2(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim3_3(&mut dev)?;
    tim::fix_tim5_1(&mut dev)?;
    tim::fix_tim9_1(&mut dev)?;
    tim::fix_tim10_1(&mut dev)?;
    tim::fix_tim11_1(&mut dev)?;
    adc::fix_adc_com(&mut dev)?;
    adc::fix_adc1_1(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x1(mut dev: Device) -> Result<Device> {
    rcc::fix_4(&mut dev)?;
    tim::fix_lptim1(&mut dev)?;
    tim::fix_lptim2(&mut dev)?;
    uart::fix_lpuart1(&mut dev)?;
    rcc::fix_1(&mut dev)?;
    spi::fix_spi2_2(&mut dev)?;
    spi::fix_spi3_1(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim1_2(&mut dev)?;
    tim::fix_tim16(&mut dev)?;
    tim::fix_tim2_1(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim15(&mut dev)?;
    tim::fix_tim3_1(&mut dev)?;
    tim::fix_tim3_2(&mut dev)?;
    uart::fix_uart4(&mut dev)?;
    uart::fix_usart1(&mut dev)?;
    uart::fix_usart3(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x2(mut dev: Device) -> Result<Device> {
    rcc::fix_4(&mut dev)?;
    i2c::fix(&mut dev)?;
    tim::fix_lptim1(&mut dev)?;
    tim::fix_lptim2(&mut dev)?;
    uart::fix_lpuart1(&mut dev)?;
    rcc::fix_1(&mut dev)?;
    spi::fix_spi2_2(&mut dev)?;
    spi::fix_spi3_1(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim1_2(&mut dev)?;
    tim::fix_tim16(&mut dev)?;
    tim::fix_tim2_1(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim15(&mut dev)?;
    tim::fix_tim3_1(&mut dev)?;
    tim::fix_tim3_2(&mut dev)?;
    uart::fix_uart4(&mut dev)?;
    uart::fix_usart1(&mut dev)?;
    uart::fix_usart3(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x3(mut dev: Device) -> Result<Device> {
    tim::add_tim3(&mut dev)?;
    rcc::fix_4(&mut dev)?;
    tim::fix_lptim1(&mut dev)?;
    tim::fix_lptim2(&mut dev)?;
    rcc::fix_1(&mut dev)?;
    spi::fix_spi3_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim1_2(&mut dev)?;
    tim::fix_tim16(&mut dev)?;
    tim::fix_tim2_1(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim15(&mut dev)?;
    tim::fix_tim3_1(&mut dev)?;
    tim::fix_tim3_2(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x5(mut dev: Device) -> Result<Device> {
    rcc::fix_4(&mut dev)?;
    exti::fix_exti_1(&mut dev)?;
    tim::fix_lptim1(&mut dev)?;
    tim::fix_lptim2(&mut dev)?;
    rcc::fix_1(&mut dev)?;
    rtc::fix(&mut dev)?;
    spi::fix_spi3_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim1_2(&mut dev)?;
    tim::fix_tim16(&mut dev)?;
    tim::fix_tim2_1(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim15(&mut dev)?;
    tim::fix_tim3_1(&mut dev)?;
    tim::fix_tim8(&mut dev)?;
    gpio::add_ascr(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4x6(mut dev: Device) -> Result<Device> {
    rcc::fix_4(&mut dev)?;
    exti::fix_exti_1(&mut dev)?;
    tim::fix_lptim1(&mut dev)?;
    tim::fix_lptim2(&mut dev)?;
    rcc::fix_1(&mut dev)?;
    spi::fix_spi3_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim1_2(&mut dev)?;
    tim::fix_tim16(&mut dev)?;
    tim::fix_tim2_1(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim15(&mut dev)?;
    tim::fix_tim3_1(&mut dev)?;
    tim::fix_tim8(&mut dev)?;
    Ok(dev)
}

fn patch_stm32l4plus(mut dev: Device) -> Result<Device> {
    dmamux::add_dmamux1(&mut dev)?;
    rcc::fix_4(&mut dev)?;
    exti::fix_exti_1(&mut dev)?;
    tim::fix_lptim1(&mut dev)?;
    tim::fix_lptim2(&mut dev)?;
    pwr::fix(&mut dev)?;
    spi::fix_spi3_2(&mut dev)?;
    tim::fix_tim1_1(&mut dev)?;
    tim::fix_tim1_2(&mut dev)?;
    tim::fix_tim16(&mut dev)?;
    tim::fix_tim2_1(&mut dev)?;
    tim::fix_tim2_3(&mut dev)?;
    tim::fix_tim15(&mut dev)?;
    tim::fix_tim3_1(&mut dev)?;
    tim::fix_tim8(&mut dev)?;
    adc::fix_adc_1(&mut dev)?;
    Ok(dev)
}

fn copy_reg(dev: &mut Device, periph_from: &str, periph_to: &str, reg_name: &str) {
    let reg = dev.periph(periph_from).reg(reg_name).clone();
    dev.periph(periph_to).add_reg(reg);
}

fn copy_field(
    dev: &mut Device,
    periph_from: &str,
    periph_to: &str,
    reg_name: &str,
    field_name: &str,
) {
    let field = dev
        .periph(periph_from)
        .reg(reg_name)
        .field(field_name)
        .clone();
    dev.periph(periph_to).reg(reg_name).add_field(field);
}

fn parse_svd(path: &str) -> Result<Device> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}
