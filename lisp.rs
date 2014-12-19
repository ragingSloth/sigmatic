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
    fn parse(&mut self) -> Tree;
}

impl Lisp for TokenTree{
    fn parse(&mut self) -> Tree{ 
        match self.clone(){
            TtDelimited(_, y) => {
                let mut y2 = box () (*y).clone();
                match y2.delim{
                    token::DelimToken::Paren => y2.parse(),
                    _ => panic!("not done yet"),
                }
            },
            TtToken(_, t) => E(t),
            _ => panic!("not done yet"),
        }
    }
}

impl Lisp for Delimited{
    fn parse(&mut self) -> Tree{
        let mut LL = vec![];
        loop{
            match self.tts.pop(){
                Some(x) => {
                    LL.push(box x.parse().clone());
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
