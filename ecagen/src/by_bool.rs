use std::io::{self, Read, Write};
use std::cmp;

pub struct Row {
    offset: isize,
    cells: Vec<bool>,
}

/*
ex. new_from_single(1), we pad 3 cells left and right with 0; so total length is 7
index   < 0 1 2 3 4 5 6 >
    _ _ _ 0 0 0 1 0 0 0 _ _ _
            * * * * *
    _ _ h h h x x x t t t _ _
when generating the next row, we copy over the "head" and "tail" padding on the left and right
*/

impl Row {
    pub fn new() -> Row {
        Row::from_single(true)
    }
    pub fn from_single(a: bool) -> Row {
        let b = !a;
        Row {
            offset: -3,
            cells: vec![b, b, b, a, b, b, b]
        }
    }
    pub unsafe fn from_raw(offset: isize, cells: &[bool]) -> Row {
        Row {
            offset,
            cells: Vec::from(cells),
        }
    }
    pub fn from_state(state: &[bool]) -> Row {
        assert!(state.len() > 0);
        let left = state[0];
        let right = state[state.len()-1];
        let mut cells = Vec::with_capacity(3 + state.len() + 3);
        cells.extend([left, left, left]);
        cells.extend(state);
        cells.extend([right, right, right]);
        Row {
            offset: -(3 + state.len() as isize/2),
            cells
        }
    }
    pub fn get_state(&self) -> (isize, &[bool]) {
        (self.offset, &self.cells)
    }
    pub fn next(&self, rule: &Rule) -> Row {
        let mut next = vec![];
        let apply = |i| rule.apply(&[self.cells[i], self.cells[i+1], self.cells[i+2]]);
        let head = apply(0);
        next.push(head);
        next.push(head);
        next.push(head);
        for i in 1..self.cells.len()-3 {
            next.push(apply(i));
        }
        let tail = apply(self.cells.len()-3);
        next.push(tail);
        next.push(tail);
        next.push(tail);
        Row {
            offset: self.offset-2,
            cells: next
        }
    }
    pub fn compress(&mut self) {
        /*
            example:
                ... 1 2 3 4 5 6 7 8 9 >
                    _ _ 1 0 0 0 0 0 0
            back = 4      ^
            tail = 0
            truncate to 0..6 (length 7 == back + 3)
        */
        // truncate back
        let len = self.cells.len();
        let tail = self.cells[len-1];
        assert!(self.cells[len-2] == tail && self.cells[len-3] == tail);
        let mut back = len-3;
        while self.cells[back-1] == tail {
            back -= 1;
        }
        if back < len-3 {
            self.cells.truncate(back + 3)
        }
        /*
            example:
                < 0 1 2 3 4 5 6 7 8 9 ...
                  0 0 0 0 0 0 0 1 _ _ ...
            front = 6         ^
            head = 0
            move [7( == front+1)] to [3]
            n = 4( == front - 2)
            so start with i = 7, move to i - 4 ( == i - n)
        */
        // shift front
        let head = self.cells[0];
        assert!(self.cells[1] == head && self.cells[2] == head);
        let mut front = 2;
        while self.cells[front+1] == head {
            front += 1;
        }
        if front > 2 {
            let len = self.cells.len();
            let n = front - 2;
            for i in front+1..len {
                self.cells[i-n] = self.cells[i];
            }
            self.cells.truncate(len - n);
        }
    }
    pub fn export<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let end = self.cells.len();
        let mut buf = vec![];
        todo!("there's an issue with not 'padding out' the right most bits of the last byte.")
        for i in 0..end/8 {
            let mut b = 0;
            for n in 0..8 {
                b |= (self.cells[n + i * 8] as u8) << (7 - n);
            }
            buf.push(b);
        }
        let start = end - end % 8;
        if start != end {
            // first, finish remaining cells
            let m = start + 7;
            let mut b = 0;
            for n in start..end {
                b |= (self.cells[n] as u8) << (m - n);
            }
            // second, extend last cell
            let tail = self.cells[end-1] as u8;
            for n in end..m {
                b |= tail << (7 - (n - end));
            }
            buf.push(b);
        }

        // unused implementation
        // let mut i = 0;
        // while i < end {
        //     let mut b = 0;
        //     let stop = cmp::min(i + 8, end);
        //     for n in i..stop {
        //         b |= (self.cells[n] as u8) << (8 + i - n);
        //     }
        //     /*
        //         ex: cells = [x, x, x, x, x, x, x, x, x, x]
        //           i = 8;                             ^
        //         previous for loop went `n in 8..10`
        //           b = x x _ _ _ _ _ _
        //           i += 8;
        //         next for loop will go `n in 10..16`
        //         so left shift tail by ((end - i) - (n - end))
        //     */
        //     i += 8;
        //     if(stop == end) {
        //         let tail = self.cells[end-1] as u8
        //         for n in end..i {
        //             b |= tail << ((end - i) - (n - end));
        //         }
        //     }
        //     buf.push(b);
        // }

        w.write_all(&(self.offset as i64).to_be_bytes())?;
        // w.write(&(self.cells.len() as u64).to_be_bytes())?;
        w.write_all(&(buf.len() as u64).to_be_bytes())?;
        w.write_all(&buf)
    }
    // TODO import
    pub fn import<R: Read>(r: &mut R) -> io::Result<()> {
        // let mut offset = [u8; 8];
        // r.read_exact(&mut offset)?;
        // let offset = i64::from_be_bytes(offset);
        // let mut len = [u8; 8];
        // r.read_exact(&mut len)?;
        // let len = u64::from_be_bytes(len) as usize;
        // let mut buf = Vec::new();
        // buf.resize(len, 0);
        // r.read_exact(&mut buf)?;
        // ... convert [bytes] to [bool] ...
        todo!();
    }
}

pub struct Rule {
    pub wolfram_code: u8,
    // config: [bool; 8],
}

impl Rule {
    pub fn new(wolfram_code: u8) -> Rule {
        // let mut n = wolfram_code;
        // let mut config = [false; 8];
        // for i in 0..8 {
        //     config[i] = n & 1 == 1;
        //     n = n >> 1;
        // }
        Rule {
            wolfram_code,
            // config,
        }
    }
    pub fn apply(&self, i: &[bool; 3]) -> bool {
        let idx = (i[0] as u8) << 2 | (i[1] as u8) << 1 | i[2] as u8;
        if (self.wolfram_code << idx) & 1 == 0 {
            false
        } else {
            true
        }
    }
}

struct Automaton {
    rule: Rule,
    rows: Vec<Row>,
}

impl Automaton {
    fn new(wolfram_code: u8, init_row: Row) -> Automaton {
        Automaton {
            rule: Rule::new(wolfram_code),
            rows: vec![init_row],
        }
    }
    fn step(&mut self) {
        self.rows.push(self.rows.last().unwrap().next(&self.rule));
    }
}