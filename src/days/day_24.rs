use std::{
    collections::{HashMap, HashSet},
    fmt,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln};

use {once_cell::sync::Lazy, regex::Regex};

struct Graph {
    circuits: HashMap<String, Node>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            circuits: HashMap::new(),
        }
    }

    fn add_circuits(&mut self, cs: &Vec<Circuit>) {
        for c in cs {
            self.circuits.insert(c.3.clone(), Node::new(c));
        }
    }

    fn add_initial_vals(&mut self, iv: &HashMap<&str, u8>) {
        for (n, v) in iv {
            self.circuits
                .entry(n.to_string())
                .and_modify(|e| e.val = Some(*v))
                .or_insert(Node::with_val(*v));
        }
    }

    fn unfinished(&self) -> HashSet<String> {
        let mut hs = HashSet::new();
        for (n, c) in &self.circuits {
            if c.val.is_none() {
                hs.insert(n.clone());
            }
        }

        hs
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (n, c) in &self.circuits {
            writeln!(
                f,
                "{}: ({:?}) {:?} {:?} {:?}",
                n, c.val, c.input1, c.op, c.input2
            )?;
        }
        Ok(())
    }
}

struct Node {
    val: Option<u8>,
    input1: Option<String>,
    op: Option<Op>,
    input2: Option<String>,
}

impl Node {
    fn new(c: &Circuit) -> Node {
        Node {
            val: None,
            input1: Some(c.0.clone()),
            op: Some(c.1),
            input2: Some(c.2.clone()),
        }
    }

    fn with_val(v: u8) -> Node {
        Node {
            val: Some(v),
            input1: None,
            op: None,
            input2: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn from_str(s: &str) -> Op {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("invalid op: {}", s),
        }
    }

    fn perform(&self, u1: u8, u2: u8) -> u8 {
        match self {
            Op::And => u1 & u2,
            Op::Or => u1 | u2,
            Op::Xor => u1 ^ u2,
        }
    }
}

type Circuit = (String, Op, String, String);

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 24).unwrap();

    let mut initial_vals: HashMap<&str, u8> = HashMap::new();

    let mut i = 0;
    let mut line = contents[i];
    while !line.is_empty() {
        let mut sp = line.split(": ");
        initial_vals.insert(sp.next().unwrap(), sp.next().unwrap().parse().unwrap());

        i += 1;
        line = contents[i];
    }

    let mut circuits: Vec<Circuit> = Vec::new();
    for line in &contents[i + 1..] {
        circuits.push(parse_circuit_line(line));
    }

    let mut g = Graph::new();
    g.add_circuits(&circuits);
    g.add_initial_vals(&initial_vals);
    println!("{:?}", g);

    part1(&mut writer, &mut g);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, g: &mut Graph) {
    let mut unfinished = g.unfinished();
    while !unfinished.is_empty() {
        let mut finished = HashSet::new();
        for u in &unfinished {
            let un = g.circuits.get(&u.to_string()).unwrap();
            let i1v = g.circuits.get(&un.input1.clone().unwrap()).unwrap().val;
            let i2v = g.circuits.get(&un.input2.clone().unwrap()).unwrap().val;

            if i1v.is_some() && i2v.is_some() {
                let un = g.circuits.get_mut(&u.to_string()).unwrap();
                un.val = Some(un.op.unwrap().perform(i1v.unwrap(), i2v.unwrap()));
                finished.insert(u.clone());
            }
        }

        for f in finished {
            unfinished.remove(&f);
        }
    }

    let mut v = Vec::new();
    let mut pv = Vec::new();
    for (n, c) in &g.circuits {
        if n.starts_with('z') {
            v.push((n, c.val.unwrap()));
        }
        pv.push(format!("{}: {}", n, c.val.unwrap()));
    }
    v.sort();
    pv.sort();

    for s in pv {
        println!("{}", s);
    }

    let mut n: u64 = 0;
    for (i, (_, b)) in v.iter().enumerate() {
        n |= (*b as u64) << i;
    }

    printwriteln!(writer, "part 1: {}", n).unwrap();
}

fn parse_circuit_line(line: &str) -> Circuit {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([^ ]+) ([^ ]+) ([^ ]+) -> (.+)").unwrap());
    let mut caps = RE.captures_iter(line);
    let cap = caps.next().unwrap();

    (
        cap[1].to_owned(),
        Op::from_str(&cap[2]),
        cap[3].to_owned(),
        cap[4].to_owned(),
    )
}
