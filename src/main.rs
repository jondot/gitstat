#![feature(collections)]
extern crate clap;
extern crate git2;
extern crate ignore;
use clap::{App, Arg};
use std::path::Path;
use std::vec::Vec;
use std::time::Instant;




use ignore::Walk;
use git2::{Repository, StatusOptions};

fn main() {
    let matches = App::new("gitstat")
        .version("1.0")
        .author("Dotan Nahum <jondotan@gmail.com>")
        .about("Mass-find Git repositories with uncommitted work.")
        .arg(Arg::with_name("PATH").multiple(true))
        .get_matches();

    let start = Instant::now();
    let mut opts = StatusOptions::new();
    opts.include_ignored(false);
    opts.include_untracked(true);
    let paths = if matches.is_present("PATH") {
        matches.values_of("PATH").unwrap().collect()
    } else {
        vec!["."]
    };
    let (valids, invalids): (Vec<&str>, Vec<&str>) =
        paths.iter().partition(|p| Path::new(p).exists());

    let mut stat_candidate = 0;
    let mut stat_found = 0;

    valids.iter().for_each(|p| {
        Walk::new(p)
            .map(|x| x.unwrap())
            .inspect(|_| stat_candidate = stat_candidate + 1)
            .filter(|x| x.file_type().unwrap().is_dir())
            .map(|x| x.path().to_str().unwrap().to_string())
            .filter(|reppath| {
                if reppath == ".git" {
                    return false;
                }
                match Repository::open(reppath) {
                    Ok(rep) => match rep.statuses(Some(&mut opts)) {
                        Ok(s) => !s.is_empty(),
                        Err(_) => false,
                    },
                    Err(_) => false,
                }
            })
            .inspect(|_| stat_found = stat_found + 1)
            .for_each(|x| println!("{}", x))
    });
    let timing = Instant::now().duration_since(start);
    println!(
        "uncommitted: {} of {}. took: {}s.",
        stat_found,
        stat_candidate,
        timing.as_secs()
    );
    if !invalids.is_empty() {
        println!(
            "warning: invalid root paths found ({:?}). did you mistype an argument?",
            invalids
        );
    }
}
