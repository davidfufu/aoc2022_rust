use std::fs::File;
use std::io::{ self, BufRead };
use std::env;

pub fn calorie_counting(){
    let p = env::current_dir().unwrap();
    let file_name = format!("{}/src/day_one/calories.txt",p.display());
    println!("{}", file_name);
    let file = File::open(file_name).unwrap();
    let lines_interator = io::BufReader::new(file).lines(); 

    for line in lines_interator{
        println!("{}", line.unwrap());
    }
}