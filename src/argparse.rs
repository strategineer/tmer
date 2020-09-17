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
        let n_rounds_str = matches.value_of("n_rounds");
        let n_rounds: usize;
        match n_rounds_str {
            None => n_rounds = 1,
            Some(s) => n_rounds = s.parse::<usize>().expect("Size parameter must be a number")
        }

        let n_players_str = matches.value_of("n_players");
        let n_players: usize;
        let mut ids: Vec<String>;
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
                n_players = s.parse::<usize>().expect("n_players parameter must be a number");
                ids = (1..n_players + 1).map(|x| x.to_string()).collect();
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
            elements: ids,
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
