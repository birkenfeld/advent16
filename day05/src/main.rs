extern crate crypto;
extern crate rayon;

use std::char;
use crypto::md5::Md5;
use crypto::digest::Digest;
use rayon::prelude::*;

const INPUT: &'static [u8] = b"uqwqemis";
const LEN: usize = 8;
const BATCH: usize = 1_000_000;

fn check(i: usize) -> Option<(usize, u8, u8)> {
    let mut buf = [0u8; 16];
    let mut hash = Md5::new();
    hash.input(INPUT);
    hash.input(format!("{}", i).as_bytes());
    hash.result(&mut buf);
    if buf[0] | buf[1] == 0 && buf[2] & 0xF0 == 0 {
        Some((i, buf[2], buf[3] >> 4))
    } else {
        None
    }
}

fn main() {
    // the first passcode can be collected directly
    let mut pass_door1 = String::new();
    // the second passcode needs to be constructed char by char
    let mut pass_door2 = vec![None; LEN];
    let mut n = 0;
    // we need more batches as long as not all pass_door2 digits are found
    // (pass_door1 is long done by then)
    while pass_door2.iter().any(|v| v.is_none()) {
        // get every candidate digit (with 00000 MD5 prefix) in the next batch
        let mut digits = (n..n + BATCH).into_par_iter()
                                       .filter_map(check)
                                       .fold(|| vec![], |mut v, x| { v.push(x); v })
                                       .reduce(|| vec![], |mut v, x| { v.extend(x); v });
        digits.sort();  // by n, then d6, then d7
        // update passcode for first door, just by order of number
        pass_door1.extend(digits.iter().map(|d| char::from_digit(d.1 as u32, 16).unwrap()));
        // update passcode for second door, where the index is d6, the code digit
        // is d7, and the first one wins
        for (_, d6, d7) in digits {
            let d6 = d6 as usize;
            if d6 < LEN && pass_door2[d6].is_none() {
                pass_door2[d6] = char::from_digit(d7 as u32, 16);
            }
        }
        n += BATCH;
    }
    let pass_door1 = &pass_door1[..8];
    let pass_door2 = pass_door2.into_iter().map(|x| x.unwrap()).collect::<String>();
    println!("First door: {}", pass_door1);
    println!("Second door: {}", pass_door2);
}
