//! The Texas Instruments SN76489 and Sega's copy of it in the Master System, Game Gear and Mega Drive: three square
//! waves and a noise channel, each with 15 steps of 2 dB attenuation.

/// One chip, stepped at a sixteenth of its clock.
pub(super) struct Sn76489 {
    /// Steps per output sample, and the fraction of a step carried over.
    step: f64,
    carry: f64,
    periods: [u16; 3],
    noise: u8,
    attenuation: [u8; 4],
    counters: [u16; 4],
    high: [bool; 4],
    lfsr: u32,
    feedback: u32,
    width: u32,
    /// The register the last latch byte chose: channel and whether it's the attenuation.
    latched: (usize, bool),
    /// Amplitude of each attenuation step.
    levels: [f32; 16],
}

impl Sn76489 {
    /// A chip clocked at `clock` Hz, producing `rate` samples a second, with the noise register's taps and width
    /// from the VGM header (0 for the Sega defaults).
    pub(super) fn new(clock: u32, rate: u32, feedback: u16, width: u8) -> Self {
        let mut levels = [0.0; 16];
        for (i, l) in levels.iter_mut().enumerate().take(15) {
            *l = 10f32.powf(-0.1 * i as f32);
        }
        let width = if width == 0 { 16 } else { u32::from(width).min(32) };
        Self {
            step: f64::from(clock) / 16.0 / f64::from(rate),
            carry: 0.0,
            periods: [0; 3],
            noise: 0,
            attenuation: [15; 4],
            counters: [0; 4],
            high: [true; 4],
            lfsr: 1 << (width - 1),
            feedback: if feedback == 0 { 0x0009 } else { u32::from(feedback) },
            width,
            latched: (0, false),
            levels,
        }
    }

    pub(super) fn write(&mut self, byte: u8) {
        let (channel, is_attenuation) = if byte & 0x80 != 0 {
            self.latched = (usize::from(byte >> 5 & 3), byte & 0x10 != 0);
            self.latched
        } else {
            self.latched
        };
        let latch = byte & 0x80 != 0;
        if is_attenuation {
            self.attenuation[channel] = byte & 0x0F;
        } else if channel < 3 {
            let p = &mut self.periods[channel];
            *p =
                if latch { (*p & 0x3F0) | u16::from(byte & 0x0F) } else { (*p & 0x0F) | (u16::from(byte & 0x3F) << 4) };
        } else {
            self.noise = byte & 0x07;
            self.lfsr = 1 << (self.width - 1);
        }
    }

    /// The next output sample, averaged over the chip's steps within it, roughly between -1 and 1.
    pub(super) fn sample(&mut self) -> f32 {
        self.carry += self.step;
        let steps = self.carry as u32;
        self.carry -= f64::from(steps);
        if steps == 0 {
            return self.output();
        }
        let mut sum = 0.0;
        for _ in 0..steps {
            self.tick();
            sum += self.output();
        }
        sum / steps as f32
    }

    fn tick(&mut self) {
        for ch in 0..3 {
            let period = self.periods[ch];
            // A period of 0 or 1 holds the output high, which games use to play samples through the volume.
            if period <= 1 {
                self.high[ch] = true;
                continue;
            }
            self.counters[ch] = self.counters[ch].saturating_sub(1);
            if self.counters[ch] == 0 {
                self.counters[ch] = period;
                self.high[ch] = !self.high[ch];
            }
        }
        let period = match self.noise & 3 {
            0 => 0x10,
            1 => 0x20,
            2 => 0x40,
            _ => self.periods[2].max(1),
        };
        self.counters[3] = self.counters[3].saturating_sub(1);
        if self.counters[3] == 0 {
            self.counters[3] = period;
            self.high[3] = !self.high[3];
            // The shift register moves once a cycle of the noise clock.
            if self.high[3] {
                let fed =
                    if self.noise & 4 != 0 { (self.lfsr & self.feedback).count_ones() & 1 } else { self.lfsr & 1 };
                self.lfsr = (self.lfsr >> 1) | (fed << (self.width - 1));
            }
        }
    }

    fn output(&self) -> f32 {
        let tone = (0..3).map(|ch| {
            let l = self.levels[usize::from(self.attenuation[ch])];
            if self.high[ch] { l } else { -l }
        });
        let noise = self.levels[usize::from(self.attenuation[3])];
        (tone.sum::<f32>() + if self.lfsr & 1 != 0 { noise } else { -noise }) / 4.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays_a_tone_at_its_pitch() {
        let mut chip = Sn76489::new(3_579_545, 44_100, 0, 0);
        // Channel 0: period 254 (about 440 Hz), loudest.
        chip.write(0x80 | 0x0E);
        chip.write(0x0F);
        chip.write(0x90);
        let wave: Vec<f32> = (0..44_100).map(|_| chip.sample()).collect();
        let crossings = wave.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
        assert!((430..=450).contains(&crossings), "{crossings} cycles a second");
        // Silenced.
        chip.write(0x9F);
        assert!((0..100).map(|_| chip.sample()).all(|s| s.abs() < 0.3));
    }
}
