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

    pub fn phase(&self) -> angular::Angle {
        angular::atan(self.im / self.re)
    }

    pub fn amplitude(&self) -> f64 {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn phase_test() {
        let c = Complex::new(0.0, 1.0);
        assert_eq!(angular::Angle::<f64>::quarter(), c.phase());
    }

    #[test]
    fn amplitude_test() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(5.0, c.amplitude());
    }

    #[test]
    fn add_test() {
        let a = Complex::new(1.0, 2.2);
        let b = Complex::new(35.4, -54.8);
        let c = Complex::new(36.4, -52.6);
        assert_eq!(c, Complex::add(&a, &b));
    }

    #[test]
    fn minus_test() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(35.4, -54.8);
        let c = Complex::new(-34.4, 56.8);
        assert_eq!(c, Complex::minus(&a, &b));
    }

    #[test]
    fn multiply_test() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = Complex::new(-5.0, 10.0);
        assert_eq!(c, Complex::multiply(&a, &b));
    }
}
