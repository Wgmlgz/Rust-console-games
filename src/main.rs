use console_engine::ConsoleEngine;

mod doom;
fn main() {
    let mut engine = ConsoleEngine::init(doom::SCREEN_W, doom::SCREEN_H, 3).unwrap();
    doom::run(&mut engine);
}
