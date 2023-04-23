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

    let months = df.fields()[1..=12].to_vec();

    Model { months }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for (i, month) in model.months.iter().enumerate() {
        let angle = map_range(i as f32, 0.0, model.months.len() as f32, 0.0, PI * 2.0);
        draw.text(month.name()).xy(polarcoords(250.0, angle));
    }

    draw.to_frame(&app, &frame).unwrap();
}

fn polarcoords(radius: f32, angle: f32) -> Vec2 {
    vec2(radius * angle.cos(), radius * angle.sin())
}

struct Model {
    months: Vec<Field>,
}
