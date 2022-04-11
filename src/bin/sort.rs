//! Sort the given file. The output is written as plan csv file and has to be gzipped.

use std::{
    env::args,
    fs::{self, File},
    io::{BufRead, Write},
    path::PathBuf,
};

use reddit_place_2022::{access::Access, create_file_name, open_dataset, HEADER};

fn main() {
    let folder = fs::read_to_string("dataset_folder.txt").unwrap();

    for i in args().skip(1) {
        let filename = create_file_name(i.parse().unwrap());

        let path = PathBuf::from(&folder).join(&filename);
        let reader = open_dataset(path).unwrap();

        let mut accesses = reader
            .split(b'\n')
            .map(Result::unwrap)
            .map(Access::new)
            .collect::<Vec<_>>();

        let path = PathBuf::from(&folder).join(&filename[..filename.len() - 5]);
        let mut file = File::create(path).unwrap();
        file.write(HEADER.as_bytes()).unwrap();

        accesses.sort_by_cached_key(Access::time);

        for access in accesses {
            file.write(format!("{}", access.time()).as_bytes()).unwrap();
            file.write(access.get(access.commas[0]..).as_bytes())
                .unwrap();
            file.write(&[b'\n']).unwrap();
        }

        file.flush().unwrap();
    }
}
