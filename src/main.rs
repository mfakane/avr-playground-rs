#![no_std]
#![no_main]

use panic_halt as _;

/*
 * 8-pin SOIC Mapping for ATtiny402:
 *       +---+
 * VDD 1 |   | 8 GND
 * PA6 2 |   | 7 PA3 (EXTCLK)
 * PA7 3 |   | 6 PA0 (RESET/UPDI)
 * PA1 4 |   | 5 PA2
 *       +---+
 * 
 * Supported Features:
 * PA0: RESET/UPDI, AIN0, XDIR, SS, LUT0-IN0
 * PA1: AIN1, TXD*, MOSI, SDA, TCA0-WO1, LUT0-IN1
 * PA2: EVOUT0, AIN2, RXD*, MISO, SCL, TCA0-WO2, LUT0-IN2
 * PA3: EXTCLK, AIN3, AC0-OUT, XCK, SCK, TCA0-WO0/WO3
 * PA6: AIN6, AC0-AINN0, TXD, MOSI*, TCB0-WO0, LUT0-OUT
 * PA7: AIN7, AC0-AINP0, RXD, MISO*, TCA0-WO0*, LUT1-OUT
 * (*) Alternate pin positions
 */

#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::attiny402::Peripherals::take().unwrap();

    // Configure PA7 (LED) as output and PA1 (ADC input) as input
    dp.PORTA.dir.write(|w| w.pa7().set_bit().pa1().clear_bit());
    dp.PORTMUX.ctrlc.write(|w| w.tca00().alternate());

    adc_init(&dp.ADC0);
    pwm_init(&dp.TCA0);

    loop {
        let adc_value = adc_read(&dp.ADC0);
        let duty_cycle = (adc_value >> 2) as u8; // Scale 10-bit to 8-bit
        pwm_set_duty(&dp.TCA0, duty_cycle);
        
        // Longer delay for more stable readings
        for _ in 0..1000 {
            avr_device::asm::delay_cycles(100);
        }
    }
}

fn adc_init(adc: &avr_device::attiny402::ADC0) {
    adc.muxpos.write(|w| w.muxpos().ain1());
    adc.ctrlc.write(|w| w.refsel().vddref().presc().div16());
    adc.ctrla.write(|w| w.enable().set_bit());
}

fn adc_read(adc: &avr_device::attiny402::ADC0) -> u16 {
    adc.command.write(|w| w.stconv().set_bit());
    while adc.intflags.read().resrdy().bit_is_clear() {}
    adc.intflags.write(|w| w.resrdy().set_bit());
    return adc.res.read().bits();
}

fn pwm_init(tca: &avr_device::attiny402::TCA0) {
    tca.single_ctrld().write(|w| w.splitm().clear_bit());
    tca.single_per().write(|w| w.bits(0xFF));
    tca.single_cmp0().write(|w| w.bits(0));
    tca.single_ctrlb().write(|w| w.wgmode().singleslope().cmp0en().set_bit());
    tca.single_ctrla().write(|w| w.enable().set_bit().clksel().div8());
}

fn pwm_set_duty(tca: &avr_device::attiny402::TCA0, duty: u8) {
    tca.single_cmp0().write(|w| w.bits(duty as u16));
}