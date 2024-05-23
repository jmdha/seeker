use anyhow::Result;
use clap::Parser;
use seeker::search::SearchKind;
use seeker::search::{self, solve};
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

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
    #[command(subcommand)]
    search: Option<SearchKind>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("reading files...");
    let t = Instant::now();
    let domain = fs::read_to_string(&args.domain)?;
    let problem = fs::read_to_string(&args.problem)?;
    println!("read time: {}", t.elapsed().as_secs_f64());
    println!("parsing files...");
    let t = Instant::now();
    let domain = pddlp::domain::parse(&domain).unwrap();
    let problem = pddlp::problem::parse(&problem).unwrap();
    println!("parse time: {}", t.elapsed().as_secs_f64());
    println!("translating task...");
    let t = Instant::now();
    let task = pddllib::translation::translate_parsed(&domain, &problem)?;
    println!("translation time: {}s", t.elapsed().as_secs_f64());
    println!("type: {}", task.types.len());
    println!("predicate: {}", task.predicates.len());
    println!("action: {}", task.actions.len());
    println!("object: {}", task.objects.len());
    if args.search.is_none() {
        println!("no search algorithm specified, exiting");
        return Ok(());
    }
    println!("generating searcher...");
    let mut searcher = search::generate(&task, args.search.as_ref().unwrap());
    println!("beginning search...");
    let t = Instant::now();
    let _result = solve(&task, args.time_limit, args.memory_limit, &mut searcher);
    let result = _result?;
    let plan = task.trace_path(&result);
    if let Some(out_path) = args.out {
        fs::write(out_path, task.export_plan(&plan))?;
    }
    if !args.quiet {
        println!("{}", task.export_plan(&plan));
    }
    println!("search time: {}s", t.elapsed().as_secs_f64());
    println!("plan length: {}", plan.len());
    Ok(())
}
