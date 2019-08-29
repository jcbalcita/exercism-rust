pub fn is_leap_year(year: u64) -> bool {
    let divisble_by_four = year % 4 == 0;
    let divisible_by_hundred = year % 100 == 0;
    let divisble_by_four_hundred = year % 400 == 0;

    match (divisble_by_four, divisible_by_hundred, divisble_by_four_hundred) {
        (true, true, b) => b,
        (true, false, _) => true,
        _ => false
    }
}
