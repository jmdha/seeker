use clap::Parser;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;
use seeker::search::dfs::DFS;
use seeker::search::solve;
use std::{
    error::Error,
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

#[derive(Parser, Debug)]
struct Args {
    /// Time limit in human time.
    /// Supported units: ns, us, ms, sec, min, hours, days, weeks, months, years (and few variations)
    #[arg(short, long)]
    #[clap(value_parser = humantime::parse_duration)]
    time_limit: Option<Duration>,
    /// Memory limit in MB
    #[arg(short, long)]
    memory_limit: Option<usize>,
    /// Avoids printing plan to stdout
    #[arg(short, long)]
    quiet: bool,
    /// Path to which the plan file will be written
    #[arg(short, long)]
    out: Option<PathBuf>,
    /// Path to domain file
    domain: PathBuf,
    /// Path to problem file
    problem: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("parsing args...");
    let args = Args::parse();
    println!("translating task...");
    let t = Instant::now();
    let task = pddllib::translation::translate_from_file(&args.domain, &args.problem)
        .map_err(|err| format!("{:?}", err))?;
    println!("translation time: {}s", t.elapsed().as_secs_f64());
    println!("type: {}", task.types.len());
    println!("predicate: {}", task.predicates.len());
    println!("action: {}", task.actions.len());
    println!("object: {}", task.objects.len());
    println!("generating searcher...");
    let mut searcher = Box::new(DFS::new(&task.init));
    println!("beginning search...");
    let t = Instant::now();
    let _result = solve(&task, args.time_limit, args.memory_limit, &mut searcher);
    println!("search time: {}s", t.elapsed().as_secs_f64());
    let result = _result?;
    let plan = task.trace_path(&result);
    println!("plan length: {}", plan.len());
    if let Some(out_path) = args.out {
        fs::write(out_path, task.export_plan(&plan))?;
    }
    if !args.quiet {
        println!("{}", task.export_plan(&plan));
    }
    Ok(())
}
