use advent_of_code_2022::read_file_lines_as;
use sscanf::scanf;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, ore_robot_ore_cost, clay_robot_ore_cost, obsidian_robot_ore_cost, obsidian_robot_clay_cost, geode_robot_ore_cost, geode_robot_obsidian_cost) = scanf!(s, "Blueprint {u32}: Each ore robot costs {u32} ore. Each clay robot costs {u32} ore. Each obsidian robot costs {u32} ore and {u32} clay. Each geode robot costs {u32} ore and {u32} obsidian.").map_err(|_| "Invalid format".to_string())?;

        Ok(Self {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        })
    }
}

impl Blueprint {
    fn max_ore_cost(&self) -> u32 {
        self.ore_robot_ore_cost
            .max(self.clay_robot_ore_cost)
            .max(self.obsidian_robot_ore_cost)
            .max(self.geode_robot_ore_cost)
    }
}

#[derive(Clone, Debug)]
struct RobotFactory {
    blueprint: Blueprint,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    elapsed_time: u32,
}

impl RobotFactory {
    fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            elapsed_time: 1,
        }
    }

    fn collect_resources(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn solve(&mut self) -> u32 {
        let max_ore_cost = self.blueprint.max_ore_cost();
        if self.elapsed_time == 24 {
            self.collect_resources();
            return self.geode;
        }
        let mut best = 0;
        if self.ore >= self.blueprint.ore_robot_ore_cost && self.ore_robots < max_ore_cost {
            let mut copy = self.clone();
            copy.ore -= copy.blueprint.ore_robot_ore_cost;
            copy.collect_resources();
            copy.elapsed_time += 1;
            copy.ore_robots += 1;
            best = best.max(copy.solve());
        }

        if self.ore >= self.blueprint.clay_robot_ore_cost
            && self.clay_robots < self.blueprint.obsidian_robot_clay_cost
        {
            let mut copy = self.clone();
            copy.ore -= copy.blueprint.clay_robot_ore_cost;
            copy.collect_resources();
            copy.elapsed_time += 1;
            copy.clay_robots += 1;
            best = best.max(copy.solve());
        }

        if self.ore >= self.blueprint.obsidian_robot_ore_cost
            && self.clay >= self.blueprint.obsidian_robot_clay_cost
            && self.obsidian_robots < self.blueprint.geode_robot_obsidian_cost
        {
            let mut copy = self.clone();
            copy.ore -= copy.blueprint.obsidian_robot_ore_cost;
            copy.clay -= copy.blueprint.obsidian_robot_clay_cost;
            copy.collect_resources();
            copy.obsidian_robots += 1;
            copy.elapsed_time += 1;
            best = best.max(copy.solve());
        }

        if self.ore >= self.blueprint.geode_robot_ore_cost
            && self.obsidian >= self.blueprint.geode_robot_obsidian_cost
        {
            let mut copy = self.clone();
            copy.ore -= copy.blueprint.geode_robot_ore_cost;
            copy.obsidian -= copy.blueprint.geode_robot_obsidian_cost;
            copy.collect_resources();
            copy.elapsed_time += 1;
            copy.geode_robots += 1;
            best = best.max(copy.solve());
        }

        let mut copy = self.clone();
        copy.collect_resources();
        copy.elapsed_time += 1;
        best = best.max(copy.solve());
        best
    }

    fn calculate_quality_level(&mut self) -> u32 {
        let max_number_of_geodes = self.solve();
        max_number_of_geodes * self.blueprint.id
    }
}

fn calculate_sum_of_quality_levels(factories: Vec<RobotFactory>) -> u32 {
    factories
        .into_iter()
        .map(|mut f| {
            println!("Processing {}...", f.blueprint.id);
            f.calculate_quality_level()
        })
        .sum()
}

fn main() {
    let factories = read_file_lines_as("input/day19.txt", |l| {
        let blueprint = Blueprint::from_str(l).unwrap();
        RobotFactory::new(blueprint)
    });
    let sum_of_quality_levels = calculate_sum_of_quality_levels(factories);
    println!("The sum of quality levels is {sum_of_quality_levels}");
}
