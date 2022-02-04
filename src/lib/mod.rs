pub mod errors;
pub mod commands;
pub mod models;
use clap;
use errors::GogError;
use serde::Deserialize;

use std::fs;

#[derive(Deserialize)]
struct Config {
    api_key: String
}



fn read_config() -> Result<Config, crate::errors::GogError> {
    if let  Some(f) = directories::ProjectDirs::from("", "", "gog/config.json") {
        let config = fs::read_to_string(f.config_dir())?;
        Ok(serde_json::from_str::<Config>(config.as_str())?)
    } else {
        Err(GogError::GogInitError)
    }
    

}


pub fn request_search(query: String) -> Vec<crate::models::Root> {
    type Output =  Vec<crate::models::Root>;
    let query = format!("
        https://www.googleapis.com/customsearch/v1?key={}&cx=017576662512468239146:omuauf_lfve&q={}
    ", read_config().expect("Error reading config file").api_key, query);
    let search = reqwest::blocking::get(&query)
        .expect("Error while getting results")
        .json::<Output>()
        .expect("Error parsing");
    
    search
}

pub fn run() -> Result<(), GogError> {
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
            clap::App::new("init")
            .about("Creates useful files and folders needed")
            .short_flag('i')
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(_) => Ok(crate::commands::gog_init()?),
        None => Err(GogError::GogNotFound),
    };

    match matches.value_of("search") {
        Some(e) => {
            let s = request_search(e.to_string());
            s.iter()
                .for_each(|a| {
                    a.items
                        .iter()
                        .for_each(|item| {
                            println!("
                            Title: {}\n
                            Link: {}\n
                            Description: {}\n
                            ", item.title, item.link, item.snippet)
                        })
                        
                });
            Ok(())
        },
        None => Err(GogError::GogNotFound)
    }

    

}


