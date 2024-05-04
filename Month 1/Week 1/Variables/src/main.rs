// CONST
const MAX_LOSE: u8 = 3; // Imuitable
const MAX_WIN: u8 = 3; // Imuitable

static CASINO_NAME: &str = "Rusty Casino"; // Imuitable

fn main() {
    println!("Constants & Statics Variables Day 4!");

    // Instade of using the "let" keyword we use "const"
    const MAX_PLAYERS: u8 = 10; // in fact both const and let are Imuitable but the "let" or the default variable can be muitable using the "mut" keyword.. but here in const it's constantly and always imuitable even if you use mut keyword
    // const can be decleared in global meaning outside of the main function also inside

}
