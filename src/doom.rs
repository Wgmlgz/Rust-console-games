use console_engine::{pixel, ConsoleEngine, KeyCode};
use std::{f32::consts::PI, time::Instant};

pub const W: u32 = 20;
pub const H: u32 = 20;

pub const SCREEN_W: u32 = 150;
pub const SCREEN_H: u32 = 40;

#[derive(Default, Clone, Copy)]
pub struct V2 {
    x: f32,
    y: f32,
}

impl V2 {
    pub fn move_polar(&mut self, dist: f32, ang: f32) {
        self.x += ang.cos() * dist;
        self.y += ang.sin() * dist;
    }
}

#[derive(Default)]
pub struct Entity {
    pos: V2,
    ang: f32,
    badge: char,
    name: String,
}

impl Entity {
    fn render(&self, engine: &mut ConsoleEngine) {
        engine.set_pxl(
            (self.pos.x.round() * 2.) as i32,
            self.pos.y.round() as i32 + 20,
            pixel::pxl(self.badge),
        );
    }
}

#[derive(Default)]
pub struct Creature {
    entity: Entity,
    vel: f32,
}

impl Creature {
    fn move_ang(&mut self, dt: f32, ang: f32) {
        let ang = ang + self.entity.ang;
        let dir = self.vel * dt;

        self.entity.pos.x += ang.cos() * dir;
        self.entity.pos.y += ang.sin() * dir;
    }
    fn rotate(&mut self, dt: f32, ang: f32) {
        self.entity.ang += ang * dt;
    }
}

struct Map {
    w: u32,
    h: u32,
    world: String,
    player: Creature,
}

impl Map {
    fn new(w: u32, h: u32, v: &Vec<&str>) -> Map {
        let mut s = "".to_owned();
        for &line in v {
            s.push_str(line);
        }
        let mut map = Map {
            w,
            h,
            world: s,
            player: Creature::default(),
        };
        map.player.entity.badge = 'P';
        map.player.entity.name = String::from("Player");
        map.player.vel = 10.;
        map.player.entity.pos.x = 3.;
        map.player.entity.pos.y = 3.;
        map
    }
    fn get_pxl(&self, x: i32, y: i32) -> char {
        let pos = y * self.w as i32 + x;
        if pos < 0 {
            return '^';
        }
        let pos = pos as usize;
        return if pos < self.world.len() {
            self.world.as_bytes()[pos] as char
        } else {
            '^'
        };
    }
    fn render(&self, engine: &mut ConsoleEngine) {
        engine.fill_rect(
            0,
            (engine.get_height() / 2) as i32,
            engine.get_width() as i32,
            (engine.get_height()) as i32,
            pixel::pxl('`'),
        );

        let fov = PI / 2.5;
        let midpx = (engine.get_height() / 2) as i32;
        for x in 0..engine.get_width() as i32 {
            let ang =
                self.player.entity.ang - fov / 2. + fov / engine.get_width() as f32 * x as f32;
            let mut dist: f32 = 0.;
            let step = 0.05;

            let mut cur = self.player.entity.pos.clone();
            while self.get_pxl(cur.x.round() as i32, cur.y.round() as i32) == ' ' {
                cur.move_polar(step, ang);
                dist += step;
            }
            let bar_sz = ((1. / dist).atan() * 12.).round() as i32;
            let ch_hit = self.get_pxl(cur.x.round() as i32, cur.y.round() as i32);
            engine.line(x, midpx - bar_sz, x, midpx + bar_sz, pixel::pxl(ch_hit));
        }
    }
    fn render_map(&self, engine: &mut ConsoleEngine) {
        for y in 0..self.h as i32 {
            for x in 0..self.w as i32 {
                engine.set_pxl((x * 2) as i32, y + 20, pixel::pxl(self.get_pxl(x, y)));
                engine.set_pxl((x * 2 + 1) as i32, y + 20, pixel::pxl(' '));
            }
        }
        self.player.entity.render(engine);
    }
    fn render_help(&self, engine: &mut ConsoleEngine) {
        engine.print(0, 0, "wasd: move, qe: rotate, f: map, h: help, x: exit")
    }
}

pub fn run(engine: &mut ConsoleEngine) {
    let v = vec![
        "#=#=#=#=#=#=#=#=#=#=",
        "#    #             #",
        "=    =        #    =",
        "#    #        =    #",
        "=      #=#    #    =",
        "#           #=#    #",
        "=    #=#=          =",
        "#    =             #",
        "=    #    #        =",
        "#         =#=#     #",
        "=         #        =",
        "#                  #",
        "=    #      #=#    =",
        "#    =             #",
        "=    #   #=#       =",
        "#           #=#    #",
        "=                  =",
        "#   #=#    #=#     #",
        "=                  =",
        "#=#=#=#=#=#=#=#=#=#=",
    ];

    let mut map = Map::new(W, H, &v);

    engine.clear_screen();

    let mut minimap = false;
    let mut help = false;

    loop {
        let now = Instant::now();
        engine.wait_frame();
        engine.clear_screen();

        let dt = now.elapsed().as_secs_f32();

        if engine.is_key_pressed(KeyCode::Char('w')) {
            map.player.move_ang(dt, 0.);
        }
        if engine.is_key_pressed(KeyCode::Char('a')) {
            map.player.move_ang(dt, -PI / 2.);
        }
        if engine.is_key_pressed(KeyCode::Char('s')) {
            map.player.move_ang(dt, PI);
        }
        if engine.is_key_pressed(KeyCode::Char('d')) {
            map.player.move_ang(dt, PI / 2.);
        }
        if engine.is_key_pressed(KeyCode::Char('q')) {
            map.player.rotate(dt, -10.);
        }
        if engine.is_key_pressed(KeyCode::Char('e')) {
            map.player.rotate(dt, 10.);
        }
        if engine.is_key_pressed(KeyCode::Char('x')) {
            break;
        }
        if engine.is_key_pressed(KeyCode::Char('f')) {
            minimap ^= true;
        }
        if engine.is_key_pressed(KeyCode::Char('h')) {
            help ^= true;
        }
        map.render(engine);
        if minimap {
            map.render_map(engine);
        }
        if help {
            map.render_help(engine);
        }

        engine.draw();
    }
}
