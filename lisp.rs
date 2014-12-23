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
use Tree::*;


pub trait Lisp
{
    fn parse<'a>(&'a mut self) -> Tree<'a>;
}

impl Lisp for TokenTree{
    fn parse<'a>(&'a mut self) -> Tree<'a>{ 
        match *self{
            TtDelimited(_, ref mut y) => {
                match y.delim{
                    token::DelimToken::Paren => return y.parse(),
                    _ => panic!("not done yet"),
                }
            },
            TtToken(_, ref mut t) => return E(t.clone()),
            _ => panic!("not done yet"),
        }
    }
}

impl Lisp for Delimited{
    fn parse<'a>(&'a mut self) -> Tree<'a>{
        let mut LL = vec![];
        loop{
            match self.tts.pop(){
                Some(ref mut x) => {
                    let temp = box x.parse().clone();
                    LL.push(temp);
                    },
                None => break,
            };
        }
        return St(LL)
    }
}

#[deriving (Show, Clone)]
pub enum Tree<'a>{
    E(token::Token),
    St(Vec<Box<Tree<'a>>>),
}

fn expand_lisp(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    //println!("{}", args);
    //let mut n = vec![args];
   // println!("{}", n.pop());
    let text = match args[0] {
        TtDelimited(_, _) => (*args)[0].parse(),
        _ => panic!("that's not lisp"),
    };
    println!("{}", text);
    MacExpr::new(cx.expr_uint(sp, 8u))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lisp", expand_lisp);
}
