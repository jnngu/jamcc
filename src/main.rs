mod lexer;

fn main() {
     let text = "int main(){return 100;}";
 /*   let re = Regex::new(r"^\{|\}|\(|\)|;|int|return|[a-zA-Z]\w*|\d+|$").unwrap();

    //fix this when i know more about iterators xd
    let test:Vec<String> = re.find_iter(text).filter_map(|x| x.as_str().parse().ok()).collect(); 
     */
    let test:Vec<String> = lexer::parse_string(text);
    for x in &test {
        println!("{}", x);
    }
}

