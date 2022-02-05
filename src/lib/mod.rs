pub mod errors;

use clap;
use errors::*;
use dialoguer::{Select};
use console::Term;



pub fn request_search(query: String, with: Option<String>) -> Result<(), GoogError> {
    let query = format!("
    https://www.google.com/search?q={}
    ", query);
    match with {
        Some(e) => Ok(open::with(&query, &e)?),
        None => Ok(open::that(&query)?)
    }
}

pub fn run() -> Result<(), GoogError> {
    let app = clap::App::new("gog")
        .setting(clap::AppSettings::ArgRequiredElseHelp)

        .about("Run Google in your terminal!")
        .arg(
            clap::Arg::new("search")
            .takes_value(true)
            .value_name("QUERY")
            .short('s')
        
        )
        .subcommand(
            clap::App::new("select")
            .about("Select your browser and the command will automatically run.")
            .alias("s")
            .arg(
                clap::Arg::new("search")
                .takes_value(true)
                .value_name("QUERY")
                .short('s')
            )
        );

    let matches = app.get_matches();

    match matches.subcommand_matches("select") {
        Some(e) => {
            let a = e.value_of("search").expect("You missed a required arg.");
            let select = Select::new()
                .default(0_usize)
                .item("firefox")
                .item("chrome")
                .interact_on(&Term::stderr())?;
            match select {
                0 => {
                    request_search(a.to_owned(), Some(format!("firefox")))?;
                    println!("Searching for\n {} on FIREFOX" , a);
                    Ok(())
                },
                1 => {
                    request_search(a.to_owned(), Some(format!("google-chrome-stable")))?;
                    println!("Searching for\n {} on CHROME" , a);
                    Ok(())
                },
                _ => Err(GoogError::GoogNotFound)
            }
        },
        None => Err(GoogError::GoogNotFound)
    };
    

    match matches.value_of("search") {
        Some(a) => {
            request_search(a.to_string(), None)?;
            println!("Searching for\n {} on CHROME" , a);
            Ok(())
        },
        None => Err(GoogError::GoogNotFound)
    }

}


