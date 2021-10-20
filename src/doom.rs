use console_engine::{pixel, ConsoleEngine, KeyCode};
// use std::{thread, time};

pub const W: u32 = 20;
pub const H: u32 = 20;

pub const SCREEN_W: u32 = 50;
pub const SCREEN_H: u32 = 25;

pub struct Entity {
    x: f32,
    y: f32,
    badge: char,
    name: String,
}

pub struct Map {
    w: u32,
    h: u32,
    data: String,
    entities: Vec<Box<Entity>>,
}

impl Map {
    pub fn new(w: u32, h: u32, v: &Vec<&str>) -> Map {
        let mut s = "".to_owned();
        for &line in v {
            s.push_str(line);
        }
        return Map {
            w,
            h,
            data: s,
            entities: vec![],
        };
    }
    pub fn get_pxl(&self, x: u32, y: u32) -> char {
        let pos = (y * self.w + x) as usize;
        return if pos < self.data.len() {
            self.data.as_bytes()[pos] as char
        } else {
            ' '
        };
    }
    pub fn render(&self, engine: &mut ConsoleEngine) {
        for y in 0..self.h {
            for x in 0..self.w {
                engine.set_pxl((x * 2) as i32, y as i32, pixel::pxl(self.get_pxl(x, y)));
                engine.set_pxl((x * 2 + 1) as i32, y as i32, pixel::pxl(' '));
            }
        }
        for entity in self.entities.iter() {
            engine.set_pxl(
                (entity.x.round() * 2.) as i32,
                entity.y.round() as i32,
                pixel::pxl(entity.badge),
            );
        }
    }
    pub fn push_entity(&mut self, entity: Box<Entity>) {
        self.entities.push(entity);
        // let entity: Entity {
        //     x: 15,
        //     y: 15,
        //     badge: '@',
        //     name: "Player".to_string(),
        // }
    }
}

pub fn run(engine: &mut ConsoleEngine) {
    let v = vec![
        ".......YYY..........",
        ".......YYY..........",
        ".......YYY..........",
        "....................",
        "....................",
        "....................",
        "...XXXXXXXX.........",
        "...XXXXXXXX.........",
        "...XXXXXXXX.........",
        "...XXXXXXXX.........",
        "...XXXXXXXX.........",
        "...XXXXXXXX.........",
        "....................",
        "....................",
        "....................",
        "....................",
        "....................",
        "....................",
        "....................",
        "....................",
    ];

    let mut map = Map::new(W, H, &v);

    let player = Box::new(Entity {
        x: 2.,
        y: 2.,
        badge: '@',
        name: "bro".to_string(),
    });

    let prikol = Box::new(Entity {
        x: 15.,
        y: 15.,
        badge: '2',
        name: "prikol".to_string(),
    });

    map.push_entity(player);
    map.push_entity(prikol);

    engine.clear_screen();
    loop {
        engine.wait_frame();
        engine.clear_screen();
        map.render(engine);

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
    }
}
