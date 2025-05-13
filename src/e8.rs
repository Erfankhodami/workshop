use serde::{Serialize, Deserialize};
use std::{fs, io};
use std::fs::File;
use std::path::Path;
use std::ptr::write;
use text_io::read;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct person{
    name:String,
    age:u8,
    gender:String,
    skin_color:String,
}
pub fn mainfn(){
    let mut people:Vec<person> = Vec::new();
    people=ReadFromJson();
    /*let person1 = person{
        name:"ahmad".to_string(),
        age:15,
        gender:"male".to_string(),
        skin_color:true,
    };
    let person2 = person{
        name:"mahdi".to_string(),
        age:16,
        gender:"male".to_string(),
        skin_color:true,

    };
    people.push(person1);
    people.push(person2);
    */
    SaveToJson(&people);
    let _person=GetUserInput();
    search_in_json(&people);
}

fn SaveToJson(itmes: &Vec<person>) {
    let path: &Path = Path::new("data.json");
    if !path.exists() {
        File::create(path).unwrap();
    }
    fs::write(path, serde_json::to_string_pretty(itmes).unwrap()).unwrap();
}
fn ReadFromJson() -> Vec<person> {
    let mut items: Vec<person> = Vec::new();
    let path: &Path = Path::new("data.json");
    let mut file: File;
    if path.exists() {
        file = File::open(path).unwrap();
    } else {
        file = File::create(path).unwrap()
    }
    let data = file.metadata().unwrap();
    if data.len() != 0 {
        items = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    }
    items
}
fn search_in_json(people: &Vec<person>){
    let toSearch = GetUserInput();
    let mut foundPeople:Vec<person>=Vec::new();
    foundPeople= people.clone();
    if(toSearch.name!="".to_string()){
        for n in people.iter().enumerate() {
            if(toSearch.name!=n.1.name){
                foundPeople.remove(n.0);
            }
        }
    }
    if(toSearch.gender!="".to_string()){
        for n in people.iter().enumerate() {
            if(toSearch.gender!=n.1.gender){
                foundPeople.remove(n.0);
            }
        }
    }
    if(toSearch.skin_color!="".to_string()){
        for n in people.iter().enumerate() {
            if(toSearch.skin_color!=n.1.skin_color){
                foundPeople.remove(n.0);
            }
        }
    }
    if(toSearch.age!=0){
        for n in people.iter().enumerate() {
            if(toSearch.age!=n.1.age){
                foundPeople.remove(n.0);
            }
        }
    }
    let path: &Path = Path::new("result.json");
    if !path.exists() {
        File::create(path).unwrap();
    }
    fs::write(path, serde_json::to_string_pretty(&foundPeople).unwrap()).unwrap();
    println!("wrote to json!");
}
fn GetUserInput()->person{
    let mut items: person = person {
        name: "".to_string(),
        age: 0,
        gender: "".to_string(),
        skin_color: "".to_string(),
    };
    let path: &Path = Path::new("search.json");
    let mut file: File;
    if path.exists() {
        file = File::open(path).unwrap();
    } else {
        file = File::create(path).unwrap()
    }
    let data = file.metadata().unwrap();
    if data.len() != 0 {
        items = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    }
    items
}
