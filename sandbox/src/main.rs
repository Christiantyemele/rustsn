fn main() {}
fn solution() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let expected = "hello world\n"; // Note the newline character 
        let mut output = Vec::new();

        std::io::Write(&mut output).unwrap(); // Capture stdout
        solution(); 

        assert_eq!(String::from_utf8(output).unwrap(), expected);
    }
}
