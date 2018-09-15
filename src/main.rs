/*
* This program will implement the lab 1 of 3040
*/

extern crate stm32l1;
use stm32l1::stm32l100;

fn main() {
    // Lets init our toggle counter
    let mut toggles = 0;
    let mut switch = false;
    let mut led_state = false;

    // Lets take ownership of all the peripherals
    let mut peripherals = stm32l100::Peripherals::take().unwrap();
    
    pin_setup(&peripherals);

    loop {
        if led_state == false {
            peripherals.GPIOC.bsrr.write(|w| w.br8().set_bit());
        } else {
            peripherals.GPIOC.bsrr.write(|w| w.bs8().set_bit());
        }

        while peripherals.GPIOA.idr.read(|r| r.bit_is_clear()) {}

        delay();
        led_state = !led_state;
        toggles += 1;
    }
}

fn pin_setup(peripherals: &stm32l100::Peripherals) {
    // Lets enable the GPIOA clock
    peripherals.RCC.ahbenr.modify(|_, w| w.gpiopaen().set_bit());
    // Now lets set PA0 to general purpose input mode
    peripherals.GPIOA.moder.modify(|_, w| w.moder0().output());

    // Enable GPIOE clock
    peripherals.RCC.ahbenr.modify(|_, w| w.gpiopeen().set_bit());
    // Set PC8, PC9 to output
    peripherals.GPIOC.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
    })
}

fn delay() {
    let n = 0;
    for i in 0..20 {
        for j in 0..20_000 {
             n = j;
        }
    }
}
