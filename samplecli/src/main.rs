use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("fomula_file")
            .value_name("FILE")
            .index(1)
            .required(false),
        )
        .arg(
            Arg::new("verbose")
            .short('v')  // charなのでシングルクォート
            .long("verbose")
            .required(false),
        )
        .get_matches();

    match matches.value_of("fomula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specifed?: {}", verbose);
}