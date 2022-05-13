# Reddit Place 2022 - Assortment of Tools

This repository contains a bunch of small tools to clean-up and analyse the official reddit place 2022 dataset.

## Examples

https://user-images.githubusercontent.com/5956390/162839320-6beb78fd-84ed-4089-a585-9cf7f8f64a56.mp4

https://user-images.githubusercontent.com/5956390/162839350-a3d73144-a8e0-4488-afcc-af394839c871.mp4

## Setup

At the time of creation, the official merged file was messed up. So all tools build on the 79 separate files. These also have some non-obvious flaws.

All tools need to know the path where the gzipped files reside. Simply add a file named `dataset_folder.txt` that contains the path to the dataset files.

## Clean-Up

Quick discussion about the flaws found in the 79 separate files download of the official dataset:

1. The given files could not be unpacked with `flate2` correctly. I had to unpack and repack them with another tool and after that it worked without a problem. Possibly the problem was, that the contained file name did not match the archive name.

2. The order of edits in some of the files is not correct (not chronological). The `checker` tool checks which files are not sorted chronologically (e.g with `cargo run --bin checker`). Use the `sort` tool to chronologically sort the given files (e.g. with `cargo run --bin sort -- file1.gzip file2.gzip`).

3. The file numbers are not in the correct chronological order and some files (number 0 and 1) even overlap in timestamps. Use `order` to print the correct file order and to find overlaps between files. Important: files have to be sorted first (see step above). I have hardcoded the order in the constant `FILE_ORDER` of `lib.rs`. You could also rename the dataset files to achieve a chronological ordering. I have fixed the overlap manually by:

    * unpacking the two files
    * using `cat` to concatenate them
    * repacking the concatenated file
    * using the `sort` tool to sort the packed concatenated file
    * splitting the resulting file roughly in the middle (be careful to split at a line-end and to include the CSV header in both files)
    * repacking both files that resulted from the split

## Commands

A quick explanation of the commands in the main file. All of these can be run using `cargo run -- <command> [argument(s)]`

Make sure to correctly populate `dataset_folder.txt` (see Setup).

* `timelapse` (`cargo run -- timelapse [x1 y1 x2 y2]`): Creates a timelapse of the specified region. The region is inclusive from the top left corner (x1, y1) and exclusive to the bottom right corner (x2, y2). The timelapse is created as single images in the folder `images`. One image every 100 edits is emitted.

* `count-access` (`cargo run -- count-access [x1 y1 x2 y2]`): Counts amount of edits inside the specified region.

* `print-all` (`cargo run -- print-all [x1 y1 x2 y2]`): Prints all edits that happened in the specified region.

* `find-user` (`cargo run -- find-user`): Prints all edits done by the specified user. To specify a user add the exact user id to the file `user_id.txt`.

* `user` (`cargo run -- user`): Creates an image containing all pixels that the specified user placed. To specify a user add the exact user id to the file `user_id.txt`. The colour of pixels that the user did not edit is set to #333 (dark-gray).
