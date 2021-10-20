use console_engine::{pixel, pixel::Pixel, Color, KeyCode};
use rand::Rng;
use std::{thread, time};

const W: usize = 25;
const H: usize = 25;
const ITERATIONS: usize = 20;
const DELAY: u64 = 200;

fn print_canvas(engine: &mut console_engine::ConsoleEngine, canvas: &[[bool; W]; H]) {
    for y in 0..W {
        for x in 0..H {
            engine.set_pxl(
                (x * 2) as i32,
                y as i32,
                pixel::pxl(match canvas[x][y] {
                    true => '#',
                    _ => ' ',
                }),
            );
            engine.set_pxl((x * 2 + 1) as i32, y as i32, pixel::pxl(' '))
        }
    }
    engine.draw();
}

fn main() {
    let mut engine = console_engine::ConsoleEngine::init((W * 2) as u32, H as u32, 3).unwrap();

    println!("Game of live!!!");

    let empty_canvas: [[bool; W]; H] = [[false; W]; H];
    let mut canvas: [[bool; W]; H] = empty_canvas;

    for row in canvas.iter_mut() {
        for el in row.iter_mut() {
            *el = rand::thread_rng().gen_bool(0.5);
        }
    }

    canvas[3][3] = true;
    canvas[3][4] = true;
    canvas[3][5] = true;

    for _ in 0..ITERATIONS {
        thread::sleep(time::Duration::from_millis(DELAY));
        print_canvas(&mut engine, &canvas);
        let mut next_canvas = empty_canvas;

        for i in 1..(W - 1) {
            for j in 1..(H - 1) {
                let mut near = 0;
                for x in -1..2 {
                    for y in -1..2 {
                        match (x, y) {
                            (0, 0) => continue,
                            (x, y) if canvas[(i as i32 + x) as usize][(j as i32 + y) as usize] => {
                                near += 1
                            }
                            _ => continue,
                        }
                    }
                }

                next_canvas[i][j] = match (canvas[i][j], near) {
                    (true, near) if near == 2 || near == 3 => true,
                    (true, _) => false,
                    (false, near) if near == 3 => true,
                    _ => false,
                }
            }
        }

        canvas = next_canvas;
    }
}
