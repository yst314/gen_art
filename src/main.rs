use crate::algos::euclidean_gcd;
use algos::div_rect;
use nannou::prelude::*;

mod algos;

fn main() {
    let ret = euclidean_gcd(14803, 12707);
    println!("{}", ret);
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model;
fn model(_app: &App) -> Model {
    Model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    let rects = div_rect(9, 4, 10);
    println!("rects");
    let stroke_weight = 2.0;
    for r in rects.iter() {
        let a = r.quad_points();
        draw.quad()
            .points(a[0], a[1], a[2], a[3])
            .stroke_weight(stroke_weight)
            .stroke_color(BLACK);
        println!("{}, {}, {}, {}", r.x, r.y, r.w, r.h);
    }

    draw.to_frame(app, &frame).unwrap();
}
