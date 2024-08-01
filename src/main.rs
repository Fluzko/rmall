use std::{
    fs::{self, remove_dir_all},
    io,
    path::Path,
    sync::mpsc::channel,
    thread::available_parallelism,
};

use clap::Parser;
use threadpool::ThreadPool;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(required = true, index = 1)]
    dir: String,

    #[arg(short, default_value = ".")]
    base_path: String,

    #[arg(short)]
    threads: Option<usize>,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let logger = get_logger(args.verbose);

    let available_threads = available_parallelism().unwrap().get();
    let thread_count = args.threads.unwrap_or(available_threads);
    let pool = ThreadPool::new(thread_count);

    let mut directories: Vec<String> = vec![];
    scan_dir(&args.base_path, &mut directories, &args.dir).unwrap();

    directories = directories
        .iter()
        .filter(|dir| {
            Path::new(dir)
                .iter()
                .any(|x| x.to_str().unwrap() == args.dir)
        })
        .cloned()
        .collect();
    let dir_count = directories.len();

    let (tx, rx) = channel();
    for dir in directories {
        let tx = tx.clone();
        pool.execute(move || {
            let result = remove_dir_all(&dir);
            tx.send((dir, result)).unwrap();
        });
    }

    rx.iter()
        .take(dir_count)
        .for_each(|(dir, result)| match result {
            Ok(_) => logger(&format!("Deleted directory {}", dir)),
            Err(e) => logger(&format!("Failed to delete directory {}: {}", dir, e)),
        });
}

fn scan_dir(path: &str, directories: &mut Vec<String>, exclude_dir: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let str_path = path.to_str().unwrap();
            directories.push(str_path.to_string());

            if !path.iter().any(|x| x == exclude_dir) {
                scan_dir(str_path, directories, exclude_dir)?;
            }
        }
    }

    Ok(())
}

fn get_logger(verbose: bool) -> Box<dyn Fn(&str)> {
    if verbose {
        Box::new(move |msg| println!("{}", msg))
    } else {
        Box::new(move |_| {})
    }
}
