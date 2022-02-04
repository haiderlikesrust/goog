use std::{io::Write, fs};
use directories::{ ProjectDirs };

pub fn gog_init() -> Result<(), super::errors::GogError> {

    if let Some(p) = ProjectDirs::from("", "", "gog") {
        if std::path::Path::new(p.config_dir()).exists() {
            println!("gog config dir already exists");
            return Ok(())
        }
        println!("Setting gog up");
       fs::create_dir(p.config_dir())?;
       if let  Some(f) = ProjectDirs::from("", "", "gog/config.json") {
           let mut file = fs::File::create(f.config_dir())?;
           let config_file = r#"
                {
                    "api_key": "YOUR_API_KEY"
                }
           "#;
           file.write(config_file.trim().as_bytes())?;
       }
    }
    
    println!("Done setting up gog ✅");
    Ok(())
}
