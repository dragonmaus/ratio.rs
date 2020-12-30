use std::io;

program::main!("ratio");

fn usage_line(program_name: &str) -> String {
    format!("Usage: {} x y [z...]", program_name)
}

fn program(name: &str) -> program::Result {
    let args = program::args().split_off(1);

    if args.len() < 2 {
        eprintln!("{}", usage_line(name));
        return Ok(1);
    }

    let numbers = args
        .iter()
        .map(|s| {
            s.trim().parse::<u64>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unable to parse '{}': {}", s, e),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let gcd = gcd(&numbers);
    let ratios: Vec<u64> = numbers.iter().map(|n| n / gcd).collect();
    if ratios.len() == 2 {
        let x = ratios[0];
        let y = ratios[1];
        let r = x as f64 / y as f64;
        let mut r = r.to_string();
        r.truncate(10);
        println!("{} ({}:{})", r, x, y);
    } else {
        println!(
            "{}",
            ratios
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(":")
        );
    }
    Ok(0)
}

fn gcd(numbers: &[u64]) -> u64 {
    let mut gcd = *numbers.iter().min().unwrap_or(&1);

    while gcd > 1 {
        if numbers.iter().all(|n| n % gcd == 0) {
            break;
        }
        gcd -= 1;
    }

    gcd
}
