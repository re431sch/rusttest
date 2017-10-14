fn main() {
    println!("Grains Total: {}", total());
}

fn square(n: u32) -> u64 {
    2u64.pow(n - 1)
}

fn total() -> u64 {
    let mut x = 0;
    for y in 1..65 {
        x += square(y);
    }
    x
}

#[test]
fn square_one() {
    assert_eq!(square(1), 1);
}

#[test]
fn square_two() {
    assert_eq!(square(2), 2);
}

#[test]
fn square_three() {
    assert_eq!(square(3), 4);
}

#[test]
fn square_four() {
    assert_eq!(square(4), 8);
}

#[test]
fn square_sixteen() {
    assert_eq!(square(16), 32_768);
}

#[test]
fn square_thirty_two() {
    assert_eq!(square(32), 2_147_483_648);
}

#[test]
fn square_sixty_four() {
    assert_eq!(square(64), 9_223_372_036_854_775_808);
}

#[test]
fn total_sums_all_squares() {
    assert_eq!(total(), 18_446_744_073_709_551_615);
    total();
}
