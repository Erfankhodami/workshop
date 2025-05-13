use text_io::read;
enum Item{
    Text(String),
    Number(i32),
    Float(f32),
}
pub fn mainfn() {
    let mut v=Vec::new();
    v.push(Item::Text(String::from("Hello")));
    v.push(Item::Number(5));
    v.push(Item::Float(3.14));
    for item in &v {
        match item {
            Item::Text(val) => println!("{}", val),
            Item::Number(val) => println!("{}", val),
            Item::Float(val) => println!("{}", val),
        }
    }
}
