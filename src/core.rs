use std::fmt;

#[derive(Clone, Debug)]
pub struct Team {
    members: Vec<String>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            members: Vec::new(),
        }
    }
    pub fn add_member(&mut self, m: String) {
        self.members.push(m);
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.members.join(","))
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
