mod double_pendulum;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn draw_disc(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
) -> Result<(), String> {
    let mut x = radius;
    let mut y = 0;
    let mut error = 1 - radius;

    canvas.set_draw_color(color);

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

fn draw_circle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
) -> Result<(), String> {
    // A translucent larger disc smooths the edge when alpha blending is enabled.
    draw_disc(canvas, cx, cy, radius + 1, Color::RGBA(color.r, color.g, color.b, 70))?;
    draw_disc(canvas, cx, cy, radius, color)
}

fn glyph(ch: char) -> [&'static str; 7] {
    match ch {
        'A' => ["01110", "10001", "10001", "11111", "10001", "10001", "10001"],
        'B' => ["11110", "10001", "10001", "11110", "10001", "10001", "11110"],
        'C' => ["01111", "10000", "10000", "10000", "10000", "10000", "01111"],
        'D' => ["11110", "10001", "10001", "10001", "10001", "10001", "11110"],
        'E' => ["11111", "10000", "10000", "11110", "10000", "10000", "11111"],
        'F' => ["11111", "10000", "10000", "11110", "10000", "10000", "10000"],
        'G' => ["01111", "10000", "10000", "10111", "10001", "10001", "01111"],
        'H' => ["10001", "10001", "10001", "11111", "10001", "10001", "10001"],
        'I' => ["11111", "00100", "00100", "00100", "00100", "00100", "11111"],
        'L' => ["10000", "10000", "10000", "10000", "10000", "10000", "11111"],
        'M' => ["10001", "11011", "10101", "10101", "10001", "10001", "10001"],
        'N' => ["10001", "11001", "10101", "10011", "10001", "10001", "10001"],
        'O' => ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
        'P' => ["11110", "10001", "10001", "11110", "10000", "10000", "10000"],
        'R' => ["11110", "10001", "10001", "11110", "10100", "10010", "10001"],
        'S' => ["01111", "10000", "10000", "01110", "00001", "00001", "11110"],
        'T' => ["11111", "00100", "00100", "00100", "00100", "00100", "00100"],
        'U' => ["10001", "10001", "10001", "10001", "10001", "10001", "01110"],
        'V' => ["10001", "10001", "10001", "10001", "10001", "01010", "00100"],
        'W' => ["10001", "10001", "10001", "10101", "10101", "11011", "10001"],
        'X' => ["10001", "10001", "01010", "00100", "01010", "10001", "10001"],
        '0' => ["01110", "10001", "10011", "10101", "11001", "10001", "01110"],
        '1' => ["00100", "01100", "00100", "00100", "00100", "00100", "01110"],
        '2' => ["01110", "10001", "00001", "00010", "00100", "01000", "11111"],
        '3' => ["11110", "00001", "00001", "01110", "00001", "00001", "11110"],
        '4' => ["00010", "00110", "01010", "10010", "11111", "00010", "00010"],
        '5' => ["11111", "10000", "10000", "11110", "00001", "00001", "11110"],
        '6' => ["01110", "10000", "10000", "11110", "10001", "10001", "01110"],
        '7' => ["11111", "00001", "00010", "00100", "01000", "01000", "01000"],
        '8' => ["01110", "10001", "10001", "01110", "10001", "10001", "01110"],
        '9' => ["01110", "10001", "10001", "01111", "00001", "00001", "01110"],
        '.' => ["00000", "00000", "00000", "00000", "00000", "00110", "00110"],
        '+' => ["00000", "00100", "00100", "11111", "00100", "00100", "00000"],
        '-' => ["00000", "00000", "00000", "11111", "00000", "00000", "00000"],
        ':' => ["00000", "00110", "00110", "00000", "00110", "00110", "00000"],
        _ => ["00000", "00000", "00000", "00000", "00000", "00000", "00000"],
    }
}

fn draw_text(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, text: &str, x: i32, y: i32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    for (i, ch) in text.chars().enumerate() {
        for (row, bits) in glyph(ch.to_ascii_uppercase()).iter().enumerate() {
            for (col, bit) in bits.chars().enumerate() {
                if bit == '1' {
                    canvas.fill_rect(sdl2::rect::Rect::new(x + i as i32 * 6 + col as i32, y + row as i32, 1, 1))?;
                }
            }
        }
    }
    Ok(())
}

fn draw_text_scaled(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    text: &str,
    x: i32,
    y: i32,
    scale: i32,
    color: Color,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    for (i, ch) in text.chars().enumerate() {
        for (row, bits) in glyph(ch.to_ascii_uppercase()).iter().enumerate() {
            for (col, bit) in bits.chars().enumerate() {
                if bit == '1' {
                    canvas.fill_rect(sdl2::rect::Rect::new(
                        x + i as i32 * 6 * scale + col as i32 * scale,
                        y + row as i32 * scale,
                        scale as u32,
                        scale as u32,
                    ))?;
                }
            }
        }
    }
    Ok(())
}

fn draw_aa_line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, a: (i32, i32), b: (i32, i32), color: Color) -> Result<(), String> {
    canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, 70));
    canvas.draw_line((a.0 - 1, a.1), (b.0 - 1, b.1))?;
    canvas.draw_line((a.0 + 1, a.1), (b.0 + 1, b.1))?;
    canvas.set_draw_color(color);
    canvas.draw_line(a, b)
}

fn rainbow_color(hue: f64) -> Color {
    let h = hue.rem_euclid(360.0) / 60.0;
    let x = (1.0 - (h % 2.0 - 1.0).abs()) * 255.0;
    let (r, g, b) = match h as u32 {
        0 => (255.0, x, 0.0),
        1 => (x, 255.0, 0.0),
        2 => (0.0, 255.0, x),
        3 => (0.0, x, 255.0),
        4 => (x, 0.0, 255.0),
        _ => (255.0, 0.0, x),
    };
    Color::RGB(r as u8, g as u8, b as u8)
}

fn theme_colors(theme: usize) -> (Color, Color) {
    match theme % 3 {
        0 => (Color::RGB(20, 20, 30), Color::RGB(180, 180, 190)),
        1 => (Color::RGB(8, 24, 20), Color::RGB(160, 220, 180)),
        _ => (Color::RGB(28, 12, 32), Color::RGB(220, 170, 230)),
    }
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
    let start_time = Instant::now();
    let mut trail: Vec<(i32, i32, Color)> = Vec::new();
    let mut simulation_speed = 1.0_f64;
    let mut theme = 0_usize;
    let mut pan_x = 0.0_f64;
    let mut pan_y = 0.0_f64;
    let mut panning = false;
    let mut paused = false;
    let mut grabbed: Option<char> = None;
    let mut mouse_grabbed = false;

    let l1: f64 = 1.5;
    let l2: f64 = 1.5;

    let mut m1: f64 = 1.0;
    let mut m2: f64 = 1.0;

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
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match key {
                    // +/- changes the simulation speed.
                    Keycode::Equals | Keycode::Plus => {
                        simulation_speed = (simulation_speed + 0.25).min(5.0);
                    }
                    Keycode::Minus | Keycode::Underscore => {
                        simulation_speed = (simulation_speed - 0.25).max(0.25);
                    }
                    // Arrow keys adjust the two pendulum weights.
                    Keycode::Up => m1 = (m1 + 0.1).min(10.0),
                    Keycode::Down => m1 = (m1 - 0.1).max(0.1),
                    Keycode::Right => m2 = (m2 + 0.1).min(10.0),
                    Keycode::Left => m2 = (m2 - 0.1).max(0.1),
                    Keycode::B => {
                        theme = (theme + 1) % 3;
                        trail.clear();
                    }
                    Keycode::R => {
                        theta1 = 1.4;
                        theta2 = 1.4;
                        omega1 = 0.0;
                        omega2 = 0.0;
                        trail.clear();
                    }
                    Keycode::M => paused = !paused,
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } if grabbed.is_none() => {
                    let x0 = (SCREEN_WIDTH / 2) as f64 + pan_x;
                    let y0 = 100.0 + pan_y;
                    let x1 = x0 + l1 * theta1.sin() * 100.0;
                    let y1 = y0 + l1 * theta1.cos() * 100.0;
                    let x2 = x1 + l2 * theta2.sin() * 100.0;
                    let y2 = y1 + l2 * theta2.cos() * 100.0;
                    let mouse_x = x as f64;
                    let mouse_y = y as f64;
                    let on_disc = |cx: f64, cy: f64| {
                        let dx = mouse_x - cx;
                        let dy = mouse_y - cy;
                        dx * dx + dy * dy <= 24.0 * 24.0
                    };

                    // Check the lower disc first in case the two discs overlap.
                    if on_disc(x2, y2) {
                        grabbed = Some('G');
                        mouse_grabbed = true;
                    } else if on_disc(x1, y1) {
                        grabbed = Some('H');
                        mouse_grabbed = true;
                    } else {
                        panning = true;
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    panning = false;
                    if mouse_grabbed {
                        grabbed = None;
                        mouse_grabbed = false;
                    }
                }
                Event::MouseMotion { x, y, .. } if grabbed.is_some() => {
                    let (pivot_x, pivot_y) = if grabbed == Some('H') {
                        (
                            (SCREEN_WIDTH / 2) as f64 + pan_x,
                            100.0 + pan_y,
                        )
                    } else {
                        let x0 = (SCREEN_WIDTH / 2) as f64 + pan_x;
                        let y0 = 100.0 + pan_y;
                        (
                            x0 + l1 * theta1.sin() * 100.0,
                            y0 + l1 * theta1.cos() * 100.0,
                        )
                    };
                    let angle = (x as f64 - pivot_x).atan2(y as f64 - pivot_y);
                    if grabbed == Some('H') {
                        theta1 = angle;
                        omega1 = 0.0;
                    } else {
                        theta2 = angle;
                        omega2 = 0.0;
                    }
                }
                Event::MouseMotion { xrel, yrel, .. } if panning => {
                    pan_x += xrel as f64;
                    pan_y += yrel as f64;
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_frame).as_secs_f64() * simulation_speed;
        last_frame = now;

        if !paused {
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
        }

        let x0 = (SCREEN_WIDTH / 2) as f64 + pan_x;
        let y0 = 100.0 + pan_y;

        let x1 = x0 + l1 * theta1.sin() * 100.0;
        let y1 = y0 + l1 * theta1.cos() * 100.0;

        let x2 = x1 + l2 * theta2.sin() * 100.0;
        let y2 = y1 + l2 * theta2.cos() * 100.0;

        // Display the linear speed of each mass, rather than its angular speed.
        let v1 = (l1 * omega1).abs();
        let vx2 = l1 * theta1.cos() * omega1 + l2 * theta2.cos() * omega2;
        let vy2 = -l1 * theta1.sin() * omega1 - l2 * theta2.sin() * omega2;
        let v2 = (vx2 * vx2 + vy2 * vy2).sqrt();

        let (background, pendulum_color) = theme_colors(theme);
        trail.push((
            (x2 - pan_x) as i32,
            (y2 - pan_y) as i32,
            rainbow_color(start_time.elapsed().as_secs_f64() * 120.0),
        ));

        canvas.set_draw_color(background);
        canvas.clear();

        for segment in trail.windows(2) {
            let (x_start, y_start, color) = segment[0];
            let (x_end, y_end, _) = segment[1];
            canvas.set_draw_color(color);
            canvas.draw_line(
                (x_start + pan_x as i32, y_start + pan_y as i32),
                (x_end + pan_x as i32, y_end + pan_y as i32),
            )?;
        }

        canvas.set_draw_color(pendulum_color);

        canvas.draw_line((x0 as i32, y0 as i32), (x1 as i32, y1 as i32))?;

        canvas.draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))?;

        draw_circle(&mut canvas, x0 as i32, y0 as i32, 6, pendulum_color)?;
        draw_circle(&mut canvas, x1 as i32, y1 as i32, 12, pendulum_color)?;
        draw_circle(&mut canvas, x2 as i32, y2 as i32, 12, pendulum_color)?;

        let ui_color = Color::RGB(240, 240, 240);
        draw_text_scaled(
            &mut canvas,
            &format!("SPEED: {:.2}X", simulation_speed),
            20,
            20,
            2,
            ui_color,
        )?;
        draw_text_scaled(&mut canvas, "UP DOWN M1", 20, 38, 2, ui_color)?;
        draw_text_scaled(&mut canvas, "LEFT RIGHT M2", 20, 56, 2, ui_color)?;
        draw_text_scaled(&mut canvas, "PLUS MINUS SPEED", 20, 74, 2, ui_color)?;
        draw_text_scaled(&mut canvas, "B THEME ESC EXIT", 20, 92, 2, ui_color)?;
        draw_text_scaled(&mut canvas, "R RESET", 20, 110, 2, ui_color)?;
        draw_text_scaled(
            &mut canvas,
            if paused { "M PLAY" } else { "M PAUSE" },
            20,
            128,
            2,
            ui_color,
        )?;
        draw_text_scaled(
            &mut canvas,
            &format!("M1:{:.2} V:{:.2}", m1, v1),
            20,
            146,
            2,
            ui_color,
        )?;
        draw_text_scaled(
            &mut canvas,
            &format!("M2:{:.2} V:{:.2}", m2, v2),
            20,
            164,
            2,
            ui_color,
        )?;

        canvas.present();
    }

    Ok(())
}
