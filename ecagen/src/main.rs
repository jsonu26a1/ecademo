// main ref https://en.wikipedia.org/wiki/Elementary_cellular_automaton

fn main() {
    println!("Hello, world!");
}

/*
currently using bool type, would like to switch to u8 or even u32 or u64, but I'm deathly
afraid of x86 vector ops, but I suppose those would give better runtime/space efficiency;
vector ops might not be usable for a generalized implementation, they might only work for
hand/hard coding each rule, which would be a very far stretch goal of this project.
*/

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
