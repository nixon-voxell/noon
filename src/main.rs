// #![allow(unused)]

mod animation;
mod app;
mod component;
mod consts;
mod ease;
mod object;
mod path;
mod scene;
mod system;

pub use crate::animation::{
    AnimBuilder, Animation, AnimationType, Animations, EntityAnimations, WithAngle, WithColor,
    WithFill, WithFontSize, WithId, WithPath, WithPosition, WithSize, WithStroke,
};

pub use crate::component::{
    Angle, Color, ColorExtension, FillColor, FontSize, Interpolate, Name, Opacity, PathCompletion,
    Point, Position, Size, StrokeColor, Value,
};

pub use crate::path::{GetPartial, Path, PathComponent};
pub use consts::*;
pub use ease::EaseType;
use nannou::{
    color::{rgb_u32, rgba, Rgb},
    rand::{prelude::SliceRandom, random_range, thread_rng},
};
pub use object::*;
pub use scene::{Bounds, Construct, Scene};
pub use system::{animate, animate_from_target, animate_position, print, update_time, Time};

// impl Construct for Scene {
//     fn construct(&mut self) {
//         let mut animations = Vec::new();
//         let mut show = Vec::new();
//         for _ in (0..100) {
//             let (x, y, w, h, ang, color) = gen_random_values();

//             if nannou::rand::random::<bool>() {
//                 let circle = self
//                     .circle()
//                     .with_position(x, y)
//                     .with_color(color)
//                     .with_radius(w / 2.0)
//                     .make();

//                 let (x, y, w, h, ang, color) = gen_random_values();

//                 show.push(circle.show_creation());

//                 animations.extend(vec![
//                     circle.set_color(color),
//                     circle.move_to(x, y),
//                     circle.set_radius(w / 2.0),
//                 ]);
//             } else {
//                 let rect = self
//                     .rectangle()
//                     .with_position(x, y)
//                     .with_color(color)
//                     .with_size(w, h)
//                     .make();

//                 let (x, y, w, h, ang, color) = gen_random_values();

//                 show.push(rect.show_creation());

//                 animations.extend(vec![
//                     rect.set_color(color),
//                     rect.move_to(x, y),
//                     rect.set_size(w, h),
//                     rect.set_angle(ang),
//                 ]);
//             }
//         }

//         self.wait();
//         self.play(show).run_time(1.0).lag(0.001);

//         self.play(animations)
//             .run_time(3.0)
//             .lag(0.0001)
//             .rate_func(EaseType::Quint);
//     }
// }

// impl Construct for Scene {
//     fn construct(&mut self) {
//         let mut animations = Vec::new();
//         let mut show = Vec::new();
//         for _ in 0..200 {
//             let (x, y, w, _, _, color) = gen_random_values();

//             let circle = self
//                 .circle()
//                 .with_position(x, y)
//                 .with_color(color)
//                 .with_radius(2.0 * w / 2.0)
//                 .make();

//             show.push(circle.show_creation());

//             let (x, y, w, h, _, color) = gen_random_values();

//             let rect = self
//                 .rectangle()
//                 .with_position(x, y)
//                 .with_color(color)
//                 .with_size(2.0 * w, 2.0 * h)
//                 .make();

//             show.push(rect.show_creation());

//             // animations.push(circle.morph(rect));
//             animations.push(rect.morph(circle));

//             let (x, y, _, _, _, color) = gen_random_values();
//             let line = self.line().from(0.0, 0.0).to(x, y).with_color(color).make();
//             show.push(line.show_creation());
//         }

//         self.wait();
//         self.play(show).run_time(1.0).lag(0.001);

//         self.play(animations)
//             .run_time(10.0)
//             .lag(0.0001)
//             .rate_func(EaseType::Quint);
//     }
// }

impl Construct for Scene {
    fn construct(&mut self) {
        let (x, y, _w, _h, _ang, color) = gen_random_values();

        let circle = self
            .circle()
            .with_position(x, y)
            .with_color(color)
            .with_radius(200.0 / 2.0)
            .make();

        let (_x, _y, _w, _h, _ang, color) = gen_random_values();

        let text = self
            .text()
            .with_text("Hello World!")
            .with_color(color)
            .with_position(-300.0, 100.0)
            .make();

        let (x, y, _w, _h, _ang, color) = gen_random_values();

        let rect = self
            .rectangle()
            .with_position(x, y)
            .with_color(color)
            .with_size(150.0, 150.0)
            .make();

        let line = self
            .line()
            .with_color(color)
            .from(-400.0, -400.0)
            .to(400.0, 400.0)
            .make();

        self.wait();
        // self.play(vec![circle.move_to(400.0, 400.0), circle.fade_in()]);
        self.play(vec![
            rect.show_creation(),
            line.show_creation(),
            text.show_creation(),
        ]);

        // self.play(line.morph(circle)).run_time(3.0);
        // self.play(circle.morph(rect)).run_time(3.0);
        // self.play(text.morph(rect)).run_time(10.0);
        self.play(rect.morph(text)).run_time(20.0);

        // self.play(vec![
        //     circle.move_to_object(rect),
        //     circle.set_color_from(rect),
        // ])
        // .rate_func(EaseType::Quint)
        // .run_time(2.0);

        // self.wait();
        // self.play(circle.move_to(400.0, 400.0))
        //     .rate_func(EaseType::Elastic);
        // self.play(circle.move_to(400.0, 400.0))
        //     .run_time(1.0)
        //     .lag(0.0001)
        //     .rate_func(EaseType::Quad);
    }
}

fn main() {
    app::run();
}

fn gen_random_values() -> (f32, f32, f32, f32, f32, Color) {
    let colors = [
        rgb_from_hex(0x264653),
        rgb_from_hex(0x2a9d8f),
        rgb_from_hex(0xe9c46a),
        rgb_from_hex(0xf4a261),
        rgb_from_hex(0xe76f51),
    ];
    let mut rng = thread_rng();

    let x_lim = 1920.0 / 2.0;
    let y_lim = 1080.0 / 2.0;

    let x = random_range::<f32>(-x_lim, x_lim);
    let y = random_range::<f32>(-y_lim, y_lim);
    let w = random_range::<f32>(4.0, 60.0);
    let h = random_range::<f32>(4.0, 60.0);
    let ang = random_range::<f32>(0.0, 360.0);
    let color = *colors.choose(&mut rng).unwrap();

    (x, y, w, h, ang, color)
}

fn rgb_from_hex(color: u32) -> Rgb {
    let color = rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}
