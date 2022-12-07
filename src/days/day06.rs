use crate::io::read_lines;
use std::collections::HashSet;

pub fn day06() {
    if let Ok(input) = read_lines("../../../day06.txt") {
        let datastream = input.last().unwrap().unwrap();
        let lol = datastream
            .as_bytes()
            .windows(4)
            .take_while(|x| {
                let x1: [u8; 4] = (*x).try_into().unwrap();
                HashSet::<u8>::from(x1).len() != 4
            })
            .count()
            + 4;

        let lul = datastream
            .as_bytes()
            .windows(14)
            .take_while(|x| {
                let x1: [u8; 14] = (*x).try_into().unwrap();
                HashSet::<u8>::from(x1).len() != 14
            })
            .count()
            + 14;
        println!("{}", lul);
    }
}
