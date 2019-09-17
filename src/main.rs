fn main() {
    println!("Hello, world!");
}

struct Symbol {
    symbol: String
}

enum Number {
    Integer(i32),
    Float(f32)
}

enum Atom {
    Symbol(Symbol),
    Number(Number)
}

enum Exp {
    Atom(Atom),
    List(Vec<String>)
}

// Split the string into tokens
fn tokenise(chars: String) -> Vec<String> {
    // Space the parenthesis so that they can be split out
    let spaced_chars: String = chars.chars().map(|c| match c {
        '(' => String::from(" ( "),
        ')' => String::from(" ) "),
        _ => c.to_string()
    }).collect();
    
    spaced_chars.split_whitespace().map(|token| {
        String::from(token)
    }).collect()
}

fn parse(program: String) -> Exp {}

fn read_from_tokens(tokens: Vec<char>) -> Exp {}

fn atomise(token: String) -> Atom {}