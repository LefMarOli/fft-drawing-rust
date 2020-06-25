mod fft {

    #[derive(Debug)]
    pub struct Complex {
        pub re: f64,
        pub im: f64,
    }

    use float_cmp::approx_eq;

    impl std::cmp::PartialEq for Complex {
        fn eq(&self, other: &Self) -> bool {
            approx_eq!(f64, self.re, other.re, epsilon = 1E-8)
                && approx_eq!(f64, self.im, other.im, epsilon = 1E-8)
        }
    }

    impl Complex {
        pub fn new(re: f64, im: f64) -> Complex {
            Complex { re, im }
        }

        pub fn phase(self) -> angular::Angle {
            angular::atan(self.im / self.re)
        }

        pub fn amplitude(self) -> f64 {
            ((self.re * self.re) + (self.im * self.im)).sqrt()
        }

        pub fn add(first: &Complex, second: &Complex) -> Complex {
            Complex::new(first.re + second.re, first.im + second.im)
        }

        pub fn minus(from: &Complex, with: &Complex) -> Complex {
            Complex::new(from.re - with.re, from.im - with.im)
        }

        pub fn multiply(first: &Complex, second: &Complex) -> Complex {
            let new_re = (first.re * second.re) - (first.im * second.im);
            let new_im = (first.re * second.im) + (first.im * second.re);
            Complex::new(new_re, new_im)
        }
    }

    pub fn butterfly(data: &mut Vec<Complex>) {
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

    pub fn fft(data: &mut Vec<Complex>) {
        butterfly(data);
        let mut step = 1;
        let length = data.len();
        while step < length {
            let jump = step << 1;

            let delta = -1.0 * std::f64::consts::PI / step as f64;

            let temp_sin = (delta * 0.5).sin();

            let factor_multiplier = Complex::new(-2.0 * temp_sin * temp_sin, delta.sin());
            let mut factor = Complex::new(1.0, 0.0);

            for group in 0..step {
                for pair in (group..data.len()).step_by(jump) {
                    let matched = pair + step;
                    let product = Complex::multiply(&factor, &data[matched]);
                    data[matched] = Complex::minus(&data[pair], &product);
                    data[pair] = Complex::add(&data[pair], &product);
                }
                factor = Complex::add(&Complex::multiply(&factor_multiplier, &factor), &factor);
            }

            step <<= 1;
        }
    }

    pub fn dft(data: Vec<Complex>) -> Vec<Complex> {
        let mut results: Vec<Complex> = Vec::new();

        for term in 0..data.len() {
            let mut sum = Complex::new(0.0, 0.0);
            for n in 0..data.len() {
                let angle =
                    std::f64::consts::PI * 2.0 * (term as f64) * (n as f64) / (data.len() as f64);
                let exp = Complex::new(angle.cos(), -1.0 * angle.sin());
                let mult = Complex::multiply(&data[n], &exp);
                sum = Complex::add(&sum, &mult);
            }
            results.push(sum);
        }
        results
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn phase_test() {
        let c = fft::Complex::new(0.0, 1.0);
        assert_eq!(angular::Angle::<f64>::quarter(), c.phase());
    }

    #[test]
    fn amplitude_test() {
        let c = fft::Complex::new(3.0, 4.0);
        assert_eq!(5.0, c.amplitude());
    }

    #[test]
    fn add_test() {
        let a = fft::Complex::new(1.0, 2.2);
        let b = fft::Complex::new(35.4, -54.8);
        let c = fft::Complex::new(36.4, -52.6);
        assert_eq!(c, fft::Complex::add(&a, &b));
    }

    #[test]
    fn minus_test() {
        let a = fft::Complex::new(1.0, 2.0);
        let b = fft::Complex::new(35.4, -54.8);
        let c = fft::Complex::new(-34.4, 56.8);
        assert_eq!(c, fft::Complex::minus(&a, &b));
    }

    #[test]
    fn multiply_test() {
        let a = fft::Complex::new(1.0, 2.0);
        let b = fft::Complex::new(3.0, 4.0);
        let c = fft::Complex::new(-5.0, 10.0);
        assert_eq!(c, fft::Complex::multiply(&a, &b));
    }

    #[test]
    fn butterfly_test() {
        let a = fft::Complex::new(1.0, 2.0);
        let b = fft::Complex::new(3.0, 4.0);
        let c = fft::Complex::new(-5.0, 10.0);
        let d = fft::Complex::new(4.5, -67.4);
        let e = fft::Complex::new(45.5, 98.9);
        let f = fft::Complex::new(32.4, -0.87);
        let g = fft::Complex::new(3.72, 87.0);
        let h = fft::Complex::new(2.65, -7.0);

        let mut data = vec![a, b, c, d, e, f, g, h];
        fft::butterfly(&mut data);
        assert_eq!(fft::Complex::new(1.0, 2.0), data[0]);
        assert_eq!(fft::Complex::new(3.0, 4.0), data[4]);
        assert_eq!(fft::Complex::new(-5.0, 10.0), data[2]);
        assert_eq!(fft::Complex::new(4.5, -67.4), data[6]);
        assert_eq!(fft::Complex::new(45.5, 98.9), data[1]);
        assert_eq!(fft::Complex::new(32.4, -0.87), data[5]);
        assert_eq!(fft::Complex::new(3.72, 87.0), data[3]);
        assert_eq!(fft::Complex::new(2.65, -7.0), data[7]);
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

    pub fn assert_complex_eq(expected: &fft::Complex, actual: &fft::Complex, epsilon: f64) {
        assert_float_eq(expected.re, actual.re, epsilon);
        assert_float_eq(expected.im, actual.im, epsilon);
    }

    #[test]
    fn fft_test() {
        let a = fft::Complex::new(1.0, 1.0);
        let b = fft::Complex::new(2.0, 2.0);
        let c = fft::Complex::new(3.0, 3.0);
        let d = fft::Complex::new(4.0, 4.0);
        let e = fft::Complex::new(5.0, 5.0);
        let f = fft::Complex::new(6.0, 6.0);
        let g = fft::Complex::new(7.0, 7.0);
        let h = fft::Complex::new(8.0, 8.0);

        let mut data = vec![a, b, c, d, e, f, g, h];

        fft::fft(&mut data);

        assert_complex_eq(&fft::Complex::new(36.000000, 36.000000), &data[0], 1E-6);
        assert_complex_eq(&fft::Complex::new(-13.656854, 5.656854), &data[1], 1E-6);
        assert_complex_eq(&fft::Complex::new(-8.000000, 0.000000), &data[2], 1E-6);
        assert_complex_eq(&fft::Complex::new(-5.656854, -2.343146), &data[3], 1E-6);
        assert_complex_eq(&fft::Complex::new(-4.000000, -4.000000), &data[4], 1E-6);
        assert_complex_eq(&fft::Complex::new(-2.343146, -5.656854), &data[5], 1E-6);
        assert_complex_eq(&fft::Complex::new(0.000000, -8.000000), &data[6], 1E-6);
        assert_complex_eq(&fft::Complex::new(5.656854, -13.656854), &data[7], 1E-6);
    }

    #[test]
    fn dft_test() {
        let a = fft::Complex::new(1.0, 1.0);
        let b = fft::Complex::new(2.0, 2.0);
        let c = fft::Complex::new(3.0, 3.0);
        let d = fft::Complex::new(4.0, 4.0);
        let e = fft::Complex::new(5.0, 5.0);
        let f = fft::Complex::new(6.0, 6.0);
        let g = fft::Complex::new(7.0, 7.0);
        let h = fft::Complex::new(8.0, 8.0);

        let data = vec![a, b, c, d, e, f, g, h];

        let result = fft::dft(data);

        assert_complex_eq(&fft::Complex::new(36.000000, 36.000000), &result[0], 1E-6);
        assert_complex_eq(&fft::Complex::new(-13.656854, 5.656854), &result[1], 1E-6);
        assert_complex_eq(&fft::Complex::new(-8.000000, 0.000000), &result[2], 1E-6);
        assert_complex_eq(&fft::Complex::new(-5.656854, -2.343146), &result[3], 1E-6);
        assert_complex_eq(&fft::Complex::new(-4.000000, -4.000000), &result[4], 1E-6);
        assert_complex_eq(&fft::Complex::new(-2.343146, -5.656854), &result[5], 1E-6);
        assert_complex_eq(&fft::Complex::new(0.000000, -8.000000), &result[6], 1E-6);
        assert_complex_eq(&fft::Complex::new(5.656854, -13.656854), &result[7], 1E-6);
    }
}
