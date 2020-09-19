use std::cmp::Ordering;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
struct Player {
    name: String,
}
impl Player {
    pub fn from(name: &str) -> Player {
        Player {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Team {
    players: Vec<Player>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            players: Vec::new(),
        }
    }
    pub fn from(ls: &[&str]) -> Team {
        let mut players: Vec<Player> = ls.iter().cloned().map(|p| Player::from(p)).collect();
        players.sort();
        Team { players }
    }
    pub fn add_player(&mut self, m: &str) {
        self.players.push(Player::from(m));
        self.players.sort();
    }
    pub fn similarity(&self, other: &Self) -> f32 {
        /*
         * Returns a value from 0.0 to 1.0 where 1.0 means both teams are the
         * same and 0.0 that they have nothing in common.
         *
         * Function is symmetric
         * */
        if self.players.len() != other.players.len() {
            return 0.0;
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
        return 1.0 - (f32::from(a_diff + b_diff) / f32::from(n));
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.players.len() < other.players.len() {
            return Ordering::Greater;
        }
        self.players.cmp(&other.players)
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Round {
    teams: Vec<Team>,
}

impl Round {
    pub fn new() -> Round {
        Round { teams: Vec::new() }
    }
    pub fn from(ls: &[Team]) -> Round {
        let mut teams: Vec<Team> = ls.iter().cloned().collect();
        teams.sort();
        Round { teams }
    }
    pub fn add_team(&mut self, t: Team) {
        self.teams.push(t);
        self.teams.sort();
    }
    pub fn similarity(&self, other: &Self) -> f32 {
        /*
         * Returns a value from 0.0 to 1.0 where 1.0 means both rounds are the
         * same and 0.0 that they have nothing in common.
         *
         * Function is symmetric
         * */
        if self.teams.len() != other.teams.len() {
            return 0.0;
        }
        let mut n = 0.0;
        let mut sum = 0.0;
        for i in 0..self.teams.len() {
            n += 1.0;
            sum += self
                .teams
                .get(i)
                .unwrap()
                .similarity(other.teams.get(i).unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;
    mod round {
        use super::*;
        mod similarity {
            use super::*;
            #[test]
            fn same() {
                let r1 = Round::from(&[Team::from(&["A", "B", "C"])]);
                let r2 = Round::from(&[Team::from(&["A", "B", "C"])]);
                assert_eq!(r1.similarity(&r2), 1.0);
                assert_eq!(r2.similarity(&r1), 1.0);
            }
            #[test]
            fn different() {
                let r1 = Round::from(&[Team::from(&["A", "B", "C"])]);
                let r2 = Round::from(&[Team::from(&["D", "E", "F"])]);
                assert_eq!(r1.similarity(&r2), 0.0);
                assert_eq!(r2.similarity(&r1), 0.0);
            }
            #[test]
            fn different_by_one() {
                let r1 = Round::from(&[Team::from(&["A", "B", "C"])]);
                let r2 = Round::from(&[Team::from(&["A", "B", "F"])]);
                assert_eq!(r1.similarity(&r2), 0.6666666);
                assert_eq!(r2.similarity(&r1), 0.6666666);
            }
            #[test]
            fn different_by_two() {
                let r1 = Round::from(&[Team::from(&["A", "B", "C"])]);
                let r2 = Round::from(&[Team::from(&["A", "E", "F"])]);
                assert_eq!(r1.similarity(&r2), 0.3333333);
                assert_eq!(r2.similarity(&r1), 0.3333333);
            }
            #[test]
            fn same_with_many_teams() {
                let r1 = Round::from(&[Team::from(&["A", "B", "C"]), Team::from(&["1", "2", "3"])]);
                let r2 = Round::from(&[Team::from(&["A", "B", "C"]), Team::from(&["1", "2", "3"])]);
                assert_eq!(r1.similarity(&r2), 1.0);
                assert_eq!(r2.similarity(&r1), 1.0);
            }
            #[test]
            fn same_with_many_teams_order_should_not_matter() {
                let r1 = Round::from(&[Team::from(&["1", "2", "3"]), Team::from(&["A", "B", "C"])]);
                let r2 = Round::from(&[Team::from(&["A", "B", "C"]), Team::from(&["1", "2", "3"])]);
                assert_eq!(r1.similarity(&r2), 1.0);
                assert_eq!(r2.similarity(&r1), 1.0);
            }
        }
    }
    mod team {
        use super::*;
        mod similarity {
            use super::*;
            #[test]
            fn different_size_is_zero() {
                let t1 = Team::from(&["A"]);
                let t2 = Team::from(&["A", "B", "C"]);
                assert_eq!(t1.similarity(&t2), 0.0);
                assert_eq!(t2.similarity(&t1), 0.0);
            }
            #[test]
            fn same_is_one() {
                let t1 = Team::from(&["A", "B", "C"]);
                let t2 = Team::from(&["A", "B", "C"]);
                assert_eq!(t1.similarity(&t2), 1.0);
                assert_eq!(t2.similarity(&t1), 1.0);
            }
            #[test]
            fn same_with_different_order_is_one() {
                let t1 = Team::from(&["A", "B", "C"]);
                let t2 = Team::from(&["C", "A", "B"]);
                assert_eq!(t1.similarity(&t2), 1.0);
                assert_eq!(t2.similarity(&t1), 1.0);
            }
            #[test]
            fn completely_different_is_zero() {
                let t1 = Team::from(&["A", "B", "C"]);
                let t2 = Team::from(&["D", "E", "F"]);
                assert_eq!(t1.similarity(&t2), 0.0);
                assert_eq!(t2.similarity(&t1), 0.0);
            }
            #[test]
            fn different_by_one_player_is_two_thirds() {
                let t1 = Team::from(&["A", "B", "C"]);
                let t2 = Team::from(&["A", "B", "F"]);
                assert_eq!(t1.similarity(&t2), 0.6666666);
                assert_eq!(t2.similarity(&t1), 0.6666666);
            }
            #[test]
            fn different_by_two_players_is_one_third() {
                let t1 = Team::from(&["A", "B", "C"]);
                let t2 = Team::from(&["A", "E", "F"]);
                assert_eq!(t1.similarity(&t2), 0.3333333);
                assert_eq!(t2.similarity(&t1), 0.3333333);
            }
        }
        mod add_player {
            use super::*;
            #[test]
            fn handle_a_new_team() {
                let t1 = Team::new();
                assert_eq!(t1.players, []);
            }
            #[test]
            fn keep_list_of_players_sorted() {
                let mut t1 = Team::from(&["B"]);
                t1.add_player("A");
                assert_eq!(t1.players, [Player::from("A"), Player::from("B")]);
            }
        }
        mod from {
            use super::*;
            #[test]
            fn keep_list_of_players_sorted() {
                let t1 = Team::from(&["C", "B", "A"]);
                assert_eq!(
                    t1.players,
                    [Player::from("A"), Player::from("B"), Player::from("C")]
                );
            }
        }
    }
}
