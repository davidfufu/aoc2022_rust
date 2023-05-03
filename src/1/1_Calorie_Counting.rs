use std::fs::File;
use std::io::{ self, BufRead, BufReader };


fn calorie_counting(){
    let file_name = "./calories.txt";
    let file = File::open(file_name).unwrap();
    let lines_interator = io::BufReader::new(file).lines(); 

    for line in lines_interator{
        println!("{}", line.unwrap());
    }
}