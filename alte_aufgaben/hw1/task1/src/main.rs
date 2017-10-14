fn main() {
    let years = [1900, 2000, 1997, 1004, 1833, 2016];
    for x in 0..years.len() {
        if is_leap_year(years[x]) {
            println!("{}*", years[x]);
        } else {
            println!("{}", years[x]);
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            }
        }
        return true;
    } else {
        return false;
    }
}


#[test]
fn test_vanilla_leap_year() {
    assert_eq!(is_leap_year(1996), true);
}

#[test]
fn test_any_old_year() {
    assert_eq!(is_leap_year(1997), false);
}

#[test]
fn test_century() {
    assert_eq!(is_leap_year(1900), false);
}

#[test]
fn test_exceptional_centuries() {
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2400), true);
}
