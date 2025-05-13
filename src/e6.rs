use text_io::read;
fn bits(n: u32) -> u32 {
    let mut bits = 0;
    let mut count = 1;
    while count < n {
        count *= 2;
        bits += 1;
    }
    bits
}
pub fn mainfn() {
    loop {
        let n: u32 = read!();
        println!("{}", bits(n));
    }
}
