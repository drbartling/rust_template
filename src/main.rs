use structopt::StructOpt;

/// Greet a person or entity by name
///
/// NAME is the name of the persom or entity you wish to greet. If not provided
/// we'll greet the whole world!
#[derive(Debug, StructOpt)]
struct Opts {
    /// Name of the person or entity to greet
    #[structopt(default_value = "World")]
    name: String,

    /// Increase the formality of the greeting
    #[structopt(short, long)]
    formal: bool,
}

fn main() {
    let opts = Opts::from_args();

    println!("{}", make_greeting(&opts.name, opts.formal));
}

fn make_greeting(name: &str, formal: bool) -> String {
    if formal {
        format!("Greetings and felicitations, {}!", name)
    } else {
        format!("Hello, {}!", name)
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use self::rstest::rstest;
    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest(
        name,
        formal,
        expected,
        case::bob_casual("Bob", false, "Hello, Bob!"),
        case::sally_formal("Sally", true, "Greetings and felicitations, Sally!")
    )]
    fn test_boundary_sliver_removal(name: &str, formal: bool, expected: &str) {
        let result = make_greeting(name, formal);
        assert_eq!(expected, &result);
    }
}
