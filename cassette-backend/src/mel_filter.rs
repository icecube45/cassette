use ndarray::{Array2, Array1, Array};
use realfft::num_traits::abs;
use itertools::izip;


// Mel-Frequency from linear freq.
pub fn hertz_to_mel(freq: f64) -> f64 {
    2595.0 * (1.0 + freq / 700.0).log10()
}

pub fn mel_to_hertz(mel: f64) -> f64 {
    700.0 * (10.0_f64.powf(mel / 2595.0)) - 700.0
}

// return center frequencies and band edges for mel filterbank
pub fn get_melfrequencies(num_bands: u32, freq_min: f64, freq_max: f64, num_fft_bands: u32) -> (Vec<f64>, Vec<f64>, Vec<f64>){
    let mel_max = hertz_to_mel(freq_max);
    let mel_min = hertz_to_mel(freq_min);
    let delta_mel = abs(mel_max - mel_min) / (num_bands as f64 + 1.0);
    let frequencies_mel = (0..num_bands+2).map(|i| i as f64 * delta_mel + mel_min).collect::<Vec<f64>>();
    let lower_edges_mel = frequencies_mel[..frequencies_mel.len()-3].to_vec();
    let upper_edges_mel = frequencies_mel[2..].to_vec();
    let center_freqs_mel = frequencies_mel[1..frequencies_mel.len()-2].to_vec();
    return (center_freqs_mel, lower_edges_mel, upper_edges_mel);
}

// returns ndarray of transformation matrix for the mel spectrum
pub fn compute_melmat(num_mel_bands: u32, freq_min: f64, freq_max: f64,
                      num_fft_bands: u32, sample_rate: u32) -> (Array2<f64>, Vec<f64>, Array1<f64>) {
    let (center_freqs_mel, lower_edges_mel, upper_edges_mel) = get_melfrequencies(num_mel_bands, freq_min, freq_max, num_fft_bands);

    let center_frequencies_hz = center_freqs_mel.iter().map(|melfreq| mel_to_hertz(*melfreq)).collect::<Vec<f64>>();
    let lower_edges_hz = lower_edges_mel.iter().map(|melfreq| mel_to_hertz(*melfreq)).collect::<Vec<f64>>();
    let upper_edges_hz = upper_edges_mel.iter().map(|melfreq| mel_to_hertz(*melfreq)).collect::<Vec<f64>>();
    let freqs = Array::linspace(0.0, sample_rate as f64 / 2.0, num_fft_bands as usize);
    let mut melmat = Array2::zeros((num_mel_bands as usize, num_fft_bands as usize));

    for (imelband, (center, lower, upper)) in izip!(center_frequencies_hz.iter(), lower_edges_hz.iter(), upper_edges_hz.iter()).enumerate() {
        let left_slope = freqs.mapv(|freq| (freq >= *lower) == (freq <= *center));
        let right_slope = freqs.mapv(|freq| (freq >= *center) == (freq <= *upper));

        // for each melmat row, set the values where left_slope is true to 1.0
        for i in 0..left_slope.len() {
            if left_slope[i] {
                melmat[[imelband, i]] = (freqs[i] - *lower) / (center - lower);
            }
            if right_slope[i] {
                melmat[[imelband, i]] = (upper - freqs[i]) / (upper - center);
            }
        }
    }
    return (melmat, center_freqs_mel, freqs);
}