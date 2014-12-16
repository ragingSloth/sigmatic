#![feature(phase)]
#![feature(globs)]
#![feature(plugin_registrar)]
#![crate_type="dylib"]


extern crate rustc;
extern crate syntax;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree, TtToken, TtDelimited, Delimited};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacExpr};
use syntax::ext::build::AstBuilder;

#[phase(plugin)]
use rustc::plugin::Registry;

use std::os;
use std::num::{Num};
use std::io::File;
use Expr::*;
use Branch::*;

#[deriving (Show, Clone)]
enum Expr<'a>{
    Symbol(String),
}

#[deriving (Show, Clone)]
enum Branch<'a>{
    E(Expr<'a>),
    B(Vec<Box<Branch<'a>>>),
}

fn parse<'a>(ts: &mut Vec<String>) -> Branch<'a>{
    let mut tree = vec![];
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
        Err(er) => panic!("couldn't read file: {}", er.desc)
    };
    let broken = data.replace("(", " ( ").replace(")", " ) ").replace("  ", " ");
    let mut tokens = vec![];
    
    for t in broken.as_slice().split_str(" ").filter(|&x| *x != *"\n"){
        tokens.push(String::from_str(t));
    }
    return tokens
}


fn expand_lisp(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    println!("{}", args);
    let mut n = vec![args];
    println!("{}", n.pop());
    let text = match args {
            [TtDelimited(_, ref y)] => {
                match y.delim{
                   token::DelimToken::Paren => "(",
                    _=> "Error"
                }
            }
        _ => {
            cx.span_err(sp, "gotta ident bro");
            return DummyResult::any(sp);
        }
    };
    println!("{}", text);
    MacExpr::new(cx.expr_uint(sp, 8u))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lisp", expand_lisp);
}
