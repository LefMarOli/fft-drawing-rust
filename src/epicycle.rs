use std::error::Error;
use std::fmt;
use crate::complex;

pub struct Epicycle{
    data: Vec<(complex::Complex, u64)>,
}

pub struct Coordinate{
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct InvalidPrecisionError{
    msg: String,
}

impl InvalidPrecisionError{
    fn new(requested_precision: usize, max_precision: usize) -> InvalidPrecisionError{
        InvalidPrecisionError{ msg: format!("{}th precision is not possible, can only compute up to {} epicycle precision", requested_precision, max_precision)}
    }
}

impl fmt::Display for InvalidPrecisionError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for InvalidPrecisionError {
    fn description(&self) -> &str {
        &self.msg
    }
}


impl Epicycle{
    pub fn new(input: Vec<complex::Complex>) -> Epicycle{
        let size: u64 = input.len() as u64;
        let v : Vec<(complex::Complex, u64)> = input.into_iter().zip(0..size).collect();
        let mut epicycle = Epicycle{ data: v};
        epicycle.data.sort_by(|(a, _) , (b, _)| b.amplitude().partial_cmp(&a.amplitude()).unwrap());
        epicycle
    }

    pub fn get_coordinate_for(&self, time: f64, precision: u32) -> Result<Coordinate, InvalidPrecisionError>{
        let nth = precision as usize;
        if nth > self.data.len() {
            return Err(InvalidPrecisionError::new(nth, self.data.len()));
        }

        let scaled_time = time / (std::f64::consts::PI * 2.0);
        let mut x_coord = 0.0;
        let mut y_coord = 0.0;
        for i in 0..nth {
            let radius = self.data[i].0.amplitude();
            let phase = self.data[i].0.phase().in_radians();
            let frequency = self.data[i].1 as f64;
            x_coord += radius * (frequency * scaled_time + phase).cos();
            y_coord += radius * (frequency * scaled_time + phase).sin();
        }
        Ok(Coordinate{ x: x_coord.round() as i64, y: y_coord.round() as i64 })
    }
}

#[cfg(test)]
mod tests{

    use super::*;
    
    #[test]
    fn get_coordinate_test(){
        let mut data = vec![];
        data.push(complex::Complex::new(1.0, 1.0));
        data.push(complex::Complex::new(3.0, 4.0));
        data.push(complex::Complex::new(5.0, 6.0));

        let epicycle = Epicycle::new(data);

        let mut coord = epicycle.get_coordinate_for(std::f64::consts::PI, 1).unwrap();

        assert_eq!(-2, coord.x);
        assert_eq!(7, coord.y);

        coord = epicycle.get_coordinate_for(std::f64::consts::PI, 2).unwrap();
        assert_eq!(-2, coord.x);
        assert_eq!(12, coord.y);

        coord = epicycle.get_coordinate_for(std::f64::consts::PI, 3).unwrap();
        assert_eq!(-1, coord.x);
        assert_eq!(13, coord.y);
    }
}
