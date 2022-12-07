use crate::io::read_lines;

pub fn day02() {
    calculate_strategy_guide_score_2()
}

fn get_round_score(own_symbol: char, opponent_symbol: char) -> i32 {
    let mut final_score = 0;

    match own_symbol {
        'X' => final_score += 1,
        'Y' => final_score += 2,
        'Z' => final_score += 3,
        _ => {}
    }

    match own_symbol as u32 - opponent_symbol as u32 {
        22 | 25 => final_score += 0,
        23 => final_score += 3,
        21 | 24 => final_score += 6,
        _ => println!("{}", own_symbol as u32 - opponent_symbol as u32),
    }

    final_score
}

fn calculate_own_symbol(opponent_symbol: char, round_result: char) -> char {
    let mut own_symbol = 'X';

    match round_result {
        'X' => {
            if opponent_symbol == 'A' {
                own_symbol = 'Z'
            } else {
                own_symbol = char::try_from(opponent_symbol as u32 + 22).unwrap()
            }
        }
        'Y' => own_symbol = char::try_from(opponent_symbol as u32 + 23).unwrap(),
        'Z' => {
            if opponent_symbol == 'C' {
                own_symbol = 'X'
            } else {
                own_symbol = char::try_from(opponent_symbol as u32 + 24).unwrap()
            }
        }
        _ => {}
    }
    own_symbol
}

fn calculate_strategy_guide_score() {
    let mut final_score = 0;

    if let Ok(lines) = read_lines("../../../day02.txt") {
        for line_buffer in lines {
            if let Ok(line) = line_buffer {
                let line_chars: Vec<char> = line.chars().collect();
                final_score += get_round_score(line_chars[2], line_chars[0])
            }
        }
    }

    println!("{}", final_score)
}

fn calculate_strategy_guide_score_2() {
    let mut final_score = 0;

    if let Ok(lines) = read_lines("../../../day02.txt") {
        for line_buffer in lines {
            if let Ok(line) = line_buffer {
                let mut line_chars: Vec<char> = line.chars().collect();
                line_chars[2] = calculate_own_symbol(line_chars[0], line_chars[2]);
                final_score += get_round_score(line_chars[2], line_chars[0])
            }
        }
    }

    println!("{}", final_score)
}
