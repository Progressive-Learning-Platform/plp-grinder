use std::process::Command;
use std::vec::Vec;
use tokens::*;

pub fn slice_of(string: &str, range: (usize, usize)) -> String
{
    let native_slice = &string[range.0..range.1];
    let mut slice = String::new();
    slice.push_str(native_slice);
    slice
}

/// Identifies the end index of a nestable body given the open and close symbols for the body
/// For instance, an arithmetic expression can have nested parenthesis groups (e.g. (1 + (2*(2+1))) * (2 + 5) )
/// In the example, this method would identify the start and stop as these (> and < respectively):
///     >(1 + (2*(2+1)))< * (2 + 5)
/// If the start_index was specified as the parenthesis surrounding 2 + 5, then this method would select:
///     (1 + (2*(2+1))) * >(2 + 5)<
///
/// The start index passed to this method should be the index AFTER the first open symbol
/// @return the index of the token that closes the specified body or None if the block is not closed or nested properly
pub fn identify_body_bounds(tokens: &Vec<Token>, start: usize, symbols: (&str, &str)) -> Option<usize>
{
    let (open, close) = symbols;
    let mut open_braces = 1;

    for(index, token) in tokens[start..].iter().enumerate()
    {
        if token.value == open
        {
            open_braces += 1;
        }
        else if token.value == close
        {
            open_braces -= 1;
            if open_braces == 0
            {
                return Some(index + start);
            }
        }
    }

    return None;
}

/// Identifies the index of the next token matching the specified symbol
/// @return the index of the specified token (after the specified start index) or None if no such symbol exists
pub fn find_next(tokens: &Vec<Token>, start: usize, symbol: &str) -> Option<usize>
{
    for (index, token) in tokens[start..].iter().enumerate()
    {
        if token.value == symbol { return Some(index + start); }
    }

    return None;
}

pub fn execute_process(args: &[&str]) -> bool
{
    let output = Command::new(args[0])
                    .args(&args[1..])
                    .output()
                    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let was_successful: bool = output.status.success();

    if was_successful
    {
        return true;
    }
    else
    {
        println!("{} stdout: {}", args[0], String::from_utf8_lossy(&output.stdout));
        println!("{} stderr: {}", args[0], String::from_utf8_lossy(&output.stderr));
    }
    println!("\n");

    false
}
