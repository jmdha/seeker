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
    let t_begin = Instant::now();
    let args = Args::parse();
    let t_preprocessing = Instant::now();
    println!("Reading files...");
    let t = Instant::now();
    println!("Reading domain...");
    let domain = fs::read_to_string(&args.domain)?;
    println!("Reading problem...");
    let problem = fs::read_to_string(&args.problem)?;
    println!("Read time: {}s", t.elapsed().as_secs_f64());
    println!("Parsing files...");
    let t = Instant::now();
    let domain = pddlp::domain::parse(&domain).unwrap();
    let problem = pddlp::problem::parse(&problem).unwrap();
    println!("Parse time: {}s", t.elapsed().as_secs_f64());
    println!("Translating task...");
    let t = Instant::now();
    let task = pddllib::translation::translate_parsed(&domain, &problem)?;
    println!("Translation time: {}s", t.elapsed().as_secs_f64());
    println!("Preprocessing time: {}s", t_preprocessing.elapsed().as_secs_f64());
    println!("Types: {}", task.types.len());
    println!("Predicates: {}", task.predicates.len());
    println!("Actions: {}", task.actions.len());
    println!("Objects: {}", task.objects.len());
    println!("Static facts: {}", task.static_facts.len());
    println!("Initial state facts: {}", task.init.fact_count());
    if args.search.is_none() {
        println!("No search algorithm specified, exiting");
        return Ok(());
    }
    println!("Generating searcher...");
    let mut searcher = search::generate(&task, args.search.unwrap());
    println!("Beginning search...");
    let t = Instant::now();
    let _result = solve(&task, args.time_limit, args.memory_limit, &mut searcher);
    let result = _result?;
    let plan = task.trace_path(&result);
    if let Some(out_path) = args.out {
        fs::write(out_path, task.export_plan(&plan))?;
    }
    if !args.quiet {
        print!("{}", task.export_plan(&plan));
    }
    println!(
        "Search time: {}s ({:.2}%)",
        t.elapsed().as_secs_f64(),
        t.elapsed().as_secs_f64() / t_begin.elapsed().as_secs_f64() * 100.0
    );
    println!("Total time: {}s", t_begin.elapsed().as_secs_f64());
    println!("Plan length: {}", plan.len());
    Ok(())
}
