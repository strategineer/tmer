extern crate clap;
use clap::{App, Arg};
use log::{info};
use rand::seq::SliceRandom;
use rand::thread_rng;

mod core;
use self::core::{Team, Round};
mod argparse;
use self::argparse::{TmerArgs};

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
            team.add_member(ids.get(i).unwrap().to_string());
        }
        round.add_team(team);
    }
    if end_index <= n_players {
        //TODO(strategineer): identify as the "leftover" team?
        let mut team: Team = Team::new();
        for i in end_index..n_players {
            team.add_member(ids.get(i).unwrap().to_string());
        }
        round.add_team(team);
    }
    return round;
}

fn run_app() -> Result<(), ()> {
    // TODO(strategineer): document arguments
    let matches = App::new("Tmer")
        .version("1.0")
        .author("strategineer <me@strategineer.com>")
        .about("Make teams")
        .arg(
            Arg::with_name("file")
                .help("...")
                .short("f")
                .long("file")
                .value_name("FILEPATH")
                .conflicts_with("n_players")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_players")
                .help("...")
                .short("n")
                .long("count")
                .value_name("NUMBER_OF_PLAYERS")
                .conflicts_with("file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_teams")
                .help("...")
                .short("t")
                .long("teams")
                .conflicts_with("n_size")
                .value_name("NUMBER_OF_TEAMS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_size")
                .help("...")
                .short("s")
                .long("size")
                .conflicts_with("n_teams")
                .value_name("TEAM_SIZE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_rounds")
                .help("...")
                .short("r")
                .long("rounds")
                .value_name("NUMBER_OF_ROUNDS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .help("print debug information verbosely")
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

    for _ in 0..args.n_rounds {
        ids.shuffle(&mut thread_rng());
        info!("shuffled: {:?}", ids);
        // TODO(strategineer): implement an algo that computes the similarity between teams and
        // rounds in order to allow for the computation of "more" different rounds to avoid teams
        // being too similar round after round
        let round = generate_teams(args.n_players, args.n_teams, args.team_size, &ids);
        println!("{}", round);
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
