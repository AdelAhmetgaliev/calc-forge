use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use plotly::{
    ImageFormat, Layout, Plot, Scatter,
    color::NamedColor,
    common::{Line, Marker, Mode},
    layout::{Axis, Legend, TicksDirection, themes::BuiltinTheme},
};

use interpo::{lagrange_interp, select_nearest_points};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "data/input_data.dat";
    let data_points = read_data_from_file(filename)?;

    let interp_x: f64 = 0.2;
    let nearest_points = select_nearest_points(&data_points, 3, interp_x);
    let interp_y: f64 = lagrange_interp(&nearest_points, interp_x)?;

    let (x_values, y_values): (Vec<f64>, Vec<f64>) = data_points.iter().cloned().unzip();
    let y_values_interp: Vec<f64> = x_values
        .iter()
        .map(|&x| lagrange_interp(&nearest_points, x).unwrap())
        .collect();

    let mae_value = compute_mae(&y_values, &y_values_interp);

    let scatter_input_data = create_scatter_trace(
        x_values.clone(),
        y_values,
        NamedColor::Blue,
        "Входные данные",
    );
    let scatter_interp_data = create_scatter_trace(
        x_values.clone(),
        y_values_interp,
        NamedColor::Green,
        "Многочлен Лагранжа 3 степени",
    );
    let scatter_interp_point = create_scatter_point(
        interp_x,
        interp_y,
        NamedColor::Red,
        "Интерполированная точка",
    );
    let text_trace = create_text_trace(format!("MAE: {:.1}", mae_value));

    let layout = create_layout();
    let mut plot = Plot::new();
    plot.add_trace(scatter_input_data);
    plot.add_trace(scatter_interp_data);
    plot.add_trace(scatter_interp_point);
    plot.add_trace(text_trace);
    plot.set_layout(layout);

    plot.write_image("graph", ImageFormat::PNG, 1280, 720, 2.0)?;

    Ok(())
}

fn read_data_from_file(filename: &str) -> Result<Vec<(f64, f64)>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data: Vec<(f64, f64)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((x, y)) = parse_line(&line) {
            data.push((x, y));
        }
    }

    Ok(data)
}

fn parse_line(line: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() == 2
        && let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse())
    {
        return Some((x, y));
    }
    None
}

fn compute_mae(y_values: &[f64], y_values_interp: &[f64]) -> f64 {
    y_values
        .iter()
        .zip(y_values_interp)
        .map(|(&val, &interp)| f64::abs(val - interp))
        .sum::<f64>()
        / y_values.len() as f64
}

fn create_scatter_trace(
    x: Vec<f64>,
    y: Vec<f64>,
    color: NamedColor,
    name: &str,
) -> Box<Scatter<f64, f64>> {
    Scatter::new(x, y)
        .mode(Mode::Lines)
        .name(name)
        .line(Line::new().width(2.0).color(color))
}

fn create_scatter_point(x: f64, y: f64, color: NamedColor, name: &str) -> Box<Scatter<f64, f64>> {
    Scatter::new(vec![x], vec![y])
        .mode(Mode::Markers)
        .name(name)
        .marker(Marker::new().size(10).color(color))
}

fn create_text_trace(label: String) -> Box<Scatter<f64, f64>> {
    Scatter::new(vec![0.0], vec![0.0])
        .name(label)
        .mode(Mode::Lines)
        .line(Line::new().width(0.0).color(NamedColor::Transparent))
}

fn create_layout() -> Layout {
    let theme = BuiltinTheme::Matplotlib;
    Layout::new()
        .template(theme.build())
        .title("Интерполяция методом Лагранжа")
        .x_axis(
            Axis::new()
                .title("Ось X")
                .ticks(TicksDirection::Inside)
                .show_grid(true)
                .grid_color(NamedColor::LightGray),
        )
        .y_axis(
            Axis::new()
                .title("Ось Y")
                .ticks(TicksDirection::Inside)
                .show_grid(true)
                .grid_color(NamedColor::LightGray),
        )
        .show_legend(true)
        .legend(Legend::new())
}
