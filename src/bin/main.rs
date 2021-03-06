use image::{ImageBuffer, Rgb};
use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs,
    io::BufRead,
    path::PathBuf,
    str,
};

use reddit_place_2022::{
    access::{Access, Location},
    create_file_name, open_dataset, FILE_ORDER,
};

const COLOURS: &[&'static str] = &[
    "#9C6926", "#BE0039", "#00A368", "#00756F", "#FFFFFF", "#94B3FF", "#493AC1", "#009EAA",
    "#D4D7D9", "#FFB470", "#7EED56", "#51E9F4", "#FF4500", "#6A5CFF", "#FFF8B8", "#6D001A",
    "#FFA800", "#000000", "#FF3881", "#FFD635", "#E4ABFF", "#00CCC0", "#FF99AA", "#00CC78",
    "#811E9F", "#B44AC0", "#515252", "#6D482F", "#DE107F", "#2450A4", "#3690EA", "#898D90",
];

/// # Examples
///
/// ```
/// assert_eq!(create_colour_map(&["#AABBCC", "#123456"], vec![[0xaa, 0xbb, 0xcc], [0x12, 0x34, 0x56]]));
/// ```
fn parse_colours(colours: &[&str]) -> Vec<[u8; 3]> {
    colours
        .iter()
        .map(|&colour| {
            assert_eq!(7, colour.len());
            let trimmed = colour.trim_start_matches('#');
            let number = u32::from_str_radix(trimmed, 16).unwrap();
            [(number >> 16) as u8, (number >> 8) as u8, number as u8]
        })
        .collect()
}

fn create_colour_map(colours: &[&'static str]) -> HashMap<&'static str, [u8; 3]> {
    let values = parse_colours(colours);
    colours.iter().map(|&v| v).zip(values).collect()
}

/// Count edits inside of bounds.
fn count_access_in_bounds(bounds: ((u32, u32), (u32, u32))) {
    let folder = fs::read_to_string("dataset_folder.txt").unwrap();
    let mut count = 0;

    for number in FILE_ORDER {
        let filename = create_file_name(number);
        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        count += reader
            .split(b'\n')
            .map(Result::unwrap)
            .map(Access::new)
            .map(|acc| acc.coords())
            .filter(|location| match *location {
                Location::Coords(x, y) => {
                    bounds.0 .0 <= x && x < bounds.1 .0 && bounds.0 .1 <= y && y < bounds.1 .1
                }
                Location::Range(_x1, _y1, _x2, _y2) => false,
            })
            .count();
    }
    println!("{}", count);
}

/// Create images after every `spacing` edits of the space inside the `bounds`.
///
/// Create movie with ffmpeg: `ffmpeg -framerate 30 -i image%08d.png -s:v 1510x370 -sws_flags neighbor -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p movie.mp4`
fn create_images(bounds: ((u32, u32), (u32, u32)), spacing: u32) {
    // Ensure target path is valid.
    fs::create_dir_all("images").unwrap();

    let folder = fs::read_to_string("dataset_folder.txt").unwrap();
    let colour_map = create_colour_map(COLOURS);

    let mut access_counter = 0;
    let mut image = ImageBuffer::new(bounds.1 .0 - bounds.0 .0, bounds.1 .1 - bounds.0 .1);
    image.fill(0xff);

    for number in FILE_ORDER {
        let filename = create_file_name(number);
        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        let in_bounds_access = reader
            .split(b'\n')
            .map(Result::unwrap)
            .map(Access::new)
            .filter_map(|access| match access.coords() {
                Location::Coords(x, y) => Some(((x, y), access)),
                Location::Range(_, _, _, _) => None,
            })
            .filter(|&((x, y), _)| {
                bounds.0 .0 <= x && x < bounds.1 .0 && bounds.0 .1 <= y && y < bounds.1 .1
            });

        for ((x, y), access) in in_bounds_access {
            let colour = colour_map[access.colour()];
            image[(x - bounds.0 .0, y - bounds.0 .1)] = Rgb(colour);
            if access_counter % spacing == 0 {
                image
                    .save_with_format(
                        format!("images/image{:0>8}.png", access_counter / spacing),
                        image::ImageFormat::Png,
                    )
                    .unwrap();
            }
            access_counter += 1;
        }
    }
}

/// Print every access that is in bounds.
fn print_every_access(bounds: ((u32, u32), (u32, u32))) {
    let folder = fs::read_to_string("dataset_folder.txt").unwrap();

    for number in FILE_ORDER {
        let filename = create_file_name(number);
        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        let in_bounds_access = reader
            .split(b'\n')
            .map(Result::unwrap)
            .map(Access::new)
            .filter(|access| match access.coords() {
                Location::Coords(x, y) => {
                    bounds.0 .0 <= x && x < bounds.1 .0 && bounds.0 .1 <= y && y < bounds.1 .1
                }
                Location::Range(_x1, _y1, _x2, _y2) => false,
            });

        for access in in_bounds_access {
            println!("{:?}", access);
        }
    }
}

/// Print every access that is for the given `user_id`.
fn print_access_by(user_id: &str) {
    let mut count = 0;
    let folder = fs::read_to_string("dataset_folder.txt").unwrap();

    for number in FILE_ORDER {
        let filename = create_file_name(number);
        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        let user_access = reader
            .split(b'\n')
            .map(Result::unwrap)
            .map(Access::new)
            .filter(|access| access.user_id() == user_id);

        for access in user_access {
            println!("{:?}", access);
            count += 1;
        }
    }

    println!("Total: {}", count);
}

// Creates image, containing only the pixels that the given user changed.
fn image_of_single_user(user_id: &str, background: u8) {
    // Ensure target path is valid.
    fs::create_dir_all("images").unwrap();

    let folder = fs::read_to_string("dataset_folder.txt").unwrap();
    let colour_map = create_colour_map(COLOURS);

    let mut image = ImageBuffer::new(2000, 2000);
    image.fill(background);

    let mut duplicate_count = 0;
    let mut pixel_duplicates = HashSet::new();

    for number in FILE_ORDER {
        let filename = create_file_name(number);
        let path = PathBuf::from(&folder).join(filename);
        let reader = open_dataset(path).unwrap();

        let user_access = reader
            .split(b'\n')
            .map(Result::unwrap)
            .map(Access::new)
            .filter_map(|access| match access.coords() {
                Location::Coords(x, y) => Some(((x, y), access)),
                Location::Range(_, _, _, _) => None,
            })
            .filter(|(_, access)| access.user_id() == user_id);

        for ((x, y), access) in user_access {
            let colour = colour_map[access.colour()];
            image[(x, y)] = Rgb(colour);

            if !pixel_duplicates.insert((x, y)) {
                duplicate_count += 1;
            }
        }
    }

    println!("Duplicate Positions: {}", duplicate_count);
    image
        .save_with_format("images/user.png", image::ImageFormat::Png)
        .unwrap();
}

fn main() {
    let args_len = args().len();

    let bounds;
    if [1, 2].contains(&args_len) {
        bounds = ((448, 646), (599, 683));
        // 1653 620 1744 640
        // 250 1850 312 1891
    } else {
        assert!([5, 6].contains(&args_len));
        let args = args()
            .skip(args_len - 4)
            .map(|arg| arg.parse().unwrap())
            .collect::<Vec<_>>();
        bounds = ((args[0], args[1]), (args[2], args[3]));
    }

    let command = if [2, 6].contains(&args_len) {
        args().nth(1).unwrap()
    } else {
        "timelapse".to_string()
    };

    let user_id = fs::read_to_string("user_id.txt").unwrap();
    let user_id = user_id.trim();

    match command.as_str() {
        "timelapse" => create_images(bounds, 100),
        "count-access" => count_access_in_bounds(bounds),
        "print-all" => print_every_access(bounds),
        "find-user" => print_access_by(user_id),
        "user" => image_of_single_user(user_id, 0x33),
        _ => eprintln!("Unknown sub-command"),
    }
}
