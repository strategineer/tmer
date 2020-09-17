use std::fmt;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Player {
    name: String,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug)]
pub struct Team {
    players: HashSet<Player>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            players: HashSet::new(),
        }
    }
    pub fn add_player(&mut self, m: String) {
        self.players.insert(Player{ name: m });
    }
    pub fn similarity(&self, other: &Team) -> f32 {
        /* 
         * Returns a value from 0.0 to 1.0 where 1.0 means both teams are the
         * same and 0.0 that they have nothing in common.
         *
         * Function is symmetric
         * */
        if self.players.len() != other.players.len() {
            return 0.0
        }
        let mut n = 0.0;
        let mut a_diff = 0.0;
        let mut b_diff = 0.0;
        for i in &self.players {
            n += 1.0;
            if !other.players.contains(i) {
                a_diff += 1.0;
            }
        }
        for i in &other.players {
            n += 1.0;
            if !self.players.contains(i) {
                b_diff += 1.0;
            }
        }
        return 1.0 - (f32::from(a_diff/b_diff) / f32::from(n));
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.players.len() {
            0 => write!(f, ""),
            _ => write!(
                f,
                "{}",
                self.players
                    .clone()
                    .into_iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Round {
    teams: Vec<Team>,
}

impl Round {
    pub fn new() -> Round {
        Round { teams: Vec::new() }
    }
    pub fn add_team(&mut self, t: Team) {
        self.teams.push(t);
    }
    pub fn similarity(&self, other: &Round) -> f32 {
        /* 
         * Returns a value from 0.0 to 1.0 where 1.0 means both rounds are the
         * same and 0.0 that they have nothing in common.
         *
         * Function is symmetric
         * */
        if self.teams.len() != other.teams.len() {
            return 0.0
        }
        let mut n = 0.0;
        let mut sum = 0.0;
        // TODO(strategineer): figure out way to take ordering into account (or not take it into
        // account)
        for i in 0..self.teams.len() {
            n += 1.0;
            sum += self.teams.get(i).unwrap().similarity(other.teams.get(i).unwrap());
        }
        return sum / n;
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.teams.len() {
            0 => write!(f, ""),
            _ => write!(
                f,
                "{}",
                self.teams
                    .clone()
                    .into_iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
        }
    }
}
