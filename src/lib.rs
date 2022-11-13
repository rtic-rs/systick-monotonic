//! # [`Monotonic`] implementation based on SysTick
//!
//! Uses [`fugit`] as underlying time library.
//!
//! [`fugit`]: https://docs.rs/crate/fugit
//! [`Monotonic`]: https://docs.rs/rtic-monotonic

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
    /// The `sysclk` parameter is the speed at which SysTick runs at. This value should come from
    /// the clock generation function of the used HAL.
    ///
    /// Notice that the actual rate of the timer is a best approximation based on the given
    /// `sysclk` and `TIMER_HZ`.
    pub fn new(mut systick: SYST, sysclk: u32) -> Self {
        // + TIMER_HZ / 2 provides round to nearest instead of round to 0.
        // - 1 as the counter range is inclusive [0, reload]
        let reload = (sysclk + TIMER_HZ / 2) / TIMER_HZ - 1;

        assert!(reload <= 0x00ff_ffff);
        assert!(reload > 0);

        systick.disable_counter();
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(reload);

        Systick { systick, cnt: 0 }
    }
}

impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

    type Instant = fugit::TimerInstantU64<TIMER_HZ>;
    type Duration = fugit::TimerDurationU64<TIMER_HZ>;

    fn now(&mut self) -> Self::Instant {
        if self.systick.has_wrapped() {
            self.cnt = self.cnt.wrapping_add(1);
        }

        Self::Instant::from_ticks(self.cnt)
    }

    unsafe fn reset(&mut self) {
        self.systick.clear_current();
        self.systick.enable_counter();
    }

    #[inline(always)]
    fn set_compare(&mut self, _val: Self::Instant) {
        // No need to do something here, we get interrupts anyway.
    }

    #[inline(always)]
    fn clear_compare_flag(&mut self) {
        // NOOP with SysTick interrupt
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }

    fn on_interrupt(&mut self) {
        if self.systick.has_wrapped() {
            self.cnt = self.cnt.wrapping_add(1);
        }
    }
}
