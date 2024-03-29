use std::collections::HashMap;

type PolymerTemplate = Vec<char>;
type Element = char;

pub fn input_generator(input: &str) -> (PolymerTemplate, HashMap<(Element, Element), Element>) {
    let mut split = input.split("\n\n");

    (
        split.next().unwrap().chars().collect(),
        split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut chars = l.chars();
                (
                    (chars.next().unwrap(), chars.next().unwrap()),
                    chars.last().unwrap(),
                )
            })
            .collect(),
    )
}

fn count_elements_and_pairs(
    polymer_template: &[Element],
) -> (HashMap<Element, usize>, HashMap<(Element, Element), usize>) {
    let mut element_counts: HashMap<Element, usize> = HashMap::new();
    let mut pair_counts: HashMap<(Element, Element), usize> = HashMap::new();

    polymer_template.windows(2).for_each(|pair| {
        element_counts
            .entry(pair[0])
            .and_modify(|count| *count += 1)
            .or_insert(1);
        pair_counts
            .entry((pair[0], pair[1]))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    element_counts
        .entry(*polymer_template.last().unwrap())
        .and_modify(|count| *count += 1)
        .or_insert(1);

    (element_counts, pair_counts)
}

fn make_insertion_step(
    pair_counts: &mut HashMap<(Element, Element), usize>,
    element_counts: &mut HashMap<Element, usize>,
    pair_insertion_rules: &HashMap<(Element, Element), Element>,
) {
    let mut new_pair_counts = pair_counts.clone();

    for (pair, inserted_element) in pair_insertion_rules {
        if let Some(pair_count) = pair_counts.get(pair) {
            let count = pair_count;

            new_pair_counts.entry(*pair).and_modify(|e| *e -= *count);

            element_counts
                .entry(*inserted_element)
                .and_modify(|e| *e += *count)
                .or_insert(*count);

            new_pair_counts
                .entry((pair.0, *inserted_element))
                .and_modify(|e| *e += *count)
                .or_insert(*count);
            new_pair_counts
                .entry((*inserted_element, pair.1))
                .and_modify(|e| *e += *count)
                .or_insert(*count);
        }
    }

    *pair_counts = new_pair_counts;
}

fn make_n_steps(
    polymer_template: &[Element],
    pair_insertion_rules: &HashMap<(Element, Element), Element>,
    steps: usize,
) -> usize {
    let (mut element_counts, mut pair_counts) = count_elements_and_pairs(polymer_template);

    (0..steps).for_each(|_| {
        make_insertion_step(&mut pair_counts, &mut element_counts, pair_insertion_rules)
    });

    element_counts.values().max().unwrap() - element_counts.values().min().unwrap()
}

pub fn solve_part1(
    (polymer_template, pair_insertion_rules): &(
        PolymerTemplate,
        HashMap<(Element, Element), Element>,
    ),
) -> usize {
    make_n_steps(polymer_template, pair_insertion_rules, 10)
}

pub fn solve_part2(
    (polymer_template, pair_insertion_rules): &(
        PolymerTemplate,
        HashMap<(Element, Element), Element>,
    ),
) -> usize {
    make_n_steps(polymer_template, pair_insertion_rules, 40)
}
