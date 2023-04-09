use std::{
    io::{stdout, Write},
    time::Duration,
};

use crate::{highscores::ScoreInfo, lotto::LottoResult};
use colored::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
enum Colour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Colour {
    pub fn colourize_text(&self, text: &str) -> ColoredString {
        match self {
            Colour::Black => text.black(),
            Colour::Red => text.red(),
            Colour::Green => text.green(),
            Colour::Yellow => text.yellow(),
            Colour::Blue => text.blue(),
            Colour::Magenta => text.magenta(),
            Colour::Cyan => text.cyan(),
        }
    }
}

pub trait TerminalOutputer {
    fn pre_commit(&self);
    fn post_commit(&self, result: &LottoResult);
    fn failed(&self);
    fn high_score(&self, new: &ScoreInfo, old: &ScoreInfo);
}

pub struct TerminalOutputerImpl;

impl TerminalOutputerImpl {
    pub fn new() -> Self {
        TerminalOutputerImpl {}
    }

    fn base_colour() -> Colour {
        Colour::Black
    }

    fn score_colours() -> Vec<Colour> {
        let mut res = vec![
            Colour::Red,
            Colour::Green,
            Colour::Yellow,
            Colour::Blue,
            Colour::Magenta,
            Colour::Cyan,
        ];
        let mut rng = thread_rng();
        res.shuffle(&mut rng);
        res
    }

    fn colourized_hash(results: &LottoResult) -> Vec<ColoredString> {
        let mut hash: Vec<ColoredString> = results
            .hash
            .chars()
            .map(|c| Self::base_colour().colourize_text(&c.to_string()).bold())
            .collect();

        for (rule, colour) in results
            .rules
            .iter()
            .flat_map(|r| r.positions())
            .zip(Self::score_colours())
        {
            for position in rule {
                hash[position] = colour.colourize_text(&hash[position]).bold();
            }
        }

        hash
    }
}

impl TerminalOutputer for TerminalOutputerImpl {
    fn pre_commit(&self) {
        println!("Committing ...");
    }

    fn post_commit(&self, result: &LottoResult) {
        let mut out = stdout();
        let mut flush = || out.flush().expect("could not flush stdout");
        println!("done!");
        print!("Your commit hash is ... ");
        flush();
        for c in Self::colourized_hash(result) {
            std::thread::sleep(Duration::from_millis(200));
            print!("{}", c);
            flush();
        }
        println!();
        for rule in &result.rules {
            println!("> {} - {}", rule.name().cyan(), rule.description().cyan());
            println!(
                "    {} {}",
                "Points: ".cyan(),
                rule.points().to_string().cyan()
            );
        }
        let points = result.total_points();
        println!(
            "{}{}{}",
            "Total points: ".magenta(),
            points.to_string().magenta().bold(),
            (if points == 0 { " :(" } else { "" }).magenta()
        )
    }

    fn failed(&self) {
        println!("{}", "Failed to commit".red());
    }

    fn high_score(&self, new: &ScoreInfo, old: &ScoreInfo) {
        println!(
            "{}{}{}",
            "New high score! ".green(),
            new.score.to_string().green().bold(),
            " points!".green()
        );
        println!("Previous high score:");
        println!(
            "    {} points from {}: {}",
            old.score.to_string().bold(),
            old.commit.bold(),
            old.rules.join(", ")
        );
    }
}
