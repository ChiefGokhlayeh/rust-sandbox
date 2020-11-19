fn main() {}

pub fn nstep(x: u32, y: u32) -> Option<u32> {
    if x != y && (x < y || x - y != 2) {
        None
    } else {
        return Some((x + y) - (if x % 2 == 0 { 0 } else { 1 }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn smoke_test() {}

    #[test]
    fn when_nstep_with_zero_zero_expect_zero() {
        let res = nstep(0, 0);

        assert_eq!(Some(0), res);
    }

    #[test]
    fn when_nstep_with_one_zero_expect_none() {
        let res = nstep(1, 0);

        assert_eq!(None, res);
    }

    #[rstest(
        x,
        y,
        expected,
        case(0, 0, 0),
        case(1, 1, 1),
        case(2, 0, 2),
        case(3, 1, 3),
        case(2, 2, 4),
        case(3, 3, 5),
        case(4, 2, 6),
        case(5, 3, 7),
        case(4, 4, 8),
        case(5, 5, 9),
        case(6, 4, 10),
        case(7, 5, 11),
        case(6, 6, 12)
    )]
    fn when_nstep_with_valid_x_y_expect_some(x: u32, y: u32, expected: u32) {
        let res = nstep(x, y);

        assert_eq!(Some(expected), res);
    }

    #[rstest(
        x,
        y,
        case(1, 0),
        case(2, 1),
        case(3, 2),
        case(4, 3),
        case(5, 4),
        case(6, 5),
        case(7, 6),
        case(7, 0),
        case(0, 6),
        case(5, 1),
        case(3, 6)
    )]
    fn when_nstep_with_invalid_x_y_expect_none(x: u32, y: u32) {
        let res = nstep(x, y);

        assert_eq!(None, res);
    }
}
