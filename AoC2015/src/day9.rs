use std::collections::HashMap;

struct Graph<'a> {
    nodes: Vec<&'a str>,
    edges: HashMap<(&'a str, &'a str), u16>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    fn permutations<'b>(collection: &Vec<&'b str>) -> Vec<Vec<&'b str>> {
        if collection.len() == 1 {
            return vec![vec![collection[0]]];
        }
        let mut result = vec![];
        for el in collection {
            for tail in
                Self::permutations(&collection.iter().filter(|x| *x != el).map(|x| *x).collect())
            {
                let mut whole = vec![*el];
                whole.extend(tail);
                result.push(whole.clone())
            }
        }
        return result;
    }

    fn cheapest(&self) -> u16 {
        Graph::permutations(self.nodes.clone().as_ref())
            .iter()
            .map(|x| {
                let mut last = "";
                x.iter().fold(0, |tot, node| {
                    if last != "" && last != *node {
                        let price = &self.edges[&(last, *node)];
                        last = node;
                        tot + price
                    } else {
                        last = node;
                        tot
                    }
                })
            })
            .min()
            .unwrap()
    }

    fn costliest(&self) -> u16 {
        Graph::permutations(self.nodes.clone().as_ref())
            .iter()
            .map(|x| {
                let mut last = "";
                x.iter().fold(0, |tot, node| {
                    if last != "" && last != *node {
                        let price = &self.edges[&(last, *node)];
                        last = node;
                        tot + price
                    } else {
                        last = node;
                        tot
                    }
                })
            })
            .max()
            .unwrap()
    }
}

fn input_to_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    input.lines().for_each(|line| {
        let mut w = line.split_whitespace();
        let s = w.next().unwrap();
        w.next();
        let d = w.next().unwrap();
        w.next();
        let l = w.next().unwrap().to_string().parse::<u16>().unwrap();
        if !graph.nodes.contains(&s) {
            graph.nodes.push(s.clone());
        }
        if !graph.nodes.contains(&d) {
            graph.nodes.push(d.clone());
        }
        graph.edges.insert((s, d), l);
        graph.edges.insert((d, s), l);
    });
    graph
}

#[aoc(day9, part1)]
pub fn part1<'a>(input: &str) -> u16 {
    input_to_graph(input).cheapest()
}

#[aoc(day9, part2)]
pub fn part2<'a>(input: &str) -> u16 {
    input_to_graph(input).costliest()
}
