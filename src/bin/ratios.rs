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

    let xmax = args[0].trim().parse::<u64>()?;
    let ymax = args[1].trim().parse::<u64>()?;

    // TODO: put this in a config file?
    let want = vec![
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

    for x in 2..=xmax {
        for y in 2..=ymax {
            let r = Ratio::new(x, y);
            if want.contains(&r.into()) {
                best.insert(r.as_pair(), r);
            }
        }
    }

    for p in &want {
        let r = best[p];
        let f: f64 = r.into();
        let mut s = f.to_string();
        s.truncate(10);
        println!("{} ({}) => {}", s, r, r.original());
    }

    Ok(0)
}
