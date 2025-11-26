use std::{env, fs, path::Path};

fn main() {
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let maven_setting_path = Path::new(&home_dir).join(".m2/settings.xml");
    let maven_setting_path_tmp = Path::new(&home_dir).join(".m2/settings.xml.tmp");
    let setting_exists = match fs::exists(&maven_setting_path) {
        Ok(exists) => exists,
        Err(err) => {
            eprintln!("check file {maven_setting_path:?} existence failed: {err}");
            std::process::exit(1);
        }
    };
    let setting_tmp_exists = match fs::exists(&maven_setting_path_tmp) {
        Ok(exists) => exists,
        Err(err) => {
            eprintln!("check file {maven_setting_path:?} existence failed: {err}");
            std::process::exit(1);
        }
    };

    // Terminate program if both files exist.
    if setting_exists && setting_tmp_exists {
        eprintln!(
            "rename file failed: both {maven_setting_path:?} file and {maven_setting_path_tmp:?} exists."
        );
        std::process::exit(1);
    }

    // Move settings.xml to settings.xml.tmp
    if setting_exists {
        if let Err(err) = fs::rename(&maven_setting_path, &maven_setting_path_tmp) {
            eprintln!("rename file {maven_setting_path:?} failed: {err}");
            std::process::exit(1);
        }

        println!("OFF");
        return;
    }

    // Move settings.xml.tmp to settings.xml
    if setting_tmp_exists {
        if let Err(err) = fs::rename(&maven_setting_path_tmp, &maven_setting_path) {
            eprintln!("rename file {maven_setting_path_tmp:?} failed: {err}");
            std::process::exit(1);
        }

        println!("ON");
        return;
    }
}
