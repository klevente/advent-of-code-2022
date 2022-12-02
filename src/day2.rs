use advent_of_code_2022::read_file_lines_as;
use sscanf::scanf;

fn parse_line_into_chars(s: &str) -> (char, char) {
    scanf!(s, "{char} {char}").unwrap()
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_char_for_elf(c: char) -> Self {
        match c {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_char_for_player(c: char) -> Self {
        match c {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => unreachable!(),
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
struct RoundWithChoices {
    player_choice: Choice,
    elf_choice: Choice,
}

impl RoundWithChoices {
    fn new(elf_choice_ch: char, player_choice_ch: char) -> Self {
        let elf_choice = Choice::from_char_for_elf(elf_choice_ch);
        let player_choice = Choice::from_char_for_player(player_choice_ch);

        Self {
            player_choice,
            elf_choice,
        }
    }

    fn calculate_score(&self) -> u32 {
        let outcome = match (&self.player_choice, &self.elf_choice) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Outcome::Loss,
        };

        let choice_score = self.player_choice.get_score();
        let outcome_score = outcome.get_score();

        choice_score + outcome_score
    }
}

fn calculate_total_score_for_rounds_with_choices(rounds: &Vec<RoundWithChoices>) -> u32 {
    rounds.iter().map(|r| r.calculate_score()).sum()
}

struct RoundsWithElfChoiceAndOutcome {
    elf_choice: Choice,
    outcome: Outcome,
}

impl RoundsWithElfChoiceAndOutcome {
    fn new(choice_ch: char, outcome_ch: char) -> Self {
        let elf_choice = Choice::from_char_for_elf(choice_ch);
        let outcome = Outcome::from_char(outcome_ch);

        Self {
            elf_choice,
            outcome,
        }
    }

    fn calculate_score(&self) -> u32 {
        let player_choice = match (&self.outcome, &self.elf_choice) {
            (Outcome::Win, Choice::Rock)
            | (Outcome::Draw, Choice::Paper)
            | (Outcome::Loss, Choice::Scissors) => Choice::Paper,
            (Outcome::Win, Choice::Paper)
            | (Outcome::Draw, Choice::Scissors)
            | (Outcome::Loss, Choice::Rock) => Choice::Scissors,
            (Outcome::Win, Choice::Scissors)
            | (Outcome::Draw, Choice::Rock)
            | (Outcome::Loss, Choice::Paper) => Choice::Rock,
        };

        let choice_score = player_choice.get_score();
        let outcome_score = self.outcome.get_score();

        choice_score + outcome_score
    }
}

fn calculate_total_score_for_rounds_with_choice_and_outcome(
    rounds: &Vec<RoundsWithElfChoiceAndOutcome>,
) -> u32 {
    rounds.iter().map(|r| r.calculate_score()).sum()
}

fn main() {
    let lines_parsed = read_file_lines_as("input/day2.txt", parse_line_into_chars);
    let rounds_with_choices = lines_parsed
        .iter()
        .map(|(a, b)| RoundWithChoices::new(*a, *b))
        .collect::<Vec<_>>();
    let total_score_rounds_with_choices =
        calculate_total_score_for_rounds_with_choices(&rounds_with_choices);
    println!("The total score for rounds with choices if everything goes exactly according to the strategy guide is {total_score_rounds_with_choices}");

    let rounds_with_choice_and_outcome = lines_parsed
        .iter()
        .map(|(a, b)| RoundsWithElfChoiceAndOutcome::new(*a, *b))
        .collect::<Vec<_>>();
    let total_score_rounds_with_choice_and_outcome =
        calculate_total_score_for_rounds_with_choice_and_outcome(&rounds_with_choice_and_outcome);
    println!("The total score for rounds with choice and outcome if everything goes exactly according to the strategy guide is {total_score_rounds_with_choice_and_outcome}");
}
