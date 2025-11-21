use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::time::{Instant, Duration};
use std::iter;

use by_bool::{Rule, Row};

mod by_bool;
// main ref https://en.wikipedia.org/wiki/Elementary_cellular_automaton

fn main() {
    println!("Hello, world!");
    test_pattern_01().unwrap();
}

fn test_pattern_01() -> io::Result<()> {
    let fout = File::create("../ecaview/www/test_pattern_01.bin")?;
    let mut w = BufWriter::new(fout);
    for i in 0..8 {
        let row = Row::from_single(i % 2 == 1);
        row.export(&mut w)?;
    }
    Ok(())
}

fn test_pattern_02() -> io::Result<()> {
    let fout = File::create("../ecaview/www/test_pattern_02.bin")?;
    let mut w = BufWriter::new(fout);
    for i in 0..512 {
        let v = i % 2 == 1;
        let p = i % 13;
        let mut cells: Vec<bool> = vec![];
        while cells.len() < 512 {
            iter::repeat_n(v, p);
            iter::repeat_n(!v, p);
        }
        todo!();
        // let row = rows::
    }
    Ok(())
}

fn test_run_02() -> io::Result<()> {
    let fout = File::create("../ecaview/www/test_run_02.bin")?;
    // let fout = File::create("test_run_01.bin")?;
    let mut w = BufWriter::new(fout);
    // let mut w: Vec<u8> = vec![];

    let mut now = Instant::now();

    let rule = Rule::new(110);
    let mut row = Row::new();
    row.export(&mut w)?;

    for i in 0..512 {
        row = row.next(&rule);
        row.export(&mut w)?;

        let _now = Instant::now();
        println!("{} {:?}", i, now - _now);
        now = _now;
    }
    Ok(())
}

/*
currently using bool type, would like to switch to u8 or even u32 or u64, but I'm deathly
afraid of x86 vector ops, but I suppose those would give better runtime/space efficiency;
vector ops might not be usable for a generalized implementation, they might only work for
hand/hard coding each rule, which would be a very far stretch goal of this project.
*/


/*
struct Row {
    offset: isize,
    len: usize,
    left: bool,
    center: Vec<bool>,
    right: bool,
}

struct Rule {
    wolfram_code: u8,
    config: [bool; 8],
}

impl Rule {
    fn new(wolfram_code: u8) -> Rule {
        let mut n = wolfram_code;
        let mut config = [false; 8];
        for i in 0..8 {
            config[i] = n & 1 == 1;
            n = n >> 1;
        }
        Rule {
            wolfram_code,
            config,
        }
    }
}

struct Automaton {
    rule: Rule,
    rows: Vec<Row>,
}
*/

// trait Automaton {
//     fn from_code(code: u8) -> Self;
// }