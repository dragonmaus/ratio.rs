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

    let ratio = Ratio::parse(&args[0], &args[1])?;

    println!("{} ({})", ratio.as_decimal_string(), ratio);

    Ok(0)
}
