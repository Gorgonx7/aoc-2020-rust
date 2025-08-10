use std::fs;



pub fn read_file_to_lines(file_name :&str) -> Vec<String> { 
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");
    contents
        .lines()
        .map(|line| line.to_string())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_to_lines() {
        use std::io::Write;
        let test_file = "test_input.txt";
        let mut file = std::fs::File::create(test_file).unwrap();
        writeln!(file, "line1\nline2\nline3").unwrap();

        let lines = read_file_to_lines(test_file);
        assert_eq!(lines, vec!["line1", "line2", "line3"]);

        std::fs::remove_file(test_file).unwrap();
    }
}
