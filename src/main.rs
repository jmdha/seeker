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
    let args = Args::parse();
    let task = pddllib::translation::translate_from_file(&args.domain, &args.problem)
        .map_err(|err| format!("{:?}", err))?;
    let mut searcher = Box::new(LGBFS::new(&task.init, &GoalCount {}));
    let _result = solve(&task, args.time_limit, args.memory_limit, &mut searcher)?;
    let plan = task.trace_path(&_result);
    if let Some(out_path) = args.out {
        fs::write(out_path, task.export_plan(&plan))?;
    } else {
        println!("{}", task.export_plan(&plan));
    }
    Ok(())
}
