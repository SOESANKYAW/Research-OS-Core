mod core;
mod dbm;
mod mtbdd;
mod ctl;
mod mdp;
mod engine;
mod utils;
mod bellman;
mod fixpoint;

use engine::checker::ModelChecker;
use ctl::syntax::CTLFormula;
use core::state::StateId;
use std::collections::HashSet;

fn main() {
    env_logger::init();

    let mut init = HashSet::new();
    init.insert(StateId(1));

    let phi = CTLFormula::True;

    let result = ModelChecker::check(phi, init);

    println!("ASS_v3 symbolic evaluation: {}", result);
}
