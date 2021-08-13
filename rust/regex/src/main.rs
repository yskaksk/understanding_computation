use std::collections::HashSet;

use regex::pattern::Pattern::*;
use regex::parse;

fn main() {
    let pattern = Empty;
    println!("{}", pattern.to_s());
    println!(" => {}", pattern.matches(String::from("")));
    println!("a => {}", pattern.matches(String::from("a")));

    let pattern = Concatenate {
        first: Box::new(Concatenate {
            first: Box::new(Literal { character: 'a' }),
            second: Box::new(Literal { character: 'b' }),
        }),
        second: Box::new(Literal { character: 'c' }),
    };
    println!("{}", pattern.to_s());
    println!("abc => {}", pattern.matches(String::from("abc")));
    println!("a => {}", pattern.matches(String::from("ac")));
    println!("abd => {}", pattern.matches(String::from("ac")));

    let pattern = Choose {
        first: Box::new(Literal { character: 'a' }),
        second: Box::new(Literal { character: 'b' }),
    };
    println!("{}", pattern.to_s());
    println!("a => {}", pattern.matches(String::from("a")));
    println!("b => {}", pattern.matches(String::from("b")));
    println!("c => {}", pattern.matches(String::from("c")));

    let pattern = Repeat(Box::new(Choose {
        first: Box::new(Concatenate {
            first: Box::new(Literal { character: 'a' }),
            second: Box::new(Literal { character: 'b' }),
        }),
        second: Box::new(Literal { character: 'a' }),
    }));
    println!("{}", pattern.to_s());
    println!("ab => {}", pattern.matches(String::from("ab")));
    println!("abab => {}", pattern.matches(String::from("abab")));
    println!("aba => {}", pattern.matches(String::from("aba")));
    println!("aa => {}", pattern.matches(String::from("aa")));
    println!("ac => {}", pattern.matches(String::from("ac")));
    println!("ba => {}", pattern.matches(String::from("ba")));
    println!("bb => {}", pattern.matches(String::from("bb")));
    println!("abbb => {}", pattern.matches(String::from("abbb")));

    let nfa_design = Empty.to_nfa_design();
    println!(
        "nfa_design.accept(''): {}",
        nfa_design.accepts(String::from(""))
    );
    println!(
        "nfa_design.accept('a'): {}",
        nfa_design.accepts(String::from("a"))
    );

    println!("複雑なパターン");
    let pattern = Repeat(Box::new(Concatenate {
        first: Box::new(Literal { character: 'a' }),
        second: Box::new(Choose {
            first: Box::new(Empty),
            second: Box::new(Literal { character: 'b' }),
        }),
    }));
    println!("{}", pattern.to_s());
    println!(" => {}", pattern.matches(String::from("")));
    println!("a => {}", pattern.matches(String::from("a")));
    println!("ab => {}", pattern.matches(String::from("ab")));
    println!("aba => {}", pattern.matches(String::from("aba")));
    println!("abab => {}", pattern.matches(String::from("abab")));
    println!("abaab => {}", pattern.matches(String::from("abaab")));
    println!("abba => {}", pattern.matches(String::from("abba")));
}
