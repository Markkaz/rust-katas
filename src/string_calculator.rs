use regex::{Regex,escape};

fn add(string: &str) -> i32 {
    if string == "" {
        0
    } else {
        if has_custom_delimiter(string) {
            add_with_custom_delimiter(string)
        } else {
            add_with_delimiter(
                string,
                vec![",".to_owned(), "\n".to_owned()]
            )
        }

    }
}

fn has_custom_delimiter(string: &str) -> bool {
    string.starts_with("//")
}

fn convert_delimiters_to_regex(delimiters: Vec<String>) -> Regex {
    Regex::new(
        &delimiters.join("|")
    ).unwrap()
}

fn add_with_delimiter(string: &str, delimiters: Vec<String>) -> i32 {
    let mut negative_numbers = Vec::new();

    let sum = convert_delimiters_to_regex(delimiters)
        .split(string)
        .map(|n| { n.parse::<i32>().unwrap() })
        .filter(|n| n <= &1000)
        .map(|n| {
            if n < 0 {
                negative_numbers.push(n);
            }

            n
        }).sum();

    if negative_numbers.len() > 0 {
        panic!("Negative numbers: {:?}", negative_numbers);
    } else {
        sum
    }
}

fn add_with_custom_delimiter(string: &str) -> i32 {
    let (delimiter, string) = string.split_once('\n').unwrap();

    add_with_delimiter(
        string,
        extract_custom_delimiters(delimiter)
    )
}

fn extract_custom_delimiters(delimiter: &str) -> Vec<String> {
    vec![escape(&delimiter.replace("//", ""))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gives_0_for_an_empty_string() {
        assert_eq!(0, add(""));
    }

    #[test]
    fn it_gives_the_number_for_a_string_containing_a_single_number() {
        assert_eq!(1, add("1"));
    }

    #[test]
    fn it_sums_two_numbers_seperated_by_an_comma() {
        assert_eq!(3, add("1,2"));
    }

    #[test]
    fn it_sums_three_numbers_seperated_by_an_comma() {
        assert_eq!(6, add("1,2,3"));
    }

    #[test]
    fn it_sums_three_numbers_seperated_by_newlines_and_commas() {
        assert_eq!(6, add("1\n2,3"));
    }

    #[test]
    fn it_allows_to_change_the_delimiter_to_a_semicolon() {
        assert_eq!(6, add("//;\n1;2;3"));
    }

    #[test]
    fn it_ignores_numbers_larger_than_1000() {
        assert_eq!(6, add("1,1001,2,3"));
    }

    #[test]
    #[should_panic]
    fn it_panics_when_summing_negative_numbers() {
        add("-1,-5,-7");
    }

    #[test]
    fn it_allows_custom_delimiters_of_arbritary_length() {
        assert_eq!(13, add("//?%?\n1?%?2?%?4?%?6"));
    }
}