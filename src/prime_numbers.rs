fn prime_factors(n: u32) -> Vec<u32> {

    let mut factors = Vec::new();
    let mut remainder = n;
    let mut factor = 2;

    while remainder > 1 {
        while remainder % factor == 0 {
            factors.push(factor);
            remainder /= factor;
        }

        factor += 1;
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_factors_number_1_to_empty_list() {
        assert_eq!(Vec::<u32>::new(), prime_factors(1));
    }

    #[test]
    fn it_factors_number_2_to_2() {
        assert_eq!(vec![2], prime_factors(2));
    }

    #[test]
    fn it_factors_number_3_to_3() {
        assert_eq!(vec![3], prime_factors(3));
    }

    #[test]
    fn it_factors_number_4_to_2_and_2() {
        assert_eq!(vec![2, 2], prime_factors(4));
    }

    #[test]
    fn it_factors_number_5_to_5() {
        assert_eq!(vec![5], prime_factors(5));
    }

    #[test]
    fn it_factors_number_6_to_3_and_2() {
        assert_eq!(vec![2, 3], prime_factors(6));
    }

    #[test]
    fn it_factors_number_7_to_7() {
        assert_eq!(vec![7], prime_factors(7));
    }

    #[test]
    fn it_factors_number_8_to_2_2_and_2() {
        assert_eq!(vec![2, 2, 2], prime_factors(8));
    }

    #[test]
    fn it_factors_number_9_to_3_and_3() {
        assert_eq!(vec![3, 3], prime_factors(9));
    }

    #[test]
    fn it_factors_a_large_number_in_its_prime_factors() {
        assert_eq!(
            vec![2, 2, 3, 3, 5, 11, 13, 17, 17, 23],
            prime_factors(2 * 2 * 3 * 3 * 5 * 11 * 13 * 17 * 17 * 23)
        )
    }
}