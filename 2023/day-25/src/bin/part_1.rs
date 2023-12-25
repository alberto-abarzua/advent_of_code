use rand::Rng;
use std::collections::HashSet;

fn main() {
    println!("Solution: {}", solution(include_str!("../input.txt")));
}

type Graph = (V, E);

type V = HashSet<String>;
type E = Vec<(String, String)>;

fn parse_input(input: &str) -> Graph {
    let mut vertices = HashSet::new();
    let mut edges = Vec::new();
    for line in input.lines() {
        let (node, neighbours) = line.split_once(":").unwrap();
        let neighbours = neighbours.split_whitespace().collect::<Vec<_>>();
        vertices.insert(node.to_string());
        for neighbour in neighbours {
            edges.push((node.to_string(), neighbour.to_string()));
        }
    }

    (vertices, edges)
}

fn solution(input: &str) -> usize {
    let (vertices, edges) = parse_input(input);

    //Karger's algorithm
    loop {
        let mut vertices = vertices.clone();
        let mut edges = edges.clone();
        while vertices.len() > 2 {
            let i = rand::thread_rng().gen_range(0..edges.len());
            let (v1, v2) = edges[i].clone();

            edges.swap_remove(i);
            vertices.remove(&v1);
            vertices.remove(&v2);

            let new_v = format!("{}:{}", v1, v2);
            vertices.insert(new_v.clone());

            for (e1, e2) in edges.iter_mut() {
                if *e1 == v1 || *e1 == v2 {
                    *e1 = new_v.clone()
                }
                if *e2 == v1 || *e2 == v2 {
                    *e2 = new_v.clone()
                }
            }

            let mut j = 0;
            while j < edges.len() {
                let (e1, e2) = &edges[j];
                if e1 == e2 {
                    edges.swap_remove(j);
                } else {
                    j += 1;
                }
            }
        }
        if edges.len() == 3 {
            break vertices
                .iter()
                .map(|s| s.split(':').count())
                .product::<usize>();
        }
    }
}
