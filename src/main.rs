mod double_pendulum;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn draw_circle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cx: i32,
    cy: i32,
    radius: i32,
) -> Result<(), String> {
    let mut x = radius;
    let mut y = 0;
    let mut error = 1 - radius;

    canvas.set_draw_color(Color::RGB(240, 180, 60));

    while x >= y {
        canvas.draw_line((cx - x, cy + y), (cx + x, cy + y))?;
        canvas.draw_line((cx - x, cy - y), (cx + x, cy - y))?;
        canvas.draw_line((cx - y, cy + x), (cx + y, cy + x))?;
        canvas.draw_line((cx - y, cy - x), (cx + y, cy - x))?;

        y += 1;

        if error < 0 {
            error += 2 * y + 1;
        } else {
            x -= 1;
            error += 2 * (y - x) + 1;
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("Test", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "macos"))]
    {
        let idx = window.display_index()?;
        let (_ddpi, hdpi, _vdpi) = video_subsystem.display_dpi(idx)?;
        let k = hdpi / 96.0;
        window
            .set_size(
                ((SCREEN_WIDTH as f32) * k) as u32,
                ((SCREEN_HEIGHT as f32) * k) as u32,
            )
            .map_err(|e| e.to_string())?;
        window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );
    }

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas
        .set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut last_frame = Instant::now();

    let l1: f64 = 1.5;
    let l2: f64 = 1.5;

    let m1: f64 = 1.0;
    let m2: f64 = 1.0;

    let mut theta1: f64 = 1.4;
    let mut theta2: f64 = 1.4;

    let mut omega1: f64 = 0.0;
    let mut omega2: f64 = 0.0;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f64();
        last_frame = now;

        double_pendulum::step(
            l1,
            m1,
            &mut theta1,
            &mut omega1,
            l2,
            m2,
            &mut theta2,
            &mut omega2,
            dt,
        );

        let x0 = (SCREEN_WIDTH / 2) as f64;
        let y0 = 100.0;

        let x1 = x0 + l1 * theta1.sin() * 100.0;
        let y1 = y0 + l1 * theta1.cos() * 100.0;

        let x2 = x1 + l2 * theta2.sin() * 100.0;
        let y2 = y1 + l2 * theta2.cos() * 100.0;

        canvas.set_draw_color(Color::RGB(20, 20, 30));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(180, 180, 190));

        canvas.draw_line((x0 as i32, y0 as i32), (x1 as i32, y1 as i32))?;

        canvas.draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))?;

        draw_circle(&mut canvas, x0 as i32, y0 as i32, 6)?;
        draw_circle(&mut canvas, x1 as i32, y1 as i32, 12)?;
        draw_circle(&mut canvas, x2 as i32, y2 as i32, 12)?;

        canvas.present();
    }

    Ok(())
}
