pub fn square_of_sum(n: i32) -> i32 {
    let mut a = 0;
    for i in 1 .. n + 1 {
        a = a + i;
    }
    a * a
}

pub fn sum_of_squares(n: i32) -> i32 {
    let mut a = 0;
    for i in 1 .. n + 1 {
        a = a + (i * i);
    }
    a
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
