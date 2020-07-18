use regex::Regex;
use std::fs;
use std::collections::VecDeque;

 pub enum Token {
    OpenBrace,
    ClosedBrace,
    OpenParen,
    ClosedParen,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier(String),
    IntegerLiteral(i32),
}
 

pub fn read_file(file_name: &str) -> String
{
    let contents = fs::read_to_string(file_name).expect("Unable to read file");
    contents
} 

pub fn print_tokens(x: &VecDeque<Token>) -> ()
{
    let mut index = 0;
    let length = x.len()-1;
    print!("[");
    for elem in x
    {
        match elem
        {
            Token::OpenBrace => print!("OpenBrace"),
            Token::ClosedBrace => print!("ClosedBrace"),
            Token::OpenParen =>  print!("OpenParen"),
            Token::ClosedParen => print!("ClosedParen"),
            Token::Semicolon => print!("Semicolon"),
            Token::IntKeyword => print!("IntKeyword"),
            Token::ReturnKeyword => print!("ReturnKeyWord"),
            Token::Identifier(s) => print!("Identifier({})", s),
            Token::IntegerLiteral(i) => print!("Integer({})", i),
            _ => panic!("Invalid token"),

        }
        if index != length
        {
            index = index + 1;
            print!(", ");
        }

    }
    println!("]");
} 

pub fn parse_string(x: &str) -> VecDeque<Token>
{
    let re = Regex::new(r#"^\{|^\}|^\(|^\)|^;"#).unwrap();
    let int_reg = Regex::new(r#"^[0-9]+"#).unwrap();
    let char_reg = Regex::new(r#"^[a-zA-Z]\w*"#).unwrap();

    let mut matches:VecDeque<Token> = VecDeque::new();
    //fix this when i know more about iterators xd


    let mut temp = String::from(x.trim_start());
    while !(&temp.is_empty())
    {
        let next_match;
        if re.is_match_at(&temp, 0) 
        {
            next_match =  re.find(&temp).unwrap().as_str();
            match next_match {
                "{" => matches.push_back(Token::OpenBrace),
                "}" => matches.push_back(Token::ClosedBrace),
                "(" => matches.push_back(Token::OpenParen),
                ")" => matches.push_back(Token::ClosedParen),
                ";" => matches.push_back(Token::Semicolon),
                _ => panic!("not a valid token"),
            }
            println!("{}", next_match);
        }
        else if int_reg.is_match_at(&temp, 0)
        {
            next_match =  int_reg.find(&temp).unwrap().as_str();
            let num:i32 = next_match.parse().expect("Not an i32"); //TODO: do bounds checking here
            matches.push_back(Token::IntegerLiteral(num));
            println!("{}", next_match);
        }
        else if char_reg.is_match_at(&temp, 0)
        {
            next_match =  char_reg.find(&temp).unwrap().as_str();
            match next_match {
                "int" => matches.push_back(Token::IntKeyword),
                "return" => matches.push_back(Token::ReturnKeyword),
                _ => matches.push_back(Token::Identifier(next_match.to_string())),
            }
            println!("{}", next_match);
        }
        else
        {
            panic!("invalid");
            break;
        }
        temp = temp.replacen(next_match, "", 1).trim_start().to_string();
    }

    matches  
}