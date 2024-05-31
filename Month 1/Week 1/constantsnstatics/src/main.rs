const MAX_PLAYER: u8 = 10;
static CASINO_NAME: &str = "Rust";

fn main() {
    println!("Constants & Statics");

    let a: u8 = MAX_PLAYER;
    let b: u8 = MAX_PLAYER;

    let c: &str = CASINO_NAME;
    let d: &str = CASINO_NAME;
}
