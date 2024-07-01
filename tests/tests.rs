use expenses_finder::parse_line;

#[test]
fn test_combinations() {
    let test_line = "10/06/2027	16/06/2024	Foo bar 436845559619 LAS VEGAS	164.64";

    assert_eq!(parse_line(test_line), Some(vec![43684555961900, 16464]));
    let test_line = "acheekyasdfads;lkj	CORPORATE REMITTANCE RECEIVED	-391.35";

    assert_eq!(parse_line(test_line), Some(vec![-39135]));
}
