#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
use std::os;
use std::num::{Num};
use std::io::File;

#[deriving (Show, Clone)]
enum Expr<'a>{
    //numeric(&'a Num + 'a),
    Symbol(String),
}

#[deriving (Show, Clone)]
enum Branch<'a>{
    E(Expr<'a>),
    B(Vec<Box<Branch<'a>>>),
}

fn parse<'a>(ts: &mut Vec<String>) -> Branch<'a>{
    let mut tree = vec![];
    let integral = regex!(r"^[0-9]+$");
    let decimal = regex!(r"[0-9]*\.[0-9]+");
    let mut first = true;
    loop{
        match ts.pop(){
            Some(t) => {
                match t.as_slice(){
                    "(" => {
                            if first {
                                first = false;
                            }
                            else{ 
                                tree.push(box parse(ts));
                            }
                        },
                    ")" => break,// B(tree),
                    "" => continue,
                    x => tree.push(box E(Symbol(String::from_str(x)))),
                }
            },
            None => break,
        }
    }
    return B(tree);
}

fn tokenize(fp: &str) -> Vec<String>{
    let data = match File::open(&Path::new(fp)).read_to_string(){
        Ok(n) => n,
        Err(er) => fail!("couldn't read file: {}", er.desc)
    };
    let broken = data.replace("(", " ( ").replace(")", " ) ").replace("  ", " ");
    let mut tokens = vec![];
    
    for t in broken.as_slice().split_str(" ").filter(|&x| *x != "\n"){
        tokens.push(String::from_str(t));
    }
    return tokens
}

fn main(){
    let args = &os::args()[1];
    //println!("{}", tokenize(args.as_slice()));
    let backward = tokenize(args.as_slice());
    let mut ts = vec![];
    for i in range(0, backward.len()){
        ts.push(backward[backward.len() - i - 1].clone());
    }
    println!("{}", parse(&mut ts));
}
