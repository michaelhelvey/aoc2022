#![allow(dead_code)]

use anyhow::Result;
use core::str::Lines;

pub fn run_part_1() -> Result<()> {
    let lines = get_input_lines();

    let mut largest = 0;
    let mut current_elf_value = 0;

    for line in lines {
        if line.is_empty() {
            if current_elf_value > largest {
                largest = current_elf_value;
            }

            current_elf_value = 0;
            continue;
        }

        let value: i32 = line.parse()?;
        current_elf_value += value;
    }

    println!("Result: {} calories", largest);

    Ok(())
}

struct MaxSet<const L: usize> {
    len: usize,
    sizes: [i32; L],
}

impl<const L: usize> MaxSet<L> {
    fn new() -> Self {
        Self {
            len: L,
            sizes: [0; L],
        }
    }

    fn push(self: &mut Self, input: i32) {
        let mut idx = 0;

        for size in self.sizes {
            if input > size {
                // demote current value by pushing it down the set
                let mut curr = self.sizes[idx];
                self.sizes[idx] = input;

                let mut n_idx = idx + 1;
                while n_idx <= L - 1 {
                    std::mem::swap(&mut self.sizes[n_idx], &mut curr);
                    n_idx += 1;
                }

                break;
            }
            idx += 1;
        }
    }
}

pub fn run_part_2() -> Result<()> {
    let lines = get_input_lines();

    let mut current = 0;
    const ELF_NUM: usize = 3;
    let mut set: MaxSet<ELF_NUM> = MaxSet::new();

    for line in lines {
        if line.is_empty() {
            // elf boundary
            set.push(current);
            current = 0;
            continue;
        }

        let parsed_value: i32 = String::from(line).parse()?;
        current += parsed_value;
    }

    println!(
        "The {} largest:\n{}",
        ELF_NUM,
        set.sizes.map(|x| x.to_string()).join("\n")
    );

    println!("Total: {}", set.sizes.iter().sum::<i32>());

    Ok(())
}

fn get_input_lines() -> Lines<'static> {
    include_str!("./inputs/day_1.txt").lines()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_set() {
        let mut set: MaxSet<3> = MaxSet::new();
        set.push(123);
        assert_eq!(set.sizes, [123, 0, 0]);

        set.push(234);
        assert_eq!(set.sizes, [234, 123, 0]);

        set.push(5);
        assert_eq!(set.sizes, [234, 123, 5]);

        set.push(124);
        assert_eq!(set.sizes, [234, 124, 123]);
    }
}
