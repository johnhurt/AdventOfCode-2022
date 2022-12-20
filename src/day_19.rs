use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::i32,
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

const MAX_RAND: u32 = 1_000_000;

const SAMPLES: usize = 1000000;

#[derive(Debug, Clone, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse_ore_bot(input: &str) -> IResult<&'_ str, i32> {
    preceded(
        tuple((tag("Blueprint "), i32, tag(": Each ore robot costs "))),
        i32,
    )(input)
}

fn parse_clay_bot(input: &str) -> IResult<&'_ str, i32> {
    preceded(tag(" ore. Each clay robot costs "), i32)(input)
}

fn parse_obsidian_bot(input: &str) -> IResult<&'_ str, (i32, i32)> {
    preceded(
        tag(" ore. Each obsidian robot costs "),
        separated_pair(i32, tag(" ore and "), i32),
    )(input)
}

fn parse_geode_bot(input: &str) -> IResult<&'_ str, (i32, i32)> {
    preceded(
        tag(" clay. Each geode robot costs "),
        separated_pair(i32, tag(" ore and "), i32),
    )(input)
}

#[derive(Debug)]
struct Blueprint {
    ore_bot_cost: i32,
    clay_bot_cost: i32,
    obsidian_bot_cost: (i32, i32),
    geode_bot_cost: (i32, i32),
}

fn parse_blueprint(line: String) -> Blueprint {
    let result: IResult<&str, Blueprint> = map(
        tuple((
            parse_ore_bot,
            parse_clay_bot,
            parse_obsidian_bot,
            parse_geode_bot,
        )),
        |(o, c, x, g)| Blueprint {
            ore_bot_cost: o,
            clay_bot_cost: c,
            obsidian_bot_cost: x,
            geode_bot_cost: g,
        },
    )(&line);

    result.expect("🤡").1
}

#[derive(Debug, Default)]
struct Inventory {
    ore_bots: i32,
    ore: i32,
    clay_bots: i32,
    clay: i32,
    obsidian_bots: i32,
    obsidian: i32,
    geode_bots: i32,
    geodes: i32,
}

impl Inventory {
    fn can_buy_ore_bot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.ore_bot_cost
    }

    fn can_buy_clay_bot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.clay_bot_cost
    }

    fn can_buy_obsidian_bot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.obsidian_bot_cost.0
            && self.clay >= bp.obsidian_bot_cost.1
    }

    fn can_buy_geode_bot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.geode_bot_cost.0 && self.obsidian >= bp.geode_bot_cost.1
    }
}

trait Strategy {
    fn buy_ore_bot(&mut self, bp: &Blueprint, inventory: &Inventory) -> bool;

    fn buy_clay_bot(&mut self, bp: &Blueprint, inventory: &Inventory) -> bool;

    fn buy_obsidian_bot(
        &mut self,
        bp: &Blueprint,
        inventory: &Inventory,
    ) -> bool;

    fn buy_geode_bot(&mut self, bp: &Blueprint, inventory: &Inventory) -> bool;
}

/// I had no idea how to solve this problem in a real way, so I tried a
/// stochastic approach (that worked somehow). Each time a decision needs to be
/// made, we sample a random number and compare it to a threshold specific to
/// the decision being made. Running this a bunch of times reliably produces the
/// optimal results ... yay?
#[derive(Debug)]
struct RandomStrategy {
    rng: ThreadRng,
    thresholds: (u32, u32, u32, u32),
}

impl Strategy for RandomStrategy {
    fn buy_ore_bot(&mut self, bp: &Blueprint, inventory: &Inventory) -> bool {
        self.rng.gen_range(0..=MAX_RAND) > self.thresholds.0
    }

    fn buy_clay_bot(&mut self, bp: &Blueprint, inventory: &Inventory) -> bool {
        self.rng.gen_range(0..=MAX_RAND) > self.thresholds.1
    }

    fn buy_obsidian_bot(
        &mut self,
        bp: &Blueprint,
        inventory: &Inventory,
    ) -> bool {
        self.rng.gen_range(0..=MAX_RAND) > self.thresholds.2
    }

    fn buy_geode_bot(&mut self, bp: &Blueprint, inventory: &Inventory) -> bool {
        self.rng.gen_range(0..=MAX_RAND) > self.thresholds.3
    }
}

fn eval_blueprints<S>(bp: &Blueprint, strategy: &mut S, minutes: i32) -> i32
where
    S: Strategy,
{
    let mut inventory = Inventory {
        ore_bots: 1,
        ..Inventory::default()
    };

    for _min in 1..=minutes {
        let mut news = (0, 0, 0, 0);
        if inventory.can_buy_geode_bot(bp)
            && strategy.buy_geode_bot(bp, &inventory)
        {
            news.3 = 1;
            inventory.ore -= bp.geode_bot_cost.0;
            inventory.obsidian -= bp.geode_bot_cost.1;
        } else if inventory.can_buy_obsidian_bot(bp)
            && strategy.buy_obsidian_bot(bp, &inventory)
        {
            news.2 = 1;
            inventory.ore -= bp.obsidian_bot_cost.0;
            inventory.clay -= bp.obsidian_bot_cost.1;
        } else if inventory.can_buy_clay_bot(bp)
            && strategy.buy_clay_bot(bp, &inventory)
        {
            news.1 = 1;
            inventory.ore -= bp.clay_bot_cost;
        } else if inventory.can_buy_ore_bot(bp)
            && strategy.buy_ore_bot(bp, &inventory)
        {
            news.0 = 1;
            inventory.ore -= bp.ore_bot_cost;
        }

        inventory.ore += inventory.ore_bots;
        inventory.clay += inventory.clay_bots;
        inventory.obsidian += inventory.obsidian_bots;
        inventory.geodes += inventory.geode_bots;

        inventory.ore_bots += news.0;
        inventory.clay_bots += news.1;
        inventory.obsidian_bots += news.2;
        inventory.geode_bots += news.3;
    }

    inventory.geodes
}

fn run_random_averaged_step<S>(
    bp: &Blueprint,
    strategy: &mut S,
    minutes: i32,
) -> (i32, f64)
where
    S: Strategy,
{
    let mut strategy = RandomStrategy {
        rng: thread_rng(),
        thresholds: (MAX_RAND >> 1, MAX_RAND >> 1, MAX_RAND >> 3, 0),
    };

    let mut avg_geodes = 0.0;
    let mut max: i32 = 0;

    for _ in 0..SAMPLES {
        let result = eval_blueprints(bp, &mut strategy, minutes);
        avg_geodes += result as f64 / SAMPLES as f64;
        max = max.max(result);
    }

    (max, avg_geodes)
}

fn run_random_strategy(bp: Blueprint, minutes: i32) -> i32 {
    let mut strategy = RandomStrategy {
        rng: thread_rng(),
        thresholds: (MAX_RAND >> 1, MAX_RAND >> 1, 0, 0),
    };

    let result = run_random_averaged_step(&bp, &mut strategy, minutes);

    println!("\n{:?}", result);

    result.0
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_blueprint)
        .map(|bp| run_random_strategy(bp, 24))
        .enumerate()
        .map(|(i, o)| (i as i32 + 1) * o)
        .sum::<i32>()
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    input_lines
        .map(parse_blueprint)
        .take(3)
        .map(|bp| run_random_strategy(bp, 32))
        .product::<i32>()
}
