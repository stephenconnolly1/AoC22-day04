use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut count = 0;
    if let Ok(lines) = read_lines(file_path) {
        let mut sum_priorities = 0;
        let mut group: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string()];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x) = line {
                let v: Vec<&str> = x.split(',').collect();
                if is_contained(v[0], v[1]) {
                    count += 1;
                }
            //    println!(" Elf1: {}, Elf2: {}", v[0], v[1]);                
            }
        }
        println!("Total of overlapping ranges: {count} ");
    } else {
        println!("Unable to open file");
    }
}
fn is_contained (range1: &str, range2: &str ) -> bool {
    let r1: Vec<&str> = range1.split('-').collect();
    let r2: Vec<&str> = range2.split('-').collect();
    let mut E1_bottom: i32 = 0;
    let mut E1_top: i32    = 0;
    let mut E2_bottom: i32 = 0;
    let mut E2_top: i32    = 0;
    if let Ok(result) = r1[0].parse::<i32>() {
        E1_bottom = result;
    }
    if let Ok(result) = r1[1].parse::<i32>() {
        E1_top = result;
    }
    if let Ok(result) = r2[0].parse::<i32>() {
        E2_bottom = result;
    }
    if let Ok(result) = r2[1].parse::<i32>() {
        E2_top = result;
    }
    // println!("{E1_bottom},{E1_top},{E2_bottom},{E2_top}");

/*
    if E1_bottom<=E2_bottom && E1_top>=E2_top { 
        println!("E1 contains E2"); 
        return true;
    } 
    if E1_bottom>=E2_bottom && E1_top<=E2_top { 
        println!("E2 contains E1"); 
        return true;
    } 
    return false;
*/
if E1_top<E2_bottom {return false;} 
if E2_top<E1_bottom {return false;}

return true;

}
