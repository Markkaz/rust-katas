fn fizzbuzz(n: u8) -> String {
    if n % 15 == 0 {
        "fizzbuzz".to_owned()
    } else if n % 3 == 0 {
        "fizz".to_owned()
    } else if n % 5 == 0 {
        "buzz".to_owned()
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_one_when_given_one() {
        assert_eq!("1", fizzbuzz(1));
    }

    #[test]
    fn it_returns_two_when_given_two() {
        assert_eq!("2", fizzbuzz(2));
    }

    #[test]
    fn it_returns_fizz_when_given_three() {
        assert_eq!("fizz", fizzbuzz(3));
    }

    #[test]
    fn it_returns_buzz_when_given_three() {
        assert_eq!("buzz", fizzbuzz(5));
    }

    #[test]
    fn it_returns_fizz_when_given_six() {
        assert_eq!("fizz", fizzbuzz(6));
    }

    #[test]
    fn it_returns_buzz_when_given_ten() {
        assert_eq!("buzz", fizzbuzz(10));
    }

    #[test]
    fn it_returns_fizzbuzz_when_given_fifteen() {
        assert_eq!("fizzbuzz", fizzbuzz(15));
    }

    #[test]
    fn it_returns_fizzbuzz_when_given_thirty() {
        assert_eq!("fizzbuzz", fizzbuzz(30));
    }
}