use std::{cmp::Ordering, collections::HashMap, iter::once};

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

    fn play(&mut self, mut result: &str, is_second: bool) {
        if is_second {
            result = match result {
                "win" => "loss",
                "loss" => "win",
                r => r,
            }
        }
        match result {
            "win" => self.win(),
            "loss" => self.loss(),
            "draw" => self.draw(),
            _ => {}
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams = HashMap::<&str, Team>::new();

    for line in match_results.lines() {
        let pline: Vec<&str> = line.split(';').collect();
        if pline.len() != 3 {
            continue;
        }
        let (t1_name, t2_name, result) = (pline[0], pline[1], pline[2]);

        teams
            .entry(t1_name)
            .or_insert(Team::new(t1_name.to_owned()))
            .play(result, false);

        teams
            .entry(t2_name)
            .or_insert(Team::new(t2_name.to_owned()))
            .play(result, true);
    }

    let mut result_table = teams.values().collect::<Vec<_>>();

    result_table.sort_unstable_by(|a, b| match b.points.cmp(&a.points) {
        Ordering::Equal => a.name.cmp(&b.name),
        v => v,
    });

    once("Team                           | MP |  W |  D |  L |  P".to_owned())
        .chain(result_table.iter().map(|team| {
            format!(
                "\n{:<30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                team.name.as_str(),
                team.played,
                team.won,
                team.drawn,
                team.lost,
                team.points
            )
        }))
        .collect()
}
