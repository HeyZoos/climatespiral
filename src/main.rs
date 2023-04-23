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

    Model { df }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let months = model.df.fields()[1..=12].to_vec();

    let draw = app.draw();

    draw.background().color(BLACK);

    for (i, month) in months.iter().enumerate() {
        // Convert the month index to an angle in radians
        let mut angle = map_range(i as f32, 0.0, months.len() as f32, 0.0, PI * 2.0);
        // Rotate back by 90 degrees to put january at the top
        angle += PI / 2.0;
        draw.text(month.name()).xy(polarcoords(250.0, angle));
    }

    let data = model.df.transpose().unwrap();

    for row in data.iter() {
        for (i, value) in row.iter().enumerate() {
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
                        // Draw the temperature value
                        draw.ellipse().w(10.0).h(10.0).xy(polarcoords(50.0, angle));
                    }
                }
                _ => {}
            }
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
}
