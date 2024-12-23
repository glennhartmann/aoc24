use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
    ops::{Deref, DerefMut},
};

use aoclib_rs::{prep_io, printwriteln};

struct Graph<'a>(HashMap<&'a str, Node<'a>>);

impl<'a> Graph<'a> {
    fn new(contents: &Vec<(&'a str, &'a str)>) -> Graph<'a> {
        let mut g = Graph(HashMap::new());
        for connection in contents {
            let left = g.entry(connection.0).or_insert(Node {
                neighbours: HashSet::new(),
            });
            left.neighbours.insert(connection.1);

            let right = g.entry(connection.1).or_insert(Node {
                neighbours: HashSet::new(),
            });
            right.neighbours.insert(connection.0);
        }

        g
    }
}

impl<'a> Deref for Graph<'a> {
    type Target = HashMap<&'a str, Node<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Graph<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Node<'a> {
    neighbours: HashSet<&'a str>,
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 23).unwrap();
    let contents: Vec<(&str, &str)> = contents
        .iter()
        .map(|line| {
            let mut sp = line.split("-");
            (sp.next().unwrap(), sp.next().unwrap())
        })
        .collect();

    let g = Graph::new(&contents);
    part1(&mut writer, &g);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, g: &Graph) {
    let mut triplets: HashSet<Vec<&str>> = HashSet::new();
    for (name, node) in g.iter() {
        for neighbour_name in &node.neighbours {
            let neighbour_node = g.get(neighbour_name).unwrap();
            for neighbour2_name in &neighbour_node.neighbours {
                let neighbour2_node = g.get(neighbour2_name).unwrap();
                if neighbour2_node.neighbours.contains(name) {
                    let mut v = vec![*name, *neighbour_name, *neighbour2_name];
                    v.sort();
                    triplets.insert(v);
                }
            }
        }
    }

    let mut total = 0;
    for triplet in triplets {
        println!("{},{},{}", triplet[0], triplet[1], triplet[2]);

        if triplet[0].starts_with('t') || triplet[1].starts_with('t') || triplet[2].starts_with('t')
        {
            total += 1;
        }
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}
