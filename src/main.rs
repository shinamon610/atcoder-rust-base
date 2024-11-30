// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }
    println!("{}", "Yes");
}

fn solve() {
    todo!()
}

fn to_yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
