use std::fs::{read_dir, read_to_string};
extern crate regex;

use regex::Regex;
fn main() {
    let regex = Regex::new(r"(?m).+0x(..).+").unwrap();
    let colours: Vec<_> = read_dir("stuff").unwrap().filter_map(|color_set_folder| {
        let directory = read_dir(color_set_folder.unwrap().path());
        if directory.is_err() {
            return None;
        }
        let text = read_to_string(
            directory
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .path(),
        )
        .unwrap();
        let mut colours = text
        .lines()
        .skip(7)
        .take(3)
        .map(|line| regex.captures(line).unwrap().extract::<1>().1[0]);
        let (blue, green, red) = (
            colours.next().unwrap(),
            colours.next().unwrap(),
            colours.next().unwrap(),
        );
        Some(format!("#{red}{green}{blue}"))
    }).collect();
    dbg!(colours);
}
