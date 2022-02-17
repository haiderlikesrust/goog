pub mod errors;
const GOOG_VERSION: &str = "0.1.2";
use clap;
use console::Term;
use dialoguer::Select;
use errors::*;

pub fn print_os_info() {
    let info = os_info::get();
    colour::cyan!("Running Goog on:\n");
    colour::blue_ln!("OS: {}\n", info.os_type());
    colour::e_yellow_ln!("Goog Version: {}\n", GOOG_VERSION)
}

pub fn request_search(query: String, with: Option<String>) -> Result<(), GoogError> {
    let query = format!(
        "
    https://www.google.com/search?q={}
    ",
        query
    );
    match with {
        Some(e) => Ok(open::with(&query, &e)?),
        None => Ok(open::that(&query)?),
    }
}

pub fn run() -> Result<(), GoogError> {
    let app = clap::App::new("goog")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("Run Google in your terminal!")
        .arg(
            clap::Arg::new("search")
                .takes_value(true)
                .value_name("QUERY")
                .short('s'),
        )
        .subcommand(
            clap::App::new("select")
                .about("Select your browser and the command will automatically run.")
                .alias("s")
                .arg(
                    clap::Arg::new("search")
                        .takes_value(true)
                        .value_name("QUERY")
                        .short('s'),
                ),
        );

    let matches = app.get_matches();

    match matches.subcommand_matches("select") {
        Some(e) => {
            let a = match e.value_of("search") {
                Some(e) => e,
                None => return Err(GoogError::GoogArgNotSupplied),
            };

            let select = Select::new()
                .default(0_usize)
                .item("firefox")
                .item("chrome")
                .interact_on(&Term::stderr())?;
            match select {
                0 => {
                    print_os_info();
                    println!("Searching for\n\"{}\" on Firefox", a);
                    request_search(a.to_owned(), Some(format!("firefox")))?;
                    Ok(())
                }
                1 => {
                    print_os_info();
                    println!("Searching for\n\"{}\" on Chrome", a);
                    request_search(a.to_owned(), Some(format!("google-chrome-stable")))?;
                    Ok(())
                }
                _ => Err(GoogError::GoogNotFound),
            }
        }
        None => Err(GoogError::GoogNotFound),
    };

    match matches.value_of("search") {
        Some(a) => {
            print_os_info();
            println!("Searching for \"{}\" on your defualt browser", a);
            request_search(a.to_string(), None)?;
            Ok(())
        }
        None => Err(GoogError::GoogNotFound),
    }
}
