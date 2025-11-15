use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

struct Team {
    name: String,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    points: u32,
}

impl Team {
    fn new(name: String) -> Self {
        Team {
            name,
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            points: 0,
        }
    }

    fn win(&mut self) {
        self.played += 1;
        self.won += 1;
        self.points += 3;
    }

    fn loss(&mut self) {
        self.played += 1;
        self.lost += 1;
    }

    fn draw(&mut self) {
        self.played += 1;
        self.drawn += 1;
        self.points += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tally = "Team                           | MP |  W |  D |  L |  P".to_owned();

    let mut teams = HashMap::<&str, Rc<RefCell<Team>>>::new();

    for line in match_results.lines() {
        let pline: Vec<&str> = line.split(';').collect();
        if pline.len() != 3 {
            continue;
        }
        let (t1_name, t2_name, result) = (pline[0], pline[1], pline[2]);

        let team1 = teams
            .entry(t1_name)
            .or_insert(Rc::new(RefCell::new(Team::new(t1_name.to_owned()))))
            .clone();

        let team2 = teams
            .entry(t2_name)
            .or_insert(Rc::new(RefCell::new(Team::new(t2_name.to_owned()))))
            .clone();

        let (mut t1, mut t2) = (team1.borrow_mut(), team2.borrow_mut());

        match result {
            "win" => {
                t1.win();
                t2.loss();
            }
            "loss" => {
                t1.loss();
                t2.win();
            }
            "draw" => {
                t1.draw();
                t2.draw();
            }
            _ => continue,
        }
    }

    let mut result_table = teams.values().map(|t| t.borrow()).collect::<Vec<_>>();

    result_table.sort_unstable_by(|a, b| match b.points.cmp(&a.points) {
        Ordering::Equal => a.name.cmp(&b.name),
        v => v,
    });

    for team in result_table {
        tally.push_str(&format!(
            "\n{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            team.name.as_str(),
            team.played,
            team.won,
            team.drawn,
            team.lost,
            team.points
        ));
    }

    tally
}
