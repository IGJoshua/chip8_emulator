use chip8_emulator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    chip8_emulator::start_emulator(
        &args[1],
        if args.len() > 2 {
            args[2].parse().unwrap()
        } else {
            1000
        },
    );
}
