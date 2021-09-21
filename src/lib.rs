//! # `Monotonic` implementation based on SysTick

#![no_std]

use cortex_m::peripheral::{syst::SystClkSource, SYST};
use rtic_monotonic::{
    embedded_time::{clock::Error, fraction::Fraction},
    Clock, Instant, Monotonic,
};

/// Systick implementing `embedded_time::Clock` and `rtic_monotonic::Monotonic` which runs at a
/// settable rate using the `TIMER_HZ` parameter.
pub struct Systick<const TIMER_HZ: u32> {
    systick: SYST,
    cnt: u32,
    reload: u32,
}

impl<const TIMER_HZ: u32> Systick<TIMER_HZ> {
    /// Provide a new `Monotonic` based on SysTick.
    ///
    /// Note that the `sysclk` parameter should come from e.g. the HAL's clock generation function
    /// so the real speed and the declared speed can be compared.
    pub fn new(mut systick: SYST, sysclk: u32) -> Self {
        systick.disable_counter();

        Systick {
            systick,
            cnt: 0,
            reload: (sysclk + TIMER_HZ / 2) / TIMER_HZ - 1,
        }
    }
}

impl<const TIMER_HZ: u32> Clock for Systick<TIMER_HZ> {
    type T = u32;

    const SCALING_FACTOR: Fraction = Fraction::new(1, TIMER_HZ);

    #[inline(always)]
    fn try_now(&self) -> Result<Instant<Self>, Error> {
        // The instant is always valid
        Ok(Instant::new(self.cnt))
    }
}

impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    unsafe fn reset(&mut self) {
        self.systick.set_clock_source(SystClkSource::Core);
        self.systick.set_reload(self.reload);
        self.systick.clear_current();
        self.systick.enable_counter();
    }

    fn set_compare(&mut self, _val: &Instant<Self>) {
        // No need to do something here, we get interrupts every tick anyways.
    }

    fn clear_compare_flag(&mut self) {
        // NOOP with SysTick interrupt
    }

    fn on_interrupt(&mut self) {
        // Increase the counter every overflow.
        self.cnt += 1;
    }
}
