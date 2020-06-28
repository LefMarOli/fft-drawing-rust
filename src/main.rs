use fft::epicycle::Epicycle;
use plotters::prelude::*;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("resources");
    dir.push("test");
    let filename = "test_path_file.txt";
    dir.push(filename);
    let var = dir.to_str().unwrap();

    let epicycle = Epicycle::from_file(var).unwrap();
    let mut results = vec![];
    let mut dt = 0.0;
    while dt < 2.0 * std::f64::consts::PI {
        let coord = epicycle.get_coordinate_for(dt, 8).unwrap();
        results.push((coord.x as f32, coord.y as f32));
        //println!("X: {}, Y: {}", coord.x, coord.y);
        dt += 0.001;
    }

    let root = BitMapBackend::new("images/test.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(20)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_ranged(0f32..6f32, 0f32..6f32)?;

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

    Ok(())
}
