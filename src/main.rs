use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{collections::HashMap, time::Duration};

mod function;
use function::Function;
mod surface;

pub fn main() {
    let mut variables = HashMap::<char, f64>::new();
    // variables.insert('r', 0.0);
    // variables.insert('t', 0.0);

    // let surface_1 = surface::Surface::<f64>::new(
    //     vec!['r', 't'],
    //     vec![|v| 400.0 + v[&'r'] * v[&'t'].cos(), |v| {
    //         300.0 + v[&'r'] * v[&'t'].sin()
    //     }],
    //     {
    //         let mut h = HashMap::new();
    //         h.insert('r', (10.0, 10.0));
    //         h.insert('t', (0.0, 2.0 * PI));
    //         h
    //     },
    //     [255, 0, 0],
    // )
    variables.insert('x', 0.0);
    let surface_1 = surface::Surface::<f64>::new(
        vec!['x'],
        vec![Function::Additive(
            Box::new(Function::Product(
                Box::new(Function::Constant(100.0)),
                Box::new(Function::Cosine(Box::new(Function::Product(
                    Box::new(Function::Variable('x')),
                    Box::new(Function::Constant(0.05)),
                )))),
            )),
            Box::new(Function::Constant(500.0)),
        )],
        None,
        [255, 0, 0],
    )
    .unwrap();
    let d = surface_1.derivative();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Parametrized Functions", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // let mut points = Vec::<(Point, (u8, u8, u8))>::new();
    let rate = 0.5;
    // let max_points: usize = 120;
    // let diff_rate: u8 = (255.0 / max_points as f64).floor() as u8;
    // println!("{}", diff_rate);

    'running: loop {
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.clear();

        // if let Some(p) = surface_1.at(&variables) {
        //     points.push((p, (255, 0, 0)));
        // }
        // let mut to_remove = Vec::<usize>::new();
        // for (i, point) in points.iter_mut().enumerate() {
        //     if point.1 .0 > diff_rate {
        //         point.1 .0 -= diff_rate;
        //     } else {
        //         to_remove.push(i);
        //     }
        // }
        // for index in to_remove {
        //     points.swap_remove(index);
        // }
        // for point in &points {
        //     canvas.set_draw_color(Color::RGB(point.1 .0, point.1 .1, point.1 .2));
        //     canvas
        //         .draw_point(sdl2::rect::Point::new(point.0[0] as i32, point.0[1] as i32))
        //         .unwrap();
        // }
        if let Some(p) = surface_1.at(&variables) {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas
                .draw_point(sdl2::rect::Point::new(variables[&'x'] as i32, p[0] as i32))
                .unwrap();
        }
        if let Some(p) = d.at(&variables) {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas
                .draw_point(sdl2::rect::Point::new(variables[&'x'] as i32, p[0] as i32))
                .unwrap();
        }
        variables.insert('x', variables[&'x'] + rate);
        // variables.insert('t', variables[&'t'] + rate);
        // variables.insert('r', 100.0 * (1.0 + variables[&'t'].sin()));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    // let surface_2;
}
