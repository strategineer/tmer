extern crate clap;
use clap::{App, Arg};
use log::{info};
use rand::seq::SliceRandom;
use rand::thread_rng;

mod core;
use self::core::{Team, Round};
mod argparse;
use self::argparse::{TmerArgs};

const ATTEMPT_LIMIT: usize = 100;

fn generate_teams(n_players: usize, n_teams: usize, team_size: usize, ids: &[String]) -> Round {
    let mut round: Round = Round::new();
    let mut start_index: usize;
    let mut end_index: usize = 0;
    for t in 0..n_teams {
        let mut team: Team = Team::new();
        start_index = t * team_size;
        end_index = t * team_size + team_size;
        info!("{}:{}", start_index, end_index);
        for i in start_index..end_index {
            team.add_player(ids.get(i).unwrap().to_string());
        }
        round.add_team(team);
    }
    if end_index <= n_players {
        //TODO(strategineer): identify as the "leftover" team?
        let mut team: Team = Team::new();
        for i in end_index..n_players {
            team.add_player(ids.get(i).unwrap().to_string());
        }
        round.add_team(team);
    }
    return round;
}

fn run_app() -> Result<(), ()> {
    let matches = App::new("Tmer")
        .version("1.0")
        .author("strategineer <me@strategineer.com>")
        .about("Make teams")
        .arg(
            Arg::with_name("file")
                .help("Filepath containing one player name per line.")
                .short("f")
                .long("file")
                .value_name("FILEPATH")
                .conflicts_with("n_players")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_players")
                .help("Number of players. Use this if numbering each player is good enough.")
                .short("n")
                .long("count")
                .value_name("NUMBER_OF_PLAYERS")
                .conflicts_with("file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_teams")
                .help("Number of teams to make.")
                .short("t")
                .long("teams")
                .conflicts_with("n_size")
                .value_name("NUMBER_OF_TEAMS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_size")
                .help("Number of players in each team.")
                .short("s")
                .long("size")
                .conflicts_with("n_teams")
                .value_name("TEAM_SIZE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_rounds")
                .help("Number of rounds to generate.")
                .short("r")
                .long("rounds")
                .value_name("NUMBER_OF_ROUNDS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .help("Print debug information verbosely.")
                .short("d")
                .long("debug"),
        )
        .get_matches();
    let is_debug = matches.is_present("debug");

    // TODO(strategineer): fix logging
    if is_debug {
        //console_log::init_with_level(Level::Info);
    } else {
        //console_log::init_with_level(Level::Debug);
    }
    // TODO(strategineer): with multiple rounds, ensure that the leftover people
    // are picked during subsequent rounds

    let args: TmerArgs = TmerArgs::new(matches);

    info!("n_rounds: {}", args.n_rounds);
    info!("n_players: {}", args.n_players);
    info!("n_teams: {}", args.n_teams);
    info!("team_size: {}", args.team_size);
    info!("ids: {:?}", args.elements);

    let mut ids: Vec<String> = args.elements.clone();

    if args.n_players < args.team_size {
        panic!("Team size must be smaller than the number of players")
    }

    let mut previous_round: Option<Round> = None;
    for _ in 0..args.n_rounds {
        let mut best_similarity = f32::MAX;
        let mut attempts = 0;
        let mut best_round_yet: Option<Round> = None;
        loop {
            ids.shuffle(&mut thread_rng());
            info!("shuffled: {:?}", ids);
            // TODO(strategineer): implement different round generation strategies (simple, then
            // similarity-using one) and allow user to select the strategy through the CLI
            let round = generate_teams(args.n_players, args.n_teams, args.team_size, &ids);
            match &previous_round {
                None => {
                    best_round_yet = Some(round);
                    break;
                }
                Some(r) => {
                    let current_similarity = r.similarity(&round);
                    match best_round_yet {
                        None => {
                            best_similarity = current_similarity;
                            best_round_yet = Some(round);
                        },
                        Some(_) => {
                            if current_similarity < best_similarity {
                                best_similarity = current_similarity;
                                best_round_yet = Some(round);
                            }
                        }

                    }
                }
            }
            attempts += 1;
            if attempts > ATTEMPT_LIMIT || best_similarity > 0.999 {
                break;
            }
        }
        let best_round = best_round_yet.unwrap();
        // TODO(strategineer): write tests for this similarity code to start and then add more
        // tests
        info!("similarity: {}", best_similarity);
        println!("{}", best_round);
        previous_round = Some(best_round);
    }

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
