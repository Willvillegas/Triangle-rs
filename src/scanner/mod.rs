//! The scanner module

pub struct Scanner {}

#[derive(PartialEq, Eq)]
pub struct Token {}

#[derive(PartialEq, Eq)]
pub enum TokenKind {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq)]
    struct ScannerTestCase {
        kind: TokenKind,
        spelling: [u8],
    }

    #[test]
    fn test_emptycommandeot() {}

    #[test]
    fn test_emptycommandeot_degenerate() {}

    #[test]
    fn test_empty_commandsemicolon() {}

    #[test]
    fn test_empty_commandsemicolon_degenerate() {}

    #[test]
    fn test_hello() {}

    #[test]
    fn test_hello_degenerate() {}

    #[test]
    fn test_eqnoteq() {}

    #[test]
    fn test_eqnoteq_degenerate() {}

    #[test]
    fn test_inc() {}

    #[test]
    fn test_inc_degenerate() {}

    #[test]
    fn test_echo() {}

    #[test]
    fn test_echo_degenerate() {}

    #[test]
    fn test_odd() {}

    #[test]
    fn test_odd_degenerate() {}

    #[test]
    fn test_sum_proc() {}

    #[test]
    fn test_sum_proc_degenerate() {}

    #[test]
    fn test_power() {}

    #[test]
    fn test_power_degenerate() {}

    #[test]
    fn test_factorial() {}

    #[test]
    fn test_factorial_degenerate() {}

    #[test]
    fn test_record() {}

    #[test]
    fn test_record_degenerate() {}

    #[test]
    fn test_leapyear() {}

    #[test]
    fn test_leapyear_degenerate() {}

    #[test]
    fn test_date() {}

    #[test]
    fn test_date_degenerate() {}

    #[test]
    fn test_print_array() {}

    #[test]
    fn test_print_array_degnerate() {}

    #[test]
    fn test_string() {}

    #[test]
    fn test_string_degenerate() {}

    #[test]
    fn test_reverse_line() {}

    #[test]
    fn test_reverse_line_degenerate() {}

    #[test]
    fn test_nestedrecords() {}

    #[test]
    fn test_nestedrecords_degenerate() {}

    #[test]
    fn test_iteratively() {}

    #[test]
    fn test_iteratively_degenerate() {}

    #[test]
    fn test_nestedarrays() {}

    #[test]
    fn test_nestedarrays_degenerate() {}

    #[test]
    fn test_line() {}

    #[test]
    fn test_line_degenerate() {}

    #[test]
    fn test_dates() {}

    #[test]
    fn test_dates_degenerate() {}

    #[test]
    fn test_monthsofyear() {}

    #[test]
    fn test_monthsofyear_degenerate() {}

    #[test]
    fn test_capitalise() {}

    #[test]
    fn test_capitalise_degenerate() {}

    #[test]
    fn test_freq() {}

    #[test]
    fn test_freq_degenerate() {}

    #[test]
    fn test_insertion_sort() {}

    #[test]
    fn test_insertion_sort_degenerate() {}

    #[test]
    fn test_rationals() {}

    #[test]
    fn test_rationals_degenerate() {}
}
