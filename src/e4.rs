pub fn mainfn() {
    for i in 1570..1741{
        let c = char::from_u32(i);
        if let Some(c) = c {
            println!("{c}");
        }
    }
}
//fn get_and_translate(s:&str){
//  for c in s.chars(){
//    println!("'{}' => u+{:04X}", c , c as u32);
//}
//}
//fn main(){
//let s="سلام";
//get_and_translate(s);
//}
