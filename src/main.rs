use console_engine::ConsoleEngine;

mod doom;
fn main() {
    let mut engine = ConsoleEngine::init(doom::SCREEN_W, doom::SCREEN_H, 60).unwrap();
    doom::run(&mut engine);
}
