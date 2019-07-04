use chip8_emulator;

fn main() {
    chip8_emulator::start_emulator(String::from("example.chip"), 1000);
}
