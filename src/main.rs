use std::io::{self, Read};
type LoopLut = Vec<(usize, usize)>;
const MEMORY_SIZE: usize = 256;
type Memory = [u8; MEMORY_SIZE];

#[derive(Debug, PartialEq)]
enum Error {
    MismatchedBrackets(usize), // Contains the index of the problematic character
                               // Add other errors here if needed
}

fn generate_loop_lookup_table(source_code: &str) -> Result<LoopLut, Error> {
    let mut loop_lut = LoopLut::new();
    let mut bracket_stack = Vec::new();
    for (index, character) in source_code.chars().enumerate() {
        match character {
            '[' => bracket_stack.push(index),
            ']' => {
                let index_of_opening_bracket = bracket_stack
                    .last()
                    .copied()
                    .ok_or(Error::MismatchedBrackets(index))?;
                bracket_stack.pop();
                loop_lut.push((index_of_opening_bracket, index));
            }
            _ => {}
        }
    }
    if let Some(index) = bracket_stack.last() {
        return Err(Error::MismatchedBrackets(*index));
    }
    Ok(loop_lut)
}

fn increment_memory_pointer(memory_pointer: usize) -> usize {
    if memory_pointer < MEMORY_SIZE {
        return memory_pointer + 1;
    } else {
        return 0;
    }
}

fn decrement_memory_pointer(memory_pointer: usize) -> usize {
    if memory_pointer > 0 {
        return memory_pointer - 1;
    } else {
        return MEMORY_SIZE - 1;
    }
}

fn run(source_code: &String) -> Result<(), Error> {
    let loop_lut = generate_loop_lookup_table(source_code)?;
    let mut memory: Memory = [0; MEMORY_SIZE];
    let mut memory_pointer: usize = 0;
    let mut source_pointer: usize = 0;

    println!(""); // Add a newline for aesthetics
    while source_pointer < source_code.len() {
        let character = source_code.chars().nth(source_pointer).unwrap();
        match character {
            '>' => memory_pointer = increment_memory_pointer(memory_pointer),
            '<' => memory_pointer = decrement_memory_pointer(memory_pointer),
            '+' => memory[memory_pointer] += 1,
            '-' => memory[memory_pointer] -= 1,
            '.' => print!("{}", memory[memory_pointer] as char),
            ',' => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                memory[memory_pointer] = input.as_bytes()[0];
            }
            '[' => {
                if memory[memory_pointer] == 0 {
                    source_pointer = loop_lut
                        .iter()
                        .find(|(open_idx, _)| *open_idx == source_pointer)
                        .map(|(_, close_idx)| *close_idx)
                        .ok_or(Error::MismatchedBrackets(source_pointer))?;
                }
            }
            ']' => {
                if memory[memory_pointer] != 0 {
                    source_pointer = loop_lut
                        .iter()
                        .find(|(_, close_idx)| *close_idx == source_pointer)
                        .map(|(open_idx, _)| *open_idx)
                        .ok_or(Error::MismatchedBrackets(source_pointer))?;
                }
            }
            _ => {}
        }
        source_pointer += 1;
    }
    println!(""); // Add a newline for aesthetics
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_loop_lookup_table() {
        let source_code = "[[]]";
        let result = generate_loop_lookup_table(source_code).unwrap();
        assert_eq!(result, vec![(1, 2), (0, 3)]);

        let source_code2 = "[[[]]]";
        let result2 = generate_loop_lookup_table(source_code2).unwrap();
        assert_eq!(result2, vec![(2, 3), (1, 4), (0, 5)]);

        let source_code3 = "[]]";
        let result3 = generate_loop_lookup_table(source_code3);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), Error::MismatchedBrackets(2));
    }
}

fn truncate_string(s: &String, a: usize, b: usize) -> String {
    let mut s = s.clone();
    s.drain(..a).for_each(drop);
    s.drain(b..).for_each(drop);
    return s;
}

fn display_lut_error(error: Error, source_code: &String) {
    println!("\n\nSorry! Your Brainfuck program experienced a runtime error!");
    match error {
        Error::MismatchedBrackets(index) => {
            let start_index = std::cmp::max(0, index as i32 - 10) as usize;
            let end_index = std::cmp::min(source_code.len() - 1, index + 10);

            let trimmed_code = truncate_string(&source_code, start_index, end_index);

            let is_left_trimmed = start_index > 0;
            let is_right_trimmed = end_index < source_code.len() - 1;
            let caret_index = if is_left_trimmed {
                10
            } else {
                index - start_index
            };
            let caret = if is_right_trimmed {
                format!("{}^", " ".repeat(caret_index))
            } else {
                format!("{}^", " ".repeat(caret_index))
            };

            println!("{}", trimmed_code);
            println!("{}", caret);

            println!(
                "The closing bracket at index {} does not have a matching opening bracket",
                index,
            );
        }
    }
}

fn sanitize_input(input: &String) -> String {
    let mut sanitized_input = String::new();
    for character in input.chars() {
        match character {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => sanitized_input.push(character),
            _ => {}
        }
    }
    return sanitized_input;
}

fn main() {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).unwrap();
    let buffer = String::from_utf8(buffer).unwrap();
    let buffer = sanitize_input(&buffer);

    let res = run(&buffer);
    if res.is_err() {
        display_lut_error(res.unwrap_err(), &buffer);
    }
}
