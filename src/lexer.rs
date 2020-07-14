use regex::Regex;

use std::fs;

pub fn read_file(file_name: &str) -> String
{
    let contents = fs::read_to_string(file_name).expect("Unable to read file");
    contents
} 


pub fn parse_string(x: &str) -> Vec<String>
{
    let re = Regex::new(r"^\{|\}|\(|\)|;|int|return|[a-zA-Z]\w*|\d+|$").unwrap();

    //fix this when i know more about iterators xd
    let matches:Vec<String> = re.find_iter(x).filter_map(|x| x.as_str().parse().ok()).collect(); 
    matches  
}