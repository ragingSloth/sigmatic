#![feature(phase)]
#![feature(globs)]
#![feature(plugin_registrar)]
#![crate_type="dylib"]


extern crate rustc;
extern crate syntax;
extern crate alloc;
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
use alloc::rc::{Rc};
use Tree::*;


//pub trait Lisp
//{
//    fn parse<'a>(&'a mut self) -> Tree<'a>;
//}
//
//impl Lisp for TokenTree{
//    fn parse<'a>(&'a mut self) -> Tree<'a>{ 
//        match *(self as *mut TokenTree){
//            TtDelimited(_, y) => {
//                match y.delim{
//                    token::DelimToken::Paren => return y.parse(),
//                    _ => panic!("not done yet"),
//                }
//            },
//            TtToken(_, ref mut t) => return E(box t.clone()),
//            _ => panic!("not done yet"),
//        }
//    }
//}
//
//impl Lisp for Delimited{
//    fn parse<'a>(&'a mut self) -> Tree<'a>{
//        let mut LL = vec![];
//        loop{
//            match self.tts.pop(){
//                Some(ref mut x) => {
//                    let temp = x.parse().clone();
//                    LL.push(temp);
//                    },
//                None => break,
//            };
//        }
//       return St(LL);
//    }
//}

fn parse_delimited<'a>(d: Rc<Delimited>) -> Tree<'a>{
    let mut LL = vec![];
    let mut ts = d.tts.clone();
    loop{
        match ts.pop(){
            Some(x) => {
                let temp = parse_TT(x);
                LL.push(temp);
                },
            None => break,
        };
    }
   return St(LL);
}

fn parse_TT<'a>(tt: TokenTree) -> Tree<'a>{
    match tt.clone() {
        TtDelimited(_, y) => {
            match y.delim{
                token::DelimToken::Paren => return parse_delimited(y.clone()),
                _ => panic!("not done yet"),
            }
        },
        TtToken(_, ref mut t) => return E(box t.clone()),
        _ => panic!("not done yet"),
    }
}

#[deriving (Show, Clone)]
pub enum Tree<'a>{
    E(Box<token::Token>),
    St(Vec<Tree<'a>>),
}

fn expand_lisp(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    //println!("{}", args);
    //let mut n = vec![args];
   // println!("{}", n.pop());
    let text = match args[0] {
        TtDelimited(_, ref x) => parse_delimited(x.clone()),
        _ => panic!("that's not lisp"),
    };
    println!("{}", text);
    MacExpr::new(cx.expr_uint(sp, 8u))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lisp", expand_lisp);
}
