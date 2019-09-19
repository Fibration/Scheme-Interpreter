use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Number {
    Integer(i32),
    Float(f32)
}

#[derive(Debug)]
enum Atom {
    Symbol(String),
    Number(Number),
    Func(String)
}

#[derive(Debug)]
enum Exp {
    Atom(Atom),
    List(Vec<Exp>)
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

fn parse(program: String) -> Exp {
    let mut tokenised: Vec<String> = tokenise(program);
    tokenised.reverse();
    read_from_tokens(tokenised).0
}

fn read_from_tokens(tokens: Vec<String>) -> (Exp, Vec<String>) {
    let mut tokens_here = tokens;
    let capacity = tokens_here.len() - 1; // allocate size of subclause vector

    // get the first token
    let token = match tokens_here.pop() {
        Some(token) => token,
        None => String::new()
    };

    // if the token is (, recurse to process subclause else return atom
    if token == "(" {
        let mut clause_list: Vec<Exp> = Vec::with_capacity(capacity);

        while tokens_here[tokens_here.len() - 1] != ")" {
            let (subclause, tape) = read_from_tokens(tokens_here);
            clause_list.push(subclause);
            tokens_here = tape; // get back the rest of the tape
        }

        tokens_here.pop(); // remove )
        (listify(clause_list), tokens_here)
    } else {
        (atomise(token), tokens_here)
    }
}

fn listify(token: Vec<Exp>) -> Exp {
    Exp::List(token)
}

// encode ints and floats before returning atom
fn atomise(token: String) -> Exp {
    match token.parse::<i32>() {
        Ok(int) => Exp::Atom(Atom::Number(Number::Integer(int))),
        Err(not_int) => match token.parse::<f32>() {
            Ok(float) => Exp::Atom(Atom::Number(Number::Float(float))),
            Err(nan) => Exp::Atom(Atom::Symbol(token))
        }
    }
}

fn get_standard_env() -> HashMap<String, Atom> {
    let mut env: HashMap<String, Atom> = HashMap::new();
    env.insert(String::from("pi"), Atom::Number(Number::Float(std::f32::consts::PI)));
    env.insert(String::from("*"), Atom::Func(String::from("*")));

    env
}

fn lisp_eval(x: Exp, env: HashMap<String, Atom>) -> Exp {}

#[test]
fn calculate_circle_area() {
    let syntax_tree: Exp = parse(String::from("(* pi (* r r))"));
    let square: Exp = Exp::List(
        vec![atomise(String::from("*")), atomise(String::from("r")), atomise(String::from("r"))]
    );
    let area: Exp = Exp::List(vec![atomise(String::from("*")), atomise(String::from("pi")), square]);
    println!("syntax tree is {:#?}", syntax_tree);
    println!("area is {:#?}", area);
}