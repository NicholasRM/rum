use std::env;
use rum::rumload::parse_input;
use rum::rumdata::RumData;

fn main() {
    let filename = env::args().nth(1);
    let program = parse_input(filename.as_deref());
    let mut machine = RumData::load(program);
    machine.execute();
}