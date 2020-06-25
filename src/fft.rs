use crate::complex;

pub fn butterfly<T>(data: &mut Vec<T>) {
    let mut target: u32 = 0;
    for position in 0..data.len() {
        let u_target = target as usize;
        if u_target > position {
            data.swap(position, u_target);
        }
        let mut mask: u32 = data.len() as u32;
        mask >>= 1;
        let zero = 0;
        while (target & mask) != zero {
            target &= !mask;
            mask >>= 1;
        }
        target |= mask;
    }
}

pub fn fft(data: &mut Vec<complex::Complex>) {
    butterfly(data);
    let mut step = 1;
    let length = data.len();
    while step < length {
        let jump = step << 1;

        let delta = -1.0 * std::f64::consts::PI / step as f64;

        let temp_sin = (delta * 0.5).sin();

        let factor_multiplier = complex::Complex::new(-2.0 * temp_sin * temp_sin, delta.sin());
        let mut factor = complex::Complex::new(1.0, 0.0);

        for group in 0..step {
            for pair in (group..data.len()).step_by(jump) {
                let matched = pair + step;
                let product = complex::Complex::multiply(&factor, &data[matched]);
                data[matched] = complex::Complex::minus(&data[pair], &product);
                data[pair] = complex::Complex::add(&data[pair], &product);
            }
            factor = complex::Complex::add(
                &complex::Complex::multiply(&factor_multiplier, &factor),
                &factor,
            );
        }

        step <<= 1;
    }
}

pub fn dft(data: Vec<complex::Complex>) -> Vec<complex::Complex> {
    let mut results: Vec<complex::Complex> = Vec::new();

    for term in 0..data.len() {
        let mut sum = complex::Complex::new(0.0, 0.0);
        for n in 0..data.len() {
            let angle =
                std::f64::consts::PI * 2.0 * (term as f64) * (n as f64) / (data.len() as f64);
            let exp = complex::Complex::new(angle.cos(), -1.0 * angle.sin());
            let mult = complex::Complex::multiply(&data[n], &exp);
            sum = complex::Complex::add(&sum, &mult);
        }
        results.push(sum);
    }
    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn butterfly_test() {
        let a = complex::Complex::new(1.0, 2.0);
        let b = complex::Complex::new(3.0, 4.0);
        let c = complex::Complex::new(-5.0, 10.0);
        let d = complex::Complex::new(4.5, -67.4);
        let e = complex::Complex::new(45.5, 98.9);
        let f = complex::Complex::new(32.4, -0.87);
        let g = complex::Complex::new(3.72, 87.0);
        let h = complex::Complex::new(2.65, -7.0);

        let mut data = vec![a, b, c, d, e, f, g, h];
        butterfly(&mut data);
        assert_eq!(complex::Complex::new(1.0, 2.0), data[0]);
        assert_eq!(complex::Complex::new(3.0, 4.0), data[4]);
        assert_eq!(complex::Complex::new(-5.0, 10.0), data[2]);
        assert_eq!(complex::Complex::new(4.5, -67.4), data[6]);
        assert_eq!(complex::Complex::new(45.5, 98.9), data[1]);
        assert_eq!(complex::Complex::new(32.4, -0.87), data[5]);
        assert_eq!(complex::Complex::new(3.72, 87.0), data[3]);
        assert_eq!(complex::Complex::new(2.65, -7.0), data[7]);
    }

    fn assert_float_eq(expected: f64, actual: f64, epsilon: f64) {
        let diff = (expected - actual).abs();
        if diff > epsilon {
            panic!(
                "Actual value {} differs by {} compared to expected of {}. Tolerance: {}",
                actual, diff, expected, epsilon
            );
        }
    }

    pub fn assert_complex_eq(expected: &complex::Complex, actual: &complex::Complex, epsilon: f64) {
        assert_float_eq(expected.re, actual.re, epsilon);
        assert_float_eq(expected.im, actual.im, epsilon);
    }

    #[test]
    fn fft_test() {
        let a = complex::Complex::new(1.0, 1.0);
        let b = complex::Complex::new(2.0, 2.0);
        let c = complex::Complex::new(3.0, 3.0);
        let d = complex::Complex::new(4.0, 4.0);
        let e = complex::Complex::new(5.0, 5.0);
        let f = complex::Complex::new(6.0, 6.0);
        let g = complex::Complex::new(7.0, 7.0);
        let h = complex::Complex::new(8.0, 8.0);

        let mut data = vec![a, b, c, d, e, f, g, h];

        fft(&mut data);

        assert_complex_eq(&complex::Complex::new(36.000000, 36.000000), &data[0], 1E-6);
        assert_complex_eq(&complex::Complex::new(-13.656854, 5.656854), &data[1], 1E-6);
        assert_complex_eq(&complex::Complex::new(-8.000000, 0.000000), &data[2], 1E-6);
        assert_complex_eq(&complex::Complex::new(-5.656854, -2.343146), &data[3], 1E-6);
        assert_complex_eq(&complex::Complex::new(-4.000000, -4.000000), &data[4], 1E-6);
        assert_complex_eq(&complex::Complex::new(-2.343146, -5.656854), &data[5], 1E-6);
        assert_complex_eq(&complex::Complex::new(0.000000, -8.000000), &data[6], 1E-6);
        assert_complex_eq(&complex::Complex::new(5.656854, -13.656854), &data[7], 1E-6);
    }

    #[test]
    fn dft_test() {
        let a = complex::Complex::new(1.0, 1.0);
        let b = complex::Complex::new(2.0, 2.0);
        let c = complex::Complex::new(3.0, 3.0);
        let d = complex::Complex::new(4.0, 4.0);
        let e = complex::Complex::new(5.0, 5.0);
        let f = complex::Complex::new(6.0, 6.0);
        let g = complex::Complex::new(7.0, 7.0);
        let h = complex::Complex::new(8.0, 8.0);

        let data = vec![a, b, c, d, e, f, g, h];

        let result = dft(data);

        assert_complex_eq(
            &complex::Complex::new(36.000000, 36.000000),
            &result[0],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(-13.656854, 5.656854),
            &result[1],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(-8.000000, 0.000000),
            &result[2],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(-5.656854, -2.343146),
            &result[3],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(-4.000000, -4.000000),
            &result[4],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(-2.343146, -5.656854),
            &result[5],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(0.000000, -8.000000),
            &result[6],
            1E-6,
        );
        assert_complex_eq(
            &complex::Complex::new(5.656854, -13.656854),
            &result[7],
            1E-6,
        );
    }
}
