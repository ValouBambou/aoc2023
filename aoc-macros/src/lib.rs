extern crate proc_macro;
use proc_macro::TokenStream;
use std::fmt::Write;

#[proc_macro]
pub fn days_modules(_item: TokenStream) -> TokenStream {
    let mut buf = String::new();
    for n in 1..=25 {
        writeln!(&mut buf, "mod day{n:02};").unwrap();
        writeln!(&mut buf, "use day{n:02}::Day{n};").unwrap();
    }
    buf.parse().unwrap()
}

fn match_func(func: &str) -> String {
    let mut buf = format!("match n {{\n");
    for n in 1..=25 {
        writeln!(&mut buf, "    {n} => Day{n}::{func}(),").unwrap();
    }
    writeln!(&mut buf, "        _ => panic!(\"wtf invalid day number\"),").unwrap();
    buf.push_str("    }\n");
    buf
}

#[proc_macro]
pub fn match_test_days(_item: TokenStream) -> TokenStream {
    match_func("test").parse().unwrap()
}

#[proc_macro]
pub fn match_time_days(_item: TokenStream) -> TokenStream {
    match_func("time").parse().unwrap()
}
