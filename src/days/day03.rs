use crate::io::read_lines;

use std::fs::File;
use std::io::{BufReader, Lines};

pub fn day03() {
    if let Ok(input) = read_lines("../../../day03.txt") {
        part_01(input);
    }

    if let Ok(input) = read_lines("../../../day03.txt") {
        part_02(input);
    }
}

fn part_01(input: Lines<BufReader<File>>) {
    let mut priority_sum: u32 = 0;

    for line_buffer in input {
        if let Ok(rucksack) = line_buffer {
            let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
            let common_item = find_common_item(compartment_1, compartment_2);
            priority_sum += get_priority(common_item);
        }
    }

    println!("{}", priority_sum);
}

fn part_02(mut input: Lines<BufReader<File>>) {
    let mut priority_sum: u32 = 0;

    while let Ok(rucksack_group) = input.next_chunk::<3>() {
        let common_item = find_badge(rucksack_group.map(|line| line.unwrap()));
        priority_sum += get_priority(common_item);
    }

    println!("{}", priority_sum);
}

fn find_common_item(compartment_1: &str, compartment_2: &str) -> char {
    for item in compartment_1.chars() {
        if !compartment_2.find(item).is_none() {
            return item;
        }
    }

    'a'
}

fn find_badge(p0: [String; 3]) -> char {
    for item in p0[0].chars() {
        if !p0[1].find(item).is_none() && !p0[2].find(item).is_none() {
            return item;
        }
    }

    'a'
}

fn get_priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        return item as u32 - 38;
    }

    item as u32 - 96
}
