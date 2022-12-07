use crate::io::read_lines;

pub fn day05() {
    if let Ok(mut input) = read_lines("../../../day05.txt") {
        let mut crate_stacks: Vec<Vec<char>> = vec![vec![]; 9];

        input.by_ref().take(8).for_each(|line| {
            let mut containers: Vec<char> = line.unwrap().chars().skip(1).step_by(4).collect();

            containers.resize(9, ' ');

            for i in 0..9 {
                if !containers[i].is_whitespace() {
                    crate_stacks[i].push(containers[i])
                }
            }
        });

        for crate_stack in &mut crate_stacks {
            crate_stack.reverse();
        }

        let instructions: Vec<Vec<usize>> = input
            .by_ref()
            .skip(2)
            .map(|line| {
                line.unwrap()
                    .split(' ')
                    .skip(1)
                    .step_by(2)
                    .map(|string| string.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        for crate_stack in &crate_stacks {
            for container in crate_stack {
                print!("{}", container)
            }
            println!()
        }

        for crate_stack in &instructions {
            for container in crate_stack {
                print!("{}", container)
            }
            println!()
        }

        operate_crane_mover_9000(crate_stacks.clone(), &instructions);
        operate_crane_mover_9001(crate_stacks, &instructions);
    }
}

fn operate_crane_mover_9000(mut container_stacks: Vec<Vec<char>>, instructions: &Vec<Vec<usize>>) {
    for instruction in instructions {
        let first_container = instruction[1] - 1;
        let second_container = instruction[2] - 1;

        for _ in 0..instruction[0] {
            let container = container_stacks[first_container].pop();

            if container.is_none() {
                break;
            } else {
                container_stacks[second_container].push(container.unwrap());
            }
        }
    }

    for container_stack in container_stacks {
        print!("{}", container_stack.last().unwrap())
    }
    println!()
}

fn operate_crane_mover_9001(mut container_stacks: Vec<Vec<char>>, instructions: &Vec<Vec<usize>>) {
    for instruction in instructions {
        let first_container = instruction[1] - 1;
        let second_container = instruction[2] - 1;

        let final_crates_moved = container_stacks[first_container]
            .len()
            .saturating_sub(instruction[0]);
        let mut containers = container_stacks[first_container].split_off(final_crates_moved);

        container_stacks[second_container].append(&mut containers);
    }

    for container_stack in container_stacks {
        print!("{}", container_stack.last().unwrap_or(&' '))
    }
    println!()
}
