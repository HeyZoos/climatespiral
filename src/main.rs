use nannou::prelude::*;
use polars::prelude::*;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let df = CsvReader::from_path("data/giss.csv")
        .unwrap()
        .with_skip_rows(1)
        .with_null_values(Some(NullValues::AllColumnsSingle("***".to_string())))
        .finish()
        .unwrap();

    Model { df, yearidx: 0 }
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    model.yearidx += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let months = model.df.fields()[1..=12].to_vec();

    let draw = app.draw();

    draw.background().color(BLACK);

    // Render the degree rings
    draw.ellipse()
        .w(ZERO_DEGREES_RADIUS * 2.0)
        .h(ZERO_DEGREES_RADIUS * 2.0)
        .no_fill()
        .stroke_weight(2.0)
        .stroke(WHITE);
    draw.ellipse()
        .w(ONE_DEGREES_RADIUS * 2.0)
        .h(ONE_DEGREES_RADIUS * 2.0)
        .no_fill()
        .stroke_weight(2.0)
        .stroke(WHITE);

    for (i, month) in months.iter().enumerate() {
        // Convert the month index to an angle in radians
        let mut angle = map_range(i as f32, 0.0, months.len() as f32, 0.0, PI * 2.0);
        // Rotate back by 90 degrees to put january at the top
        angle += PI / 2.0;
        draw.text(month.name()).xy(polarcoords(250.0, angle));
    }

    let data = model.df.transpose().unwrap();

    let mut points = vec![];
    for y in 0..=model.yearidx {
        for (i, value) in data[y].iter().enumerate() {
            match value {
                AnyValue::Float64(value) => {
                    if i == 0 {
                        // Draw the year label
                        draw.text(&value.to_string());
                    } else {
                        // Map the index to an angle in radians
                        let mut angle =
                            map_range(i as f32, 1.0, months.len() as f32 + 1.0, 0.0, PI * 2.0);
                        // Rotate back by 90 degrees to put january at the top
                        angle += PI / 2.0;
                        // Map the temperature to a radius value
                        let temperature_radius =
                            map_range(value, 0.0, 1.0, ZERO_DEGREES_RADIUS, ONE_DEGREES_RADIUS);
                        // Draw the temperature value
                        points.push((polarcoords(temperature_radius, angle), WHITE));
                    }
                }
                _ => {}
            }
            draw.polyline().points_colored(points.clone());
        }
    }

    draw.to_frame(&app, &frame).unwrap();
}

fn polarcoords(radius: f32, angle: f32) -> Vec2 {
    // Multiply by -1.0 to produce coordinates moving clockwise
    vec2(radius * angle.cos() * -1.0, radius * angle.sin())
}

struct Model {
    df: DataFrame,
    yearidx: usize,
}

const ZERO_DEGREES_RADIUS: f32 = 75.0;
const ONE_DEGREES_RADIUS: f32 = 150.0;
