use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut total = 0;
    for line in _input.split('\n') {
        // beginning of a new line, reset first and last
        let mut first = 0;
        let mut last = 0;
    
        for c in line.chars() {
            if c.is_ascii_digit() { 
                if first == 0 { 
                    first = c.to_digit(10).unwrap(); 
                }
                else {
                    last = c.to_digit(10).unwrap();
                }
            };
        }
        // iterated over each character in line
        if last == 0 { last = first }
        total += (first*10) + last;
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        //todo!("haven't built test yet");
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
