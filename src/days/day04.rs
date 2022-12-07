use crate::io::read_lines;
use std::collections::HashSet;

pub fn day04() {
    if let Ok(input) = read_lines("../../../day04.txt") {
        let lol: Vec<Vec<HashSet<u32>>> = input
            .map(|line| {
                line.unwrap()
                    .split(',')
                    .map(|x| {
                        x.split('-')
                            .map(|x1| x1.parse::<u32>().unwrap())
                            .next_chunk::<2>()
                            .map(|t| HashSet::from_iter(t[0]..t[1] + 1))
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        part_01(&lol);
        part_02(&lol);
    }
}

fn part_01(input: &Vec<Vec<HashSet<u32>>>) {
    let complete_overlaps: Vec<&Vec<HashSet<u32>>> = input
        .iter()
        .filter(|section_assignment_pair| do_completely_overlap(*section_assignment_pair))
        .collect();

    let total_complete_overlaps = complete_overlaps.len();

    println!("{}", total_complete_overlaps)
}

fn part_02(input: &Vec<Vec<HashSet<u32>>>) {
    let mut total_complete_overlaps = 0;

    for section_assignment_pair in input {
        let section_assignment_intersection: HashSet<&u32> = section_assignment_pair[0]
            .intersection(&section_assignment_pair[1])
            .collect();

        if 0 < section_assignment_intersection.len() {
            total_complete_overlaps += 1;
        }
    }

    println!("{}", total_complete_overlaps)
}

fn do_completely_overlap(section_assignment_pair: &Vec<HashSet<u32>>) -> bool {
    let intersections: HashSet<&u32> = section_assignment_pair[0]
        .intersection(&section_assignment_pair[1])
        .collect();
    if intersections.len() == section_assignment_pair[0].len()
        || intersections.len() == section_assignment_pair[1].len()
    {
        return true;
    }

    false
}
