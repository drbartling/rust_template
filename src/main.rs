use clap::Parser;
use klask::Settings;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        klask::run_derived::<Args, _>(Settings::default(), app_main);
    } else {
        let args = Args::parse();
        app_main(args);
    }
}

fn app_main(args: Args) {
    for _ in 0..args.count {
        println!("{}", make_greeting(&args.name));
    }
}

fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use self::rstest::rstest;
    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case("Bob", "Hello, Bob!")]
    #[case("Sally", "Hello, Sally!")]
    fn test_make_greeting(#[case] name: &str, #[case] expected: &str) {
        let result = make_greeting(name);
        assert_eq!(expected, &result);
    }
}
