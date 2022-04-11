//! Figures out the correct order of files and prints them to the console.
//! order.rs assumes, that the files are internally sorted. Use sort.rs to do so if this does not apply yet.
//! At the end there's an additional overlap check to check if files are consecutive and not overlapping at any point.

use std::{fs, io::BufRead, path::PathBuf};

use reddit_place_2022::{access::Access, create_file_name, open_dataset, NUM_FILES};

fn main() {
    let folder = fs::read_to_string("dataset_folder.txt").unwrap();

    let mut dates = Vec::new();

    for i in 0..NUM_FILES {
        let filename = create_file_name(i);

        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        let mut accesses = reader.split(b'\n').map(Result::unwrap).map(Access::new);
        dates.push((
            i,
            accesses.nth(0).unwrap().time(),
            accesses.last().unwrap().time(),
        ));
    }

    dates.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));
    let order = dates.iter().map(|(i, _, _)| i).collect::<Vec<_>>();
    println!("Order: {:?}", order);

    // Order:
    // [
    //     10, 6, 9, 14, 15, 17, 13, 11, 7, 16, 12, 18, 24, 20, 26, 19, 21, 22, 23, 25, 27, 28, 35,
    //     30, 31, 32, 33, 34, 39, 42, 36, 37, 38, 43, 44, 45, 46, 48, 29, 51, 47, 54, 3, 2, 41, 0, 1,
    //     55, 56, 4, 50, 57, 59, 53, 40, 49, 62, 52, 65, 61, 69, 8, 63, 64, 60, 66, 67, 5, 74, 68,
    //     75, 76, 77, 70, 71, 72, 73, 58, 78,
    // ]

    let mut last_i = dates[0].0;
    let mut last_end = dates[0].1;
    for (i, start, end) in dates {
        if start < last_end {
            println!("Overlap: {} and {}", last_i, i);
            println!("! {} <= {}", last_end, start);
        }
        last_i = i;
        last_end = end;
    }

    // Overlap: 1 and 0
    // ! 2022-04-04 01:47:41.048 UTC <= 2022-04-04 00:53:32.285 UTC

    // Had to manually fix these: concatenate -> sort -> split roughly in the middle
}
