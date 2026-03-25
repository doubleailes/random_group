use std::fs;
use clap::{ArgGroup, Parser};

/// Randomize a group of people from a text file (one name per line).
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["num_groups", "group_size"]),
))]
struct Cli {
    /// Path to the text file containing one name per line
    file: String,

    /// Number of groups to create
    #[arg(short = 'n', long)]
    num_groups: Option<usize>,

    /// Size of each group (last group may be smaller)
    #[arg(short = 's', long)]
    group_size: Option<usize>,
}

fn main() {
    // Set UTF-8 output on Windows so names with accented characters render correctly
    #[cfg(target_os = "windows")]
    {
        unsafe extern "system" {
            fn SetConsoleOutputCP(wCodePageID: u32) -> i32;
        }
        unsafe { SetConsoleOutputCP(65001) };
    }

    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.file)
        .unwrap_or_else(|e| {
            eprintln!("Error reading '{}': {}", cli.file, e);
            std::process::exit(1);
        });

    let mut people: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    if people.is_empty() {
        eprintln!("No names found in file.");
        std::process::exit(1);
    }

    fastrand::shuffle(&mut people);

    let total = people.len();

    if let Some(n) = cli.num_groups {
        if n == 0 {
            eprintln!("Number of groups must be at least 1.");
            std::process::exit(1);
        }
        let n = n.min(total);
        // Distribute as evenly as possible: first `remainder` groups get one extra person
        let base = total / n;
        let remainder = total % n;
        let mut offset = 0;
        for i in 0..n {
            let size = if i < remainder { base + 1 } else { base };
            println!("Group {}:", i + 1);
            for name in &people[offset..offset + size] {
                println!("  - {}", name);
            }
            offset += size;
        }
    } else {
        let s = cli.group_size.unwrap();
        if s == 0 {
            eprintln!("Group size must be at least 1.");
            std::process::exit(1);
        }
        for (i, group) in people.chunks(s).enumerate() {
            println!("Group {}:", i + 1);
            for name in group {
                println!("  - {}", name);
            }
        }
    }
}

