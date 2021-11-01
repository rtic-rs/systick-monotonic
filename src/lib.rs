//! # `Monotonic` implementation based on SysTick
//!
//! Uses [`fugit`] as underlying time library.
//!
//! [`fugit`]: https://docs.rs/crate/fugit

#![no_std]

use cortex_m::peripheral::{syst::SystClkSource, SYST};
pub use fugit::{self, ExtU64};
use rtic_monotonic::Monotonic;

/// Systick implementing `rtic_monotonic::Monotonic` which runs at a
/// settable rate using the `TIMER_HZ` parameter.
pub struct Systick<const TIMER_HZ: u32> {
    systick: SYST,
    cnt: u64,
}

impl<const TIMER_HZ: u32> Systick<TIMER_HZ> {
    /// Provide a new `Monotonic` based on SysTick.
    ///
    /// Note that the `sysclk` parameter should come from e.g. the HAL's clock generation function
    /// so the real speed and the declared speed can be compared.
    pub fn new(mut systick: SYST, sysclk: u32) -> Self {
        let reload = (sysclk + TIMER_HZ / 2) / TIMER_HZ - 1;

        systick.disable_counter();
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(reload);

        Systick {
            systick,
            cnt: 0,
        }
    }
}

impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;
    const ZERO: Self::Instant = Self::Instant::from_ticks(0);

    type Instant = fugit::TimerInstantU64<TIMER_HZ>;
    type Duration = fugit::TimerDurationU64<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        if self.systick.has_wrapped() {
            self.cnt += 1;
        }

        Self::Instant::from_ticks(self.cnt)
    }

    unsafe fn reset(&mut self) {
        self.systick.clear_current();
        self.systick.enable_counter();
    }

    #[inline(always)]
    fn set_compare(&mut self, _val: Self::Instant) {
        // No need to do something here, we get interrupts every tick anyways.
    }

    #[inline(always)]
    fn clear_compare_flag(&mut self) {
        // NOOP with SysTick interrupt
    }

    fn on_interrupt(&mut self) {
        if self.systick.has_wrapped() {
            self.cnt += 1;
        }
    }
}
