fn main() {
    let mut x = 27;
    let mut zaehler = 1;

    while x > 1 {
        if x % 2 == 0 && x != 1 {
            x = x / 2;
            println!("{} -> {}", zaehler, x);
            zaehler += 1;
        }
        if x % 2 != 0 && x != 1 {
            x = (x * 3) + 1;
            println!("{} -> {}", zaehler, x);
            zaehler += 1;
        }
    }
}
