use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("1.txt")?;
    let result = solution(&input);
    println!("{result}");
    Ok(())
}

fn solution(string: &str) -> i32 {
    let mut result = 0;
    for line in string.lines() {
        let mut first_digit = ' ';
        for chr in line.chars() {
            if chr.is_digit(10) {
                first_digit = chr;
                break;
            }
        }
        let mut last_digit = ' ';
        for chr in line.chars().rev() {
            if chr.is_digit(10) {
                last_digit = chr;
                break;
            }
        }

        let number = format!("{first_digit}{last_digit}");
        let number = number.parse::<i32>().unwrap();
        result += number;
    }

    result
}

#[test]
fn test1() {
    let input = "1abc2\n\
                 pqr3stu8vwx\n\
                 a1b2c3d4e5f\n\
                 treb7uchet";
    let expected = 142;
    let actual = solution(input);
    assert_eq!(expected, actual);
}
