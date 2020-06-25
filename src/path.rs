use crate::complex;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Path {
    pub data: Vec<complex::Complex>,
}

impl Path {
    pub fn new(filename: &str) -> Result<Path, Box<dyn Error>> {
        let data = read_from_file(filename)?;
        Ok(Path { data })
    }
}

fn read_from_file(filename: &str) -> Result<Vec<complex::Complex>, Box<dyn Error>> {
    let file_content = fs::read_to_string(filename)?;

    let result = file_content
        .lines()
        .map(|line| {
            let parts = line.trim().split(",").collect::<Vec<&str>>();
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
        println!("{}", var);
        let path = Path::new(var).expect("Problem reading file");

        assert_eq!(10, path.data.len());
    }
}
