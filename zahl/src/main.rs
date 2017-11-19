


fn main() {

    let vec: Vec<String> = std::env::args().collect();


    //falls 1 zahl übergeben
    match vec.len() {
        2 => {
            //zahl in i32 verwandeln
            match vec[1].parse::<i32>() {
                Ok(zahl) => {
                    //sachen mit zahl machen
                    match zahl {
                        1 => {
                            println!("I");

                        }
                        _ => println!("mmm"),
                    }
                    //println!("{}", vec[1]);
                }
                Err(e) => println!("{}",e),
            }
        }
        _ => println!("Zahl eingeben"),
    }
}
