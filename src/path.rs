use crate::complex;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct WrongPathLengthError{
    msg: String,
}

impl WrongPathLengthError{
    pub fn new(wrong_length: u64) -> WrongPathLengthError{
        let message = format!("Path length of {} is not a power of 2, add more data to input", wrong_length);
        WrongPathLengthError{ msg: message }
    }

}

impl fmt::Display for WrongPathLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for WrongPathLengthError {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug)]
pub struct Path {
    pub data: Vec<complex::Complex>,
}

impl Path {
    pub fn new(filename: &str) -> Result<Path, Box<dyn Error>> {
        let data = read_from_file(filename)?;
        let data_length: u64 = data.len() as u64;
        Path::assert_power_of_2_length(data_length)?; 
        let mut path = Path { data };
        path.normalize();
        Ok( path )
    }

    fn assert_power_of_2_length(length: u64) -> Result<(), WrongPathLengthError> {
        if !((length != 0) && ((length & (length - 1)) == 0)) {
            return Err(WrongPathLengthError::new(length));
        }
        Ok(())
    }

    fn normalize(&mut self) {
        let mut min_x = std::f64::MAX;
        let mut max_x = std::f64::MIN;
        let mut min_y = std::f64::MAX;
        let mut max_y = std::f64::MIN;

        for val in self.data.iter() {
            if val.re < min_x {
                min_x = val.re;
            }
            if val.re > max_x {
                max_x = val.re;
            }
            if val.im < min_y {
                min_y = val.im;
            }
            if val.im > max_y {
                max_y = val.im;
            }
        }

        let scaling_factor = (((max_x - min_x) * (max_x - min_x)) + ((max_y - min_y) * (max_y - min_y))).sqrt();
        for val in self.data.iter_mut() {
            val.re = (val.re - min_x) / scaling_factor;
            val.im = (val.im - min_y) / scaling_factor;
        }

    }
}

fn read_from_file(filename: &str) -> Result<Vec<complex::Complex>, Box<dyn Error>> {
    let file_content = fs::read_to_string(filename)?;

    let result = file_content
        .lines()
        .map(|line| {
            let parts = line.trim().split(',').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err("Wrong number of arguments in input lines");
            }
            Ok(complex::Complex::new(
                parts[0].trim().parse::<f64>().unwrap(),
                parts[1].trim().parse::<f64>().unwrap(),
            ))
        })
        .map(|val| val.unwrap())
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn read_from_file_test() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("resources");
        dir.push("test");
        let filename = "test_path_file.txt";
        dir.push(filename);
        let var = dir.to_str().unwrap();
        let path = Path::new(var).expect("Problem reading file");

        assert_eq!(8, path.data.len());
    }

    #[test]
    fn test_normalize() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("resources");
        dir.push("test");
        let filename = "test_path_file.txt";
        dir.push(filename);
        let var = dir.to_str().unwrap();
        let path = Path::new(var).expect("Problem reading file");
        let sqrt_2_over_2: f64 = (2.0_f64).sqrt() / 2.0;

        assert!((path.data[0].re - 2.0) < 1E-8);
        assert!((path.data[0].im - 1.0) < 1E-8);
        assert!((path.data[1].re - (sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[1].im - (sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[2].re - 1.0) < 1E-8);
        assert!((path.data[2].im - 2.0) < 1E-8);
        assert!((path.data[3].re - (-sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[3].im - (sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[4].re - 0.0) < 1E-8);
        assert!((path.data[4].im - 1.0) < 1E-8);
        assert!((path.data[5].re - (-sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[5].im - (-sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[6].re - 1.0) < 1E-8);
        assert!((path.data[6].im - 0.0) < 1E-8);
        assert!((path.data[7].re - (sqrt_2_over_2 + 1.0)) < 1E-8);
        assert!((path.data[7].im - (-sqrt_2_over_2 + 1.0)) < 1E-8);
    }
}
