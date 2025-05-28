mod e3;
mod e4;
mod e5;
mod e6;
mod e7;
mod e8;
mod modWeather;

use std::fmt::UpperHex;
fn main() {
/*    let args = std::env::args();
    let x:Vec<u64>=args.skip(1).map(|x| x.parse::<u64>().unwrap()).filter(|x| x%2==0).collect();
    println!("{:?}",x);
*/
/*
    let args = std::env::args();
    let x:Vec<dyn UpperHex>=args.skip(1).map(|x| x.parse::<dyn UpperHex>().unwrap()).collect();
    for n in x {
        println!("{:X}",n);
    }
*/
/*    let args = std::env::args();
    let x:bool=args.skip(1).map(|x| x.parse::<u64>().unwrap()).all(|x| x%5==0);
    println!("{}",x as u8);
*/
/*    let args: Vec<u64> = std::env::args()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();


    let (greater_or_equal, less): (Vec<u64>, Vec<u64>) =
        args.into_iter().partition(|&x| x >= 100);

    print!("{:?} ", greater_or_equal);
    print!("{:?}", less);
*/
/*
    let mut args = std::env::args();
    let x:i32=args.next().unwrap().parse().unwrap();
    let mut result=0;
    for n in 1..=x {
        result*=n;
    }
    println!("{result}");
*/

    //e3::mainfn();
    //e4::mainfn();
    //e5::mainfn();
    //e6::mainfn();
    e8::mainfn();
}

