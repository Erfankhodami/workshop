use text_io::read;

fn main()  {
    println!("Please enter your city name to check:");
    let cityname:String = read!();
    let url = format!("https://wttr.in/{cityname}?format=3");
    let response = reqwest::blocking::get(url).unwrap();
    let content = response.text().unwrap();
    println!("The city temperature is: {content}");
}