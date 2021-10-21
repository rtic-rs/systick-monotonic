//! # `Monotonic` implementation based on SysTick

#![no_std]

use rtic_monotonic::{MonoInit, Monotonic};

pub struct SystickInternal<const FREQ: u32> {}

pub struct SystickInit<const FREQ: u32> {
    timer: SystickInternal<FREQ>,
}

pub struct Systick<const FREQ: u32> {
    timer: SystickInternal<FREQ>,
}

impl<const FREQ: u32> Systick<FREQ> {
    fn new(m: SystickInit<FREQ>) -> Self {
        Systick { timer: m.timer } // move the actual timer
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Instant {
    instant: i32,
}

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.instant.cmp(&other.instant)
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub struct Duration {}

impl<const FREQ: u32> MonoInit for SystickInit<FREQ> {
    type Instant = Instant;
    type Duration = Duration;
    type Mono = Systick<FREQ>;

    fn now(&mut self) -> Self::Instant {
        todo!()
    }

    /// Turn the Init Type to a monotonic;
    fn into_mono(self) -> Self::Mono {
        Systick::new(self)
    }
}

impl<const FREQ: u32> Monotonic for Systick<FREQ> {
    type Instant = Instant;
    type Duration = Duration;

    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    /// Get the current time instant
    fn now(&mut self) -> Self::Instant {
        // for now
        Instant { instant: 0 }
    }

    /// Set the compare value of the timer interrupt.
    fn set_compare(&mut self, instant: &Self::Instant) {
        todo!()
    }

    /// Clear the compare interrupt flag.
    fn clear_compare_flag(&mut self) {
        todo!()
    }
}

// use cortex_m::peripheral::{syst::SystClkSource, SYST};
// use rtic_monotonic::{
//     embedded_time::{clock::Error, fraction::Fraction},
//     Clock, Instant, Monotonic,
// };

// /// Systick implementing `embedded_time::Clock` and `rtic_monotonic::Monotonic` which runs at a
// /// settable rate using the `TIMER_HZ` parameter.
// pub struct Systick<const TIMER_HZ: u32> {
//     systick: SYST,
//     cnt: u32,
//     reload: u32,
// }

// impl<const TIMER_HZ: u32> Systick<TIMER_HZ> {
//     /// Provide a new `Monotonic` based on SysTick.
//     ///
//     /// Note that the `sysclk` parameter should come from e.g. the HAL's clock generation function
//     /// so the real speed and the declared speed can be compared.
//     pub fn new(mut systick: SYST, sysclk: u32) -> Self {
//         systick.disable_counter();

//         Systick {
//             systick,
//             cnt: 0,
//             reload: (sysclk + TIMER_HZ / 2) / TIMER_HZ - 1,
//         }
//     }
// }

// impl<const TIMER_HZ: u32> Clock for Systick<TIMER_HZ> {
//     type T = u32;

//     const SCALING_FACTOR: Fraction = Fraction::new(1, TIMER_HZ);

//     #[inline(always)]
//     fn try_now(&self) -> Result<Instant<Self>, Error> {
//         // The instant is always valid
//         Ok(Instant::new(self.cnt))
//     }
// }

// impl<const TIMER_HZ: u32> Monotonic for Systick<TIMER_HZ> {
//     const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = false;

//     unsafe fn reset(&mut self) {
//         self.systick.set_clock_source(SystClkSource::Core);
//         self.systick.set_reload(self.reload);
//         self.systick.clear_current();
//         self.systick.enable_counter();
//     }

//     fn set_compare(&mut self, _val: &Instant<Self>) {
//         // No need to do something here, we get interrupts every tick anyways.
//     }

//     fn clear_compare_flag(&mut self) {
//         // NOOP with SysTick interrupt
//     }

//     fn on_interrupt(&mut self) {
//         // Increase the counter every overflow.
//         self.cnt += 1;
//     }
// }
