use std::ops::Range;
use reqwest;
pub fn mainfn(){
    let cities=vec!["tehran","isfahan","yazd","shiraz","nishabor"];
    let ranges=vec![(0..250),(250..500),(500..750),(750..1000)];
    let mut handlers=Vec::new();
    for range in ranges {
        let handle=std::thread::spawn( || { FindPrimeNums(range) });
        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();
    }

}
fn FindPrimeNums(range:Range<i32>){
    println!("gonna find...");
    for num in range {
        if(IsPrime(num)){
            println!("{num}");
        }
    }
}

fn IsPrime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}