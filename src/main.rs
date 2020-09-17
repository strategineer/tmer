extern crate clap;
use log::{info,Level};
use std::fs::File;
use std::io::{self, BufRead};
use rand::thread_rng;
use rand::seq::SliceRandom;
use clap::{Arg, App};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn generate_teams(n_players: usize, n_teams: usize, team_size: usize, ids: Vec<String>) {
    let mut start_index: usize;
    let mut end_index: usize = 0;
    for t in 0..n_teams {
        start_index = t * team_size;
        end_index = t * team_size + team_size;
        info!("{}:{}", start_index, end_index);
        for i in start_index..end_index {
            print!("{} ", ids.get(i).unwrap().to_string());
        }
        println!()
    }
    if end_index <= n_players {
        for i in end_index..n_players {
            print!("{} ", ids.get(i).unwrap().to_string());
        }
    }
}


fn run_app() -> Result<(), ()> {
    let matches = App::new("Tmer")
        .version("1.0")
        .author("strategineer <me@strategineer.com>")
        .about("Make teams")
        .arg(Arg::with_name("file")
            .help("...")
            .short("f")
            .long("file")
            .value_name("FILEPATH")
            .conflicts_with("n_players")
            .takes_value(true))
        .arg(Arg::with_name("n_players")
            .help("...")
            .short("n")
            .long("count")
            .value_name("NUMBER_OF_PLAYERS")
            .conflicts_with("file")
            .takes_value(true))
        .arg(Arg::with_name("n_teams")
            .help("...")
            .short("t")
            .long("teams")
            .conflicts_with("n_size")
            .value_name("NUMBER_OF_TEAMS")
            .takes_value(true))
        .arg(Arg::with_name("n_size")
            .help("...")
            .short("s")
            .long("size")
            .conflicts_with("n_teams")
            .value_name("TEAM_SIZE")
            .takes_value(true))
        .arg(Arg::with_name("n_rounds")
            .help("...")
            .short("r")
            .long("rounds")
            .value_name("NUMBER_OF_ROUNDS")
            .takes_value(true))
        .arg(Arg::with_name("debug")
            .help("print debug information verbosely")
            .short("d")
            .long("debug"))
        .get_matches();
    let is_debug = matches.is_present("debug");

    // TODO(strategineer): fix logging
    if is_debug {
        //console_log::init_with_level(Level::Info);
    } else {
        //console_log::init_with_level(Level::Debug);
    }
    // TODO(strategineer): allow the generation of multiple rounds of teams ensuring that the
    // leftover people are picked during subsequent rounds
    let n_rounds_str = matches.value_of("n_rounds");
    let n_rounds: usize;
    match n_rounds_str {
        None => panic!("Either the -t or -s parameters must be set"),
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => {
                    n_teams = n_players / n;
                    team_size = n;
                },
                Err(err) => panic!("Size parameter must be a number: {:?}", err)
            }
        }
    }

    let n_players_str = matches.value_of("n_players");
    let n_players: usize;
    let mut ids : Vec<String>;
    match n_players_str {
        None => {
            let file_str = matches.value_of("file");
            match file_str {
                None => panic!("Either the -f or -c parameters must be set"),
                Some(s) => {
                    if !Path::new(s).exists() {
                        panic!("File parameter must point to a valid file: {:?}")
                    }
                    ids = Vec::new();
                    if let Ok(lines) = read_lines(s) {
                        for line in lines {
                            if let Ok(l) = line {
                                let p: String = l.split_whitespace().collect();
                                if p.len() > 0 {
                                    ids.push(p);
                                }
                            }
                        }
                    }
                    n_players = ids.len();
                    if n_players == 0 {
                        panic!("File must contain more than one line.");
                    }
                }
            }
        }
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => {
                    n_players = n;
                    ids = (1..n_players + 1).map(|x| x.to_string()).collect();
                },
                Err(err) => panic!("n_players parameter must be a number: {:?}", err)
            }
        }
    }
    let n_teams: usize;
    let team_size: usize;
    let n_teams_str = matches.value_of("n_teams");
    match n_teams_str {
        None => {
            let team_size_str = matches.value_of("n_size");
            match team_size_str {
                None => panic!("Either the -t or -s parameters must be set"),
                Some(s) => {
                    match s.parse::<usize>() {
                        Ok(n) => {
                            n_teams = n_players / n;
                            team_size = n;
                        },
                        Err(err) => panic!("Size parameter must be a number: {:?}", err)
                    }
                }
            }
        }
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => {
                    n_teams = n;
                    team_size = n_players / n;
                },
                Err(err) => panic!("Team parameter must be a number: {:?}", err)
            }
        }
    }

    info!("n_players: {}", n_players);
    info!("n_teams: {}", n_teams);
    info!("team_size: {}", team_size);
    info!("ids: {:?}", ids);
    ids.shuffle(&mut thread_rng());
    info!("shuffled: {:?}", ids);

    if n_players < team_size {
        panic!("Team size must be smaller than the number of players")
    }

    generate_teams(n_players, n_teams, team_size, ids);

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
