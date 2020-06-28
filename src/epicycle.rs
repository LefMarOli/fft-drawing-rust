use crate::complex;
use crate::fft;
use crate::path;
use std::error::Error;
use std::fmt;

pub struct Epicycle {
    pub data: Vec<(complex::Complex, u64)>,
}

pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct InvalidPrecisionError {
    msg: String,
}

impl InvalidPrecisionError {
    fn new(requested_precision: usize, max_precision: usize) -> InvalidPrecisionError {
        InvalidPrecisionError {
            msg: format!(
                "{}th precision is not possible, can only compute up to {} epicycle precision",
                requested_precision, max_precision
            ),
        }
    }
}

impl fmt::Display for InvalidPrecisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for InvalidPrecisionError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl Epicycle {
    pub fn from_file(filename: &str) -> Result<Epicycle, Box<dyn Error>> {
        let input_path = path::Path::new(filename)?;
        Ok(Epicycle::from_path(input_path))
    }

    pub fn from_path(mut input_path: path::Path) -> Epicycle {
        fft::fft(&mut input_path.data);
        Epicycle::new(input_path.data)
    }

    pub fn new(input: Vec<complex::Complex>) -> Epicycle {
        let size: u64 = input.len() as u64;
        let v: Vec<(complex::Complex, u64)> = input.into_iter().zip(0..size).collect();
        let mut epicycle = Epicycle { data: v };
        epicycle
            .data
            .sort_by(|(a, _), (b, _)| b.amplitude().partial_cmp(&a.amplitude()).unwrap());
        epicycle
    }

    pub fn get_coordinate_for(
        &self,
        time: f64,
        precision: u32,
    ) -> Result<Coordinate, InvalidPrecisionError> {
        let nth = precision as usize;
        if nth > self.data.len() {
            return Err(InvalidPrecisionError::new(nth, self.data.len()));
        }

        let mut x_coord = 0.0;
        let mut y_coord = 0.0;
        for i in 0..nth {
            let radius = self.data[i].0.amplitude();
            if radius < 1E-9 {
                break;
            }
            let phase = self.data[i].0.phase().in_radians();
            let frequency = self.data[i].1 as f64;
            x_coord += radius * (frequency * time + phase).cos();
            y_coord += radius * (frequency * time + phase).sin();
        }
        Ok(Coordinate {
            x: x_coord,
            y: y_coord,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use plotters::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn get_coordinate_test() {
        let mut data = vec![];
        data.push(complex::Complex::new(1.0, 1.0));
        data.push(complex::Complex::new(3.0, 4.0));
        data.push(complex::Complex::new(5.0, 6.0));

        let epicycle = Epicycle::new(data);

        let mut coord = epicycle
            .get_coordinate_for(std::f64::consts::PI, 1)
            .unwrap();

        assert_eq!(-2, coord.x);
        assert_eq!(7, coord.y);

        coord = epicycle
            .get_coordinate_for(std::f64::consts::PI, 2)
            .unwrap();
        assert_eq!(-2, coord.x);
        assert_eq!(12, coord.y);

        coord = epicycle
            .get_coordinate_for(std::f64::consts::PI, 3)
            .unwrap();
        assert_eq!(-1, coord.x);
        assert_eq!(13, coord.y);
    }

    #[test]
    fn test_epicycle_from_file() -> Result<(), Box<dyn Error>> {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("resources");
        dir.push("test");
        let filename = "test_path_file.txt";
        dir.push(filename);
        let var = dir.to_str().unwrap();

        let epicycle = Epicycle::from_file(var).unwrap();
        for (c, i) in epicycle.data.iter() {
            println!("Re: {}", c.re);
            println!("Im: {}", c.im);
            println!("Freq: {}", i);
            println!("Amplitude: {}", c.amplitude());
            println!("Phase: {}", c.phase());
            println!("------------------------");
        }

        let mut results = vec![];

        let mut dt = 0.0;
        while dt < 2.0 * std::f64::consts::PI {
            let coord = epicycle.get_coordinate_for(dt, 5).unwrap();
            results.push((coord.x as f32, coord.y as f32));
            dt += 0.001;
        }

        let root = BitMapBackend::new("plotters-doc-data/5.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE);
        let root = root.margin(10, 10, 10, 10);
        // After this point, we should be able to draw construct a chart context
        let mut chart = ChartBuilder::on(&root)
            // Set the caption of the chart
            .caption("This is our first plot", ("sans-serif", 40).into_font())
            // Set the size of the label region
            .x_label_area_size(20)
            .y_label_area_size(40)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_ranged(0f32..3f32, 0f32..3f32)?;

        // Then we can draw a mesh
        chart
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            // We can also change the format of the label text
            .y_label_formatter(&|x| format!("{:.3}", x))
            .draw()?;

        // And we can draw something in the drawing area
        chart.draw_series(LineSeries::new(results, &RED))?;

        // Similarly, we can draw point series
        // chart.draw_series(PointSeries::of_element(
        //     results,
        //     5,
        //     &RED,
        //     &|c, s, st| {
        //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
        //     + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
        //     + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        //     },
        // ))?;

        root.present();
        Ok(())
    }
}
