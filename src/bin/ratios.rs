use ratio::{Pair, Ratio};
use std::collections::HashMap;

program::main!("ratios");

fn usage_line(program_name: &str) -> String {
    format!("Usage: {} x y", program_name)
}

fn program(name: &str) -> program::Result {
    let args = program::args().split_off(1);

    if args.len() != 2 {
        eprintln!("{}", usage_line(name));
        return Ok(1);
    }

    let max = Pair::parse(&args[0], &args[1])?;
    // TODO: put this in a config file?
    let want = vec![
        Pair(6, 13),
        Pair(1, 2),
        Pair(9, 16),
        Pair(3, 4),
        Pair(1, 1),
        Pair(5, 4),
        Pair(4, 3),
        Pair(8, 5),
        Pair(16, 9),
        Pair(16, 5),
        Pair(32, 9),
    ];
    let mut best = HashMap::new();

    for x in 1..=max.x() {
        for y in 1..=max.y() {
            let r = Ratio::new(x, y);
            if want.contains(&r.into()) {
                best.insert(r.as_pair(), r);
            }
        }
    }

    for p in &want {
        if let Some(r) = best.get(p) {
            println!(
                "{} ({}) => {} [{}]",
                r.as_decimal_string(),
                r,
                r.original(),
                percent_string(max, r.original())
            );
        }
    }

    Ok(0)
}

fn percent_string(old: Pair, new: Pair) -> String {
    let old = (old.x() * old.y()) as f64;
    let new = (new.x() * new.y()) as f64;
    format!("{:>2}%", (new / old * 100.0).round())
}
