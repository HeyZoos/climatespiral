use extend::ext;
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
    let data = df.transpose().unwrap();

    Model {
        data,
        yearidx: 0,
        monthidx: 0,
        months,
    }
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    if model.yearidx < 143 {
        model.monthidx += 1;
        if model.monthidx == model.months.len() {
            model.yearidx += 1;
            model.monthidx = 0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let mut previous_point = None;
    let mut previous_value = None;

    for y in 0..model.yearidx {
        let mut total_months = model.months.len();
        if y == model.yearidx {
            total_months = model.monthidx;
        }

        for (i, value) in model.data[y].iter().enumerate().skip(1) {
            if let AnyValue::Float64(temperature) = value {
                if i < total_months {
                    // Map the index to an angle in radians
                    let mut angle = map_range(
                        i as f32,
                        1.0,
                        model.months.len() as f32 + 1.0,
                        0.0,
                        PI * 2.0,
                    );
                    // Rotate back by 90 degrees to put january at the top
                    angle += PI / 2.0;
                    // Map the temperature to a radius value
                    let temperature_radius = map_range(
                        temperature,
                        0.0,
                        1.0,
                        ZERO_DEGREES_RADIUS,
                        ONE_DEGREES_RADIUS,
                    );

                    let current_point = polarcoords(temperature_radius, angle);
                    if let Some(previous_point) = previous_point {
                        let average: f64 = (previous_value.unwrap() + temperature) / 2.0;

                        let cold = vec3(0.0, 0.0, 1.0);
                        let warm = vec3(1.0, 0.0, 0.0);
                        let zero = vec3(1.0, 1.0, 1.0);

                        let line_color = if average < 0.0 {
                            zero.lerp(cold, abs(average as f32))
                        } else {
                            zero.lerp(warm, abs(average as f32))
                        };

                        draw.line().start(previous_point).end(current_point).rgb(
                            line_color.x,
                            line_color.y,
                            line_color.z,
                        );
                    }

                    previous_point = Some(current_point);
                    previous_value = Some(temperature);
                }
            }
        }
    }

    for (i, month) in model.months.iter().enumerate() {
        // Convert the month index to an angle in radians
        let mut angle = map_range(i as f32, 0.0, model.months.len() as f32, 0.0, PI * 2.0);
        // Rotate back by 90 degrees to put january at the top
        angle += PI / 2.0;
        let xy = polarcoords(MONTH_LABELS_RADIUS, angle);
        draw.label(month.name(), xy, 30.0, 20.0);
    }

    // Display current year
    let year: f64 = model.data[model.yearidx]
        .get(0)
        .unwrap()
        .try_extract()
        .unwrap();
    draw.text(&year.to_string());

    // Render the degree rings
    draw.borderedring(ZERO_DEGREES_RADIUS);
    draw.borderedring(ONE_DEGREES_RADIUS);
    draw.label("0°", vec2(0.0, ZERO_DEGREES_RADIUS), 30.0, 20.0);
    draw.label("1°", vec2(0.0, ONE_DEGREES_RADIUS), 30.0, 20.0);

    draw.to_frame(&app, &frame).unwrap();
}

fn polarcoords(radius: f32, angle: f32) -> Vec2 {
    // Multiply by -1.0 to produce coordinates moving clockwise
    vec2(radius * angle.cos() * -1.0, radius * angle.sin())
}

struct Model {
    data: DataFrame,
    yearidx: usize,
    monthidx: usize,
    months: Vec<Field>,
}

const ZERO_DEGREES_RADIUS: f32 = 200.0;
const ONE_DEGREES_RADIUS: f32 = 350.0;
const MONTH_LABELS_RADIUS: f32 = 400.0;

#[ext]
impl Draw {
    fn ring(&self, radius: f32, color: Rgb8) {
        self.ellipse()
            .radius(radius)
            .no_fill()
            .stroke(color)
            .stroke_weight(2.0);
    }

    fn borderedring(&self, radius: f32) {
        self.ring(radius, YELLOW);
        self.ring(radius + 2.0, BLACK);
        self.ring(radius - 2.0, BLACK);
    }

    fn label(&self, s: &str, xy: Vec2, w: f32, h: f32) {
        self.rect().xy(xy).w(w).h(h).color(BLACK);
        self.text(s).xy(xy).center_justify().color(WHITE);
    }
}
