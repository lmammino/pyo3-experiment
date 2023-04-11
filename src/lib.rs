use std::collections::{HashSet, VecDeque};

use pyo3::prelude::*;
use pyo3::types::PyDict;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default)]
struct State {
    robots_ore: u32,
    robots_clay: u32,
    robots_obsidian: u32,
    robots_geode: u32,
    resources_ore: u32,
    resources_clay: u32,
    resources_obsidian: u32,
    resources_geode: u32,
    timer: u32,
}

impl State {
    fn new(
        robots_ore: u32,
        robots_clay: u32,
        robots_obsidian: u32,
        robots_geode: u32,
        resources_ore: u32,
        resources_clay: u32,
        resources_obsidian: u32,
        resources_geode: u32,
        timer: u32,
    ) -> Self {
        State {
            robots_ore,
            robots_clay,
            robots_obsidian,
            robots_geode,
            resources_ore,
            resources_clay,
            resources_obsidian,
            resources_geode,
            timer,
        }
    }
}

/// Simulate extraction
#[pyfunction]
fn run_blueprint(blueprint: &PyDict, time_limit: u32) -> PyResult<u32> {
    let bp_ore_ore: u32 = blueprint
        .get_item("ore")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let bp_clay_ore: u32 = blueprint
        .get_item("clay")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let bp_obs_ore: u32 = blueprint
        .get_item("obsidian")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let bp_obs_clay: u32 = blueprint
        .get_item("obsidian")
        .unwrap()
        .get_item("clay")
        .unwrap()
        .extract()
        .unwrap();
    let bp_geo_ore: u32 = blueprint
        .get_item("geode")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let bp_geo_obs: u32 = blueprint
        .get_item("geode")
        .unwrap()
        .get_item("obsidian")
        .unwrap()
        .extract()
        .unwrap();

    let max_ore = bp_ore_ore.max(bp_clay_ore).max(bp_obs_ore).max(bp_geo_ore);

    let mut stack: VecDeque<State> = VecDeque::new();
    stack.push_back(State {
        robots_ore: 1,
        robots_clay: 0,
        robots_obsidian: 0,
        robots_geode: 0,
        resources_ore: 0,
        resources_clay: 0,
        resources_obsidian: 0,
        resources_geode: 0,
        timer: 0,
    });
    let mut seen: HashSet<State> = Default::default();

    let mut ret_max = 0;
    let mut gen = 0;
    let mut geodes_at_gen = 0;

    while let Some(state) = stack.pop_front() {        
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());

        if state.timer == time_limit {
            if state.resources_geode > ret_max {
                ret_max = state.resources_geode;
            }
            continue;
        }

        gen = gen.max(state.timer);
        geodes_at_gen = geodes_at_gen.max(state.robots_geode);

        if state.timer == gen && state.robots_geode < geodes_at_gen {
            continue;
        }

        // no robot build, just storage
        stack.push_back(State::new(
            state.robots_ore,
            state.robots_clay,
            state.robots_obsidian,
            state.robots_geode,
            state.robots_ore + state.resources_ore,
            state.robots_clay + state.resources_clay,
            state.robots_obsidian + state.resources_obsidian,
            state.robots_geode + state.resources_geode,
            state.timer + 1,
        ));

        // create an ore robot
        if bp_ore_ore <= state.resources_ore && state.robots_ore < max_ore {
            stack.push_back(State::new(
                state.robots_ore+1,
                state.robots_clay,
                state.robots_obsidian,
                state.robots_geode,
                state.robots_ore + state.resources_ore- bp_ore_ore,
                state.robots_clay + state.resources_clay,
                state.robots_obsidian + state.resources_obsidian,
                state.robots_geode + state.resources_geode,
                state.timer + 1,
            ));
        }

        // clay robot
        if bp_clay_ore <= state.resources_ore && state.robots_clay < bp_obs_clay {
            stack.push_back(State::new(
                state.robots_ore,
                state.robots_clay+1,
                state.robots_obsidian,
                state.robots_geode,
                state.robots_ore + state.resources_ore- bp_clay_ore,
                state.robots_clay + state.resources_clay,
                state.robots_obsidian + state.resources_obsidian,
                state.robots_geode + state.resources_geode,
                state.timer + 1,
            ));
        }

        //obsidian robot
        if bp_obs_ore <= state.resources_ore
            && bp_obs_clay <= state.resources_clay
            && state.robots_obsidian < bp_geo_obs {
            stack.push_back(State::new(
                state.robots_ore,
                state.robots_clay,
                state.robots_obsidian+1,
                state.robots_geode,
                state.robots_ore + state.resources_ore- bp_obs_ore,
                state.robots_clay + state.resources_clay- bp_obs_clay,
                state.robots_obsidian + state.resources_obsidian,
                state.robots_geode + state.resources_geode,
                state.timer + 1,
            ));
        }

        //geode robot
        if bp_geo_ore <= state.resources_ore && bp_geo_obs <= state.resources_obsidian {
            stack.push_back(State::new(
                state.robots_ore,
                state.robots_clay,
                state.robots_obsidian,
                state.robots_geode+1,
                state.robots_ore + state.resources_ore-bp_geo_ore,
                state.robots_clay + state.resources_clay,
                state.robots_obsidian + state.resources_obsidian- bp_geo_obs,
                state.robots_geode + state.resources_geode,
                state.timer + 1,
            ));
        }
    }

    Ok(ret_max)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_streaming_exercise(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_blueprint, m)?)?;
    Ok(())
}
