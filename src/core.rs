use std::collections::HashSet;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Player {
    name: String,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Team {
    players: HashSet<Player>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            players: HashSet::new(),
        }
    }
    pub fn from_slice(ls: &[&str]) -> Team {
        Team {
            players: ls
                .iter()
                .cloned()
                .map(|p| Player {
                    name: p.to_string(),
                })
                .collect(),
        }
    }
    pub fn add_player(&mut self, m: String) {
        self.players.insert(Player { name: m });
    }
    pub fn similarity(&self, other: &Team) -> f32 {
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
    pub fn from_slice(ls: &[Team]) -> Round {
        Round {
            teams: ls.iter().cloned().collect(),
        }
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
            return 0.0;
        }
        let mut n = 0.0;
        let mut sum = 0.0;
        // TODO(strategineer): figure out way to take ordering into account (or not take it into
        // account)
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
    #[test]
    fn round_similarity_same() {
        let r1 = Round::from_slice(&[Team::from_slice(&["A", "B", "C"])]);
        let r2 = Round::from_slice(&[Team::from_slice(&["A", "B", "C"])]);
        assert_eq!(r1.similarity(&r2), 1.0);
        assert_eq!(r2.similarity(&r1), 1.0);
    }
    #[test]
    fn round_similarity_different() {
        let r1 = Round::from_slice(&[Team::from_slice(&["A", "B", "C"])]);
        let r2 = Round::from_slice(&[Team::from_slice(&["D", "E", "F"])]);
        assert_eq!(r1.similarity(&r2), 0.0);
        assert_eq!(r2.similarity(&r1), 0.0);
    }
    #[test]
    fn round_similarity_different_by_one() {
        let r1 = Round::from_slice(&[Team::from_slice(&["A", "B", "C"])]);
        let r2 = Round::from_slice(&[Team::from_slice(&["A", "B", "F"])]);
        assert_eq!(r1.similarity(&r2), 0.6666666);
        assert_eq!(r2.similarity(&r1), 0.6666666);
    }
    #[test]
    fn round_similarity_different_by_two() {
        let r1 = Round::from_slice(&[Team::from_slice(&["A", "B", "C"])]);
        let r2 = Round::from_slice(&[Team::from_slice(&["A", "E", "F"])]);
        assert_eq!(r1.similarity(&r2), 0.3333333);
        assert_eq!(r2.similarity(&r1), 0.3333333);
    }
    #[test]
    fn round_with_many_teams_similarity_same() {
        let r1 = Round::from_slice(&[
            Team::from_slice(&["A", "B", "C"]),
            Team::from_slice(&["1", "2", "3"]),
        ]);
        let r2 = Round::from_slice(&[
            Team::from_slice(&["A", "B", "C"]),
            Team::from_slice(&["1", "2", "3"]),
        ]);
        assert_eq!(r1.similarity(&r2), 1.0);
        assert_eq!(r2.similarity(&r1), 1.0);
    }
    #[test]
    fn team_similarity_size() {
        let t1 = Team::from_slice(&["A"]);
        let t2 = Team::from_slice(&["A", "B", "C"]);
        assert_eq!(t1.similarity(&t2), 0.0);
        assert_eq!(t2.similarity(&t1), 0.0);
    }
    #[test]
    fn team_similarity_same() {
        let t1 = Team::from_slice(&["A", "B", "C"]);
        let t2 = Team::from_slice(&["A", "B", "C"]);
        assert_eq!(t1.similarity(&t2), 1.0);
        assert_eq!(t2.similarity(&t1), 1.0);
    }
    #[test]
    fn team_similarity_same_changed_order() {
        let t1 = Team::from_slice(&["A", "B", "C"]);
        let t2 = Team::from_slice(&["C", "A", "B"]);
        assert_eq!(t1.similarity(&t2), 1.0);
        assert_eq!(t2.similarity(&t1), 1.0);
    }
    #[test]
    fn team_similarity_different() {
        let t1 = Team::from_slice(&["A", "B", "C"]);
        let t2 = Team::from_slice(&["D", "E", "F"]);
        assert_eq!(t1.similarity(&t2), 0.0);
        assert_eq!(t2.similarity(&t1), 0.0);
    }
    #[test]
    fn team_similarity_different_by_one() {
        let t1 = Team::from_slice(&["A", "B", "C"]);
        let t2 = Team::from_slice(&["A", "B", "F"]);
        assert_eq!(t1.similarity(&t2), 0.6666666);
        assert_eq!(t2.similarity(&t1), 0.6666666);
    }
    #[test]
    fn team_similarity_different_by_two() {
        let t1 = Team::from_slice(&["A", "B", "C"]);
        let t2 = Team::from_slice(&["A", "E", "F"]);
        assert_eq!(t1.similarity(&t2), 0.3333333);
        assert_eq!(t2.similarity(&t1), 0.3333333);
    }
}
