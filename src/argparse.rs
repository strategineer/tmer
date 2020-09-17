use clap::{ArgMatches};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct TmerArgs {
    pub n_rounds: usize,
    pub n_players: usize,
    pub n_teams: usize,
    pub team_size: usize,
    pub elements: Vec<String>,
}

impl TmerArgs {
    pub fn new(matches: ArgMatches) -> TmerArgs {
        let n_rounds: usize;
        match matches.value_of("n_rounds") {
            Some(s) => n_rounds = s.parse::<usize>().expect("Size parameter must be a number"),
            None => n_rounds = 1
        }

        let n_players: usize;
        let mut elements: Vec<String>;
        match matches.value_of("n_players") {
            Some(s) => {
                n_players = s.parse::<usize>().expect("n_players parameter must be a number");
                elements = (1..n_players + 1).map(|x| x.to_string()).collect();
            }
            None => {
                match matches.value_of("file") {
                    None => panic!("Either the -f or -c parameters must be set"),
                    Some(s) => {
                        if !Path::new(s).exists() {
                            panic!("File parameter must point to a valid file: {:?}")
                        }
                        // TODO(strategineer): panic if we find any duplicate elements
                        elements = Vec::new();
                        if let Ok(lines) = read_lines(s) {
                            for line in lines {
                                if let Ok(l) = line {
                                    let p: String = l.split_whitespace().collect();
                                    if p.len() > 0 {
                                        elements.push(p);
                                    }
                                }
                            }
                        }
                        n_players = elements.len();
                        if n_players == 0 {
                            panic!("File must contain more than one line.");
                        }
                    }
                }
            }
        }
        let n_teams: usize;
        let team_size: usize;
        match matches.value_of("n_teams") {
            None => {
                match matches.value_of("n_size") {
                    None => panic!("Either the -t or -s parameters must be set"),
                    Some(s) => {
                        team_size = s.parse::<usize>().expect("Size parameter must be a number");
                        n_teams = n_players / team_size;
                    }
                }
            }
            Some(s) => {
                n_teams = s.parse::<usize>().expect("Team parameter must be a number");
                team_size = n_players / n_teams;
            }
        }
        return TmerArgs {
            n_rounds: n_rounds,
            n_players: n_players,
            n_teams: n_teams,
            team_size: team_size,
            elements: elements,
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
