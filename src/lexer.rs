use regex::Regex;

pub fn parse_string(x: &str) -> Vec<String>
{
    let re = Regex::new(r"^\{|\}|\(|\)|;|int|return|[a-zA-Z]\w*|\d+|$").unwrap();

    //fix this when i know more about iterators xd
    let output:Vec<String> = re.find_iter(x).filter_map(|x| x.as_str().parse().ok()).collect(); 
    output   
}