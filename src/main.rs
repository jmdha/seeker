mod heuristic;
mod search;

use clap::Parser;
use fxhash::FxBuildHasher;
use heuristic::goal_count::GoalCount;
use indexmap::IndexMap;
use search::{lgbfs::LGBFS, solve};
use std::{error::Error, fs, path::PathBuf, time::Duration};

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    #[clap(value_parser = humantime::parse_duration)]
    time_limit: Option<Duration>,
    #[arg(short, long)]
    memory_limit: Option<usize>,
    #[arg(short, long)]
    out: Option<PathBuf>,
    domain: PathBuf,
    problem: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Parsing args");
    let args = Args::parse();
    println!("Translating task");
    let task = pddllib::translation::translate_from_file(&args.domain, &args.problem)
        .map_err(|err| format!("{:?}", err))?;
    println!("Types: {}", task.types.len());
    println!("Predicates: {}", task.predicates.len());
    println!("Actions: {}", task.actions.len());
    println!("Objects: {}", task.objects.len());
    println!("Generating searcher");
    let mut searcher = Box::new(LGBFS::new(&task.init, Box::new(GoalCount::new())));
    println!("Beginning search");
    let _result = solve(&task, args.time_limit, args.memory_limit, &mut searcher)?;
    let plan = task.trace_path(&_result);
    if let Some(out_path) = args.out {
        fs::write(out_path, task.export_plan(&plan))?;
    } else {
        println!("{}", task.export_plan(&plan));
    }
    Ok(())
}
