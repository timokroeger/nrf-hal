//! Configuration and control of the High and Low Frequency Clock sources.

#[cfg(feature = "9160")]
use crate::pac::CLOCK_NS as CLOCK;

#[cfg(not(feature = "9160"))]
use crate::pac::CLOCK;

/// High Frequency Clock Frequency (in Hz).
pub const HFCLK_FREQ: u32 = 64_000_000;
/// Low Frequency Clock Frequency (in Hz).
pub const LFCLK_FREQ: u32 = 32_768;

/// Allowable configuration options for the low frequency oscillator when
/// driven fron an external crystal.
pub enum LfOscConfiguration {
    NoExternalNoBypass,
    ExternalNoBypass,
    ExternalAndBypass,
}

/// A high level abstraction for the CLOCK peripheral.
pub struct Clocks {
    periph: CLOCK,
}

impl Clocks {
    pub fn new(clock: CLOCK) -> Clocks {
        Clocks {
            periph: clock,
        }
    }

    /// Use an external oscillator as the high frequency clock source.
    pub fn enable_ext_hfosc(&self) {
        self.periph.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });

        // Datasheet says this is likely to take 0.36ms
        while self.periph.events_hfclkstarted.read().bits() != 1 {}
        self.periph
            .events_hfclkstarted
            .write(|w| unsafe { w.bits(0) });
    }

    /// Use the internal oscillator as the high frequency clock source.
    pub fn disable_ext_hfosc(&self) {
        self.periph.tasks_hfclkstop.write(|w| unsafe { w.bits(1) });
    }

    /// Start the Low Frequency clock.
    pub fn start_lfclk(&self) {
        self.periph.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        // Datasheet says this could take 100us from synth source
        // 600us from rc source, 0.25s from an external source.
        while self.periph.events_lfclkstarted.read().bits() != 1 {}
        self.periph
            .events_lfclkstarted
            .write(|w| unsafe { w.bits(0) });

    }

    /// Stop the Low Frequency clock.
    pub fn stop_lfclk(&self) {
        self.periph.tasks_lfclkstop.write(|w| unsafe { w.bits(1) });
    }

    /// Use the internal RC Oscillator for the low frequency clock source.
    #[cfg(feature = "51")]
    pub fn set_lfclk_src_rc(&self) {
        self.periph.lfclksrc.write(|w| w.src().rc());
    }

    /// Generate the Low Frequency clock from the high frequency clock source.
    #[cfg(feature = "51")]
    pub fn set_lfclk_src_synth(&self) {
        self.periph.lfclksrc.write(|w| w.src().synth());
    }

    /// Use an external crystal to drive the low frequency clock.
    #[cfg(feature = "51")]
    pub fn set_lfclk_src_external(&self) {
        self.periph.lfclksrc.write(move |w| w.src().xtal());
    }

    /// Use the internal RC Oscillator for the low frequency clock source.
    #[cfg(not(any(feature = "9160", feature = "51")))]
    pub fn set_lfclk_src_rc(&self) {
        self.periph
            .lfclksrc
            .write(|w| w.src().rc().bypass().disabled().external().disabled());
    }

    /// Generate the Low Frequency clock from the high frequency clock source.
    #[cfg(not(any(feature = "9160", feature = "51")))]
    pub fn set_lfclk_src_synth(&self) {
        self.periph
            .lfclksrc
            .write(|w| w.src().synth().bypass().disabled().external().disabled());
    }

    /// Use an external crystal to drive the low frequency clock.
    #[cfg(not(any(feature = "9160", feature = "51")))]
    pub fn set_lfclk_src_external(
        &self,
        cfg: LfOscConfiguration,
    ) {
        let (ext, byp) = match cfg {
            LfOscConfiguration::NoExternalNoBypass => (false, false),
            LfOscConfiguration::ExternalNoBypass => (true, false),
            LfOscConfiguration::ExternalAndBypass => (true, true),
        };
        self.periph
            .lfclksrc
            .write(move |w| w.src().xtal().bypass().bit(byp).external().bit(ext));
    }
}
