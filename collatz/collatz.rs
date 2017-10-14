fn main () {
    let mut x = 27;

    for i in 1 .. {
        if x % 2 == 0 {
            x = x/2;
        } else {
            x = 3*x+1;
        }
        println!("{} -> {}", i, x);
        if x == 1 {
            break;
        }
    }
}
