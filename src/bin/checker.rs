//! Checks, if any two consecutive rows are in correct timestamp oder in all the files.
//! Does not compare files against each other. Use order to get the correct file order.

use std::{fs, io::BufRead, path::PathBuf};

use reddit_place_2022::{access::Access, create_file_name, open_dataset, NUM_FILES};

fn main() {
    let folder = fs::read_to_string("dataset_folder.txt").unwrap();

    for i in 0..NUM_FILES {
        let filename = create_file_name(i);
        println!("Checking {}", filename);

        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        let accesses = reader.split(b'\n').map(Result::unwrap).map(Access::new);

        let mut last = chrono::MIN_DATETIME;
        let mut count_error = 0;
        let mut lines = 0;
        for access in accesses {
            let time = access.time();
            if time < last {
                count_error += 1
            }
            last = time;
            lines += 1;
        }

        if count_error > 0 {
            println!("Found Errors: {}", count_error);
        } else {
            println!("No Errors");
        }
        println!("{}", lines);
    }
}
