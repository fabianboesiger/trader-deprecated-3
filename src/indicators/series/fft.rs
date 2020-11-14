use super::Series;

use stft::{STFT, WindowType};

pub struct Fft<const PERIOD: usize> {
    stft: STFT::<f64>,
    spectrogram: [f64; PERIOD],
}

impl<const PERIOD: usize> Series for Fft<PERIOD> {
    type Analysis = [f64; PERIOD];

    fn new() -> Self {
        let stft = STFT::new(WindowType::Hanning, PERIOD, 1);
        let spectrogram = [0.0; PERIOD];

        Fft {
            stft,
            spectrogram,
        }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<Self::Analysis> {
        self.stft.append_samples(&vec![value][..]);

        if self.stft.contains_enough_to_compute() {
            self.stft.compute_column(&mut self.spectrogram[..]);
            let result = self.spectrogram.clone();
            self.stft.move_to_next_column();
            Some(result)
        } else {
            None
        }
    }    
}