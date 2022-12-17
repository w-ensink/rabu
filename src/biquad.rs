/// The coefficients for a `BiquadFilter`.
pub struct BiquadCoefficients {
    pub a1: f64,
    pub a2: f64,
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
}

/// A biquad filter used to filter audio signals.
pub struct BiquadFilter {
    coefficients: BiquadCoefficients,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl BiquadFilter {
    /// Creates a new biquad filter using the provided coefficients.
    pub fn new(coefficients: BiquadCoefficients) -> Self {
        Self {
            coefficients,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Sets the coefficients to the provided ones.
    pub fn set_coefficients(&mut self, coefficients: BiquadCoefficients) {
        self.coefficients = coefficients;
    }

    /// Processes one sample of input audio and produces the filter output sample.
    pub fn process(&mut self, input: f64) -> f64 {
        let output = self.coefficients.b0 * input
            + self.coefficients.b1 * self.x1
            + self.coefficients.b2 * self.x2
            - self.coefficients.a1 * self.y1
            - self.coefficients.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        output
    }
}

/// Creates the biquad coefficients for a low pass filter,
/// given a sample rate and a cutoff frequency.
pub fn low_pass_coefficients(sample_rate: f64, cutoff_frequency: f64) -> BiquadCoefficients {
    let w0 = 2.0 * std::f64::consts::PI * cutoff_frequency / sample_rate;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = sin_w0 / (2.0 * 0.5);

    let b0 = (1.0 - cos_w0) / 2.0;
    let b1 = 1.0 - cos_w0;
    let b2 = (1.0 - cos_w0) / 2.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * cos_w0;
    let a2 = 1.0 - alpha;

    BiquadCoefficients {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

/// Creates the biquad coefficients for a band pass filter,
/// given a sample rate and a cutoff frequency.
pub fn band_pass_coefficients(
    sample_rate: f64,
    center_frequency: f64,
    bandwidth: f64,
) -> BiquadCoefficients {
    let w0 = 2.0 * std::f64::consts::PI * center_frequency / sample_rate;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = sin_w0 * std::f64::consts::SQRT_2 / 2.0 * bandwidth / center_frequency;

    let b0 = sin_w0 / 2.0;
    let b1 = 0.0;
    let b2 = -sin_w0 / 2.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * cos_w0;
    let a2 = 1.0 - alpha;

    BiquadCoefficients {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}

/// Creates the biquad coefficients for a high pass filter,
/// given a sample rate and a cutoff frequency.
pub fn high_pass_coefficients(sample_rate: f64, cutoff_frequency: f64) -> BiquadCoefficients {
    let w0 = 2.0 * std::f64::consts::PI * cutoff_frequency / sample_rate;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = sin_w0 / (2.0 * 0.5);

    let b0 = (1.0 + cos_w0) / 2.0;
    let b1 = -(1.0 + cos_w0);
    let b2 = (1.0 + cos_w0) / 2.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * cos_w0;
    let a2 = 1.0 - alpha;

    BiquadCoefficients {
        b0: b0 / a0,
        b1: b1 / a0,
        b2: b2 / a0,
        a1: a1 / a0,
        a2: a2 / a0,
    }
}
