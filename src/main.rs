use ratio::Ratio;

program::main!("ratio");

fn usage_line(program_name: &str) -> String {
    format!("Usage: {} x y", program_name)
}

fn program(name: &str) -> program::Result {
    let args = program::args().split_off(1);

    if args.len() != 2 {
        eprintln!("{}", usage_line(name));
        return Ok(1);
    }

    let x = args[0].trim().parse::<u64>()?;
    let y = args[1].trim().parse::<u64>()?;
    let ratio = Ratio::new(x, y);
    let number = {
        let f: f64 = ratio.into();
        let mut s = f.to_string();
        s.truncate(10);
        s
    };

    println!("{} ({})", number, ratio);

    Ok(0)
}
