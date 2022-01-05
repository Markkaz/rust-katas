fn add(string: &str) -> i32 {
    if string == "" {
        0
    } else {
        if has_custom_delimiter(string) {
            add_with_custom_delimiter(string)
        } else {
            add_with_delimiter(
                string,
                vec![',', '\n']
            )
        }

    }
}

fn has_custom_delimiter(string: &str) -> bool {
    string.starts_with("//")
}

fn add_with_delimiter(string: &str, delimiters: Vec<char>) -> i32 {
    string.split(&delimiters[..])
        .map(|n| { n.parse::<i32>().unwrap() })
        .sum()
}

fn add_with_custom_delimiter(string: &str) -> i32 {
    let (delimiter, string) = string.split_once('\n').unwrap();

    add_with_delimiter(
        string,
        vec![delimiter.chars().nth(2).unwrap()]
    )
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
}