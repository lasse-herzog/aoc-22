use crate::io::read_lines;

pub fn day01() {
    /*
    DAYS.lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .insert("day01", day01 as fn() -> ());
    */

    get_most_calories();
    get_most_accumulated_calories();
}

fn get_most_accumulated_calories() {
    let mut accumulated_calories_by_top_three_elf = vec![0, 0, 0];

    if let Ok(lines) = read_lines("./day01.txt") {
        let mut calories_carried_by_current_elf = 0;

        for line_buffer in lines {
            if let Ok(line) = line_buffer {
                if line.is_empty() {
                    for i in 0..2 {
                        if calories_carried_by_current_elf
                            > accumulated_calories_by_top_three_elf[i]
                        {
                            accumulated_calories_by_top_three_elf[i] =
                                calories_carried_by_current_elf;
                            break;
                        }

                        accumulated_calories_by_top_three_elf.sort();
                    }

                    calories_carried_by_current_elf = 0;
                } else {
                    calories_carried_by_current_elf += line.parse::<i32>().unwrap();
                }
            }
        }
    }

    println!(
        "{}",
        accumulated_calories_by_top_three_elf.iter().sum::<i32>()
    );
}

fn get_most_calories() {
    let mut most_calories_carried_by_elf = 0;

    if let Ok(lines) = read_lines("./day01.txt") {
        let mut calories_carried_by_current_elf = 0;

        for line_buffer in lines {
            if let Ok(line) = line_buffer {
                if line.is_empty() {
                    if calories_carried_by_current_elf > most_calories_carried_by_elf {
                        most_calories_carried_by_elf = calories_carried_by_current_elf;
                    }

                    calories_carried_by_current_elf = 0;
                } else {
                    calories_carried_by_current_elf += line.parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("{}", most_calories_carried_by_elf)
}
