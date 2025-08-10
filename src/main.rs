use util::{self, read_file_to_lines};

fn main() {
    let lines = read_file_to_lines("input.txt");
    let mut counter = 0;
    for line in lines {
        if is_line_valid(line) {
            counter += 1;
        }
    }
    print!("{}", counter)
}

fn is_line_valid(line : String) -> bool {
    let [rules, content]: [&str; 2] = line.split(":").collect::<Vec<&str>>().try_into().unwrap_or_default();
    println!("{}, {}", rules, content);
    let [range, character]: [&str; 2] = rules.split_whitespace().collect::<Vec<&str>>().try_into().unwrap_or_default();
    let [lower_range_s, upper_range_s]: [&str; 2] = range.split("-").collect::<Vec<&str>>().try_into().unwrap_or_default();
    let lower_range = lower_range_s.trim().parse::<i32>().unwrap_or(0);
    let upper_range = upper_range_s.trim().parse::<i32>().unwrap_or(0);
    let mut count = 0;
    for c in content.chars() {
        if c == character.chars().next().unwrap() {
            count += 1;
        }
    }
    let result = (lower_range <= count) && (count <= upper_range);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_line_valid(){
        let test_line: String =String::from("1-3 a: asdfg");
        let result = is_line_valid(test_line);
        assert_eq!(result, true)
    }
    #[test]
    fn test_is_line_invalid(){
        let test_line: String =String::from("1-3 a: aaaasdfg");
        let result = is_line_valid(test_line);
        assert_eq!(result, false)
    }
}