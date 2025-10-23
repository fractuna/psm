use crate::password;
use crate::password::Password;
use md5;
use std::env;
use std::fs::{create_dir, read, read_dir, remove_dir_all, File};
use std::io::Write;
use std::io::{stdin, Read};
use std::path::Path;

// Cause we can't use global variables
pub fn ORIGIN_PATH() -> String {
    return format!("{}/.psm", env::home_dir().unwrap().display());
}

pub fn banner(version: &'static str) {
    println!(
        "psm password manager version {}
Usage: psm [OPTIONS...] \n
  create - to create a new password
  name - set name for your password
  password - set the password
  description - set description of your password
  get - get the password
  info - more info about how it works
  key - set key for password\n",
        version,
    )
}

// PSM Passwords Folder Struction:

//  .pass folder to store all the passwords that each password has
//  its own folder with the data and meta files.
//  For example:

//  ===================================
//  .pass/
//  |- instagram/
//  |  |- data - data file that stores things like password and etc.
//  |  |- meta - metadata file that stores the ascii base meta data.
//  |- TikTok/
//  |  |- data - ~
//  |  |- meta - ~
//  |- Spacehey/
//  |  |- data - ~
//  |  |- meta - ~
// ====================================

// The data file is the main file that stores the encrypted password
// And if there is any metadata file, the needed data will be store
// on the same file. The data file will store password and metadata
// by order.

// The metadata file is the information file that use to store the
// extra information about the password, like description and date.
// Notice that the name of the password will be the same as the name
// of the parent folder. with that said we don't need to store the name
// of the password in the meta file anymore.

pub fn get_hash(key: &str) -> String {
    // Do the hash logic
    format!("{:x}", md5::compute(key))
}

// Add password to the origin
pub fn origin_add(password: &password::Password) -> Result<(), String> {
    if !is_data_exists(&password.name) {
        if let Err(_) = create_dir(format!("{}/{}", &ORIGIN_PATH(), &password.name)) {
            return Err(format!("[!] Can't find the password folder"));
        }
        // Make the meta file for saving metadata about the origin
    } else {
        // TODO: Ask the user to agree the existence of password
        // let answer = Ask(format!(
        //     "You are overriting the current content of {} password, are you ok? [Y,n]? ",
        //     &password.name
        // )
        // .as_str())?;
        //
        // if !answer {
        //     return Err(format!("Stop process by user input"));
        // }
        Info("Notice, existed password updated");
    }

    // TODO: Check this out
    let data: String = format!("{}", password.value.clone());
    let meta: String = format!("{}\n{}", password.description, password.date);

    let file_data = File::create(format!("{}/{}/data", &ORIGIN_PATH(), &password.name));
    if let Err(_) = file_data {
        return Err(format!("Can't make data file_data"));
    }

    let res = file_data.unwrap().write_all(data.as_bytes());
    if let Err(_) = res {
        return Err(format!("Can't write to the file_data"));
    }

    let file_meta = File::create(format!("{}/{}/meta", &ORIGIN_PATH(), &password.name));
    if let Err(_) = file_meta {
        return Err(format!("Can't make meta)) file"));
    }

    let res = file_meta.unwrap().write_all(meta.as_bytes());
    if let Err(_) = res {
        return Err(format!("Can't write to the file meta"));
    }

    Ok(())
}

pub fn create_origin() -> Result<(), String> {
    let result = create_dir(&ORIGIN_PATH());
    if let Err(err) = result {
        return Err(format!("Can't make the origin folder cause: {}", err));
    }
    return Ok(());
}

// Show password from origin
pub fn origin_show(name: &String) -> Result<password::Password, String> {
    let mut u_pass = password::Password::default();

    // Check for existing of password
    if !is_data_exists(name) {
        // TODO: Ask the user if wants to make a new password with the same name
        return Err(format!("Can't find any password with that name."));
    }

    // Read the data and meta files and Check for data/meta readabliti
    let data = read(format!("{}/{}/data", &ORIGIN_PATH(), name));
    if let Err(err) = data {
        return Err(err.to_string());
    }

    // Read the meta file
    let meta = read(format!("{}/{}/meta", &ORIGIN_PATH(), name));
    if let Err(err) = meta {
        return Err(err.to_string());
    }

    // Shadowing the data and meta
    let data = String::from_utf8(data.unwrap());
    let meta = String::from_utf8(meta.unwrap());

    // TODO: Better warning handeling
    if let Err(err) = data {
        return Err(err.to_string());
    } else if let Err(err1) = meta {
        return Err(err1.to_string());
    }

    let data = data.unwrap();
    let meta = meta.unwrap();

    // Parse them into u_pass

    let l_data = data.split("\n").collect::<Vec<&str>>();
    let d_pass = l_data[0]; // First line name

    let l_meta: Vec<&str> = meta.split("\n").collect();
    let description = l_meta[0];
    let date = l_meta[1];

    u_pass.name = String::from(name);
    u_pass.description = String::from(description);
    u_pass.value = String::from(d_pass);
    u_pass.date = String::from(date);

    // return the pass obj
    Ok(u_pass)
}

pub fn list_origin() -> Result<Vec<Password>, String> {
    let list_res: Vec<Password> = Vec::new();
    let mut counter = 0;

    // get the files from origin
    if let Ok(v) = read_dir(&ORIGIN_PATH()) {
        println!("This is the saved passwords list: \n");
        for items in v {
            if let Err(_) = items {
                return Err(format!("Error when reading origin passwords"));
            }
            let item = items.unwrap();
            // Read only the folders
            if item.path().is_dir() {
                println!("{}. {:?}", counter + 1, item.file_name());
            }
            // TODO: Show more information about the password
            counter += 1;
        }
    } else {
        return Err(format!("Can't read from origin"));
    }

    Ok(list_res)
}

pub fn Ask(text: &str) -> Result<bool, String> {
    // TODO: The print is not working. idk...
    println!("[?] {}", text);
    let mut input: String = String::new();
    let res = stdin().read_line(&mut input).ok();
    match res {
        Some(_) => {
            // TODO: Better way of removing new line
            if &input.to_lowercase().as_str()[0..input.len() - 2] == "y" {
                return Ok(true);
            }
            return Ok(false);
        }
        None => return Err(format!("Can't read the user input")),
    }
}

pub fn AskStr(text: &str) -> Result<String, String> {
    // TODO: The print is not working. idk...
    println!("[?] {}", text);
    let mut input: String = String::new();
    let res = stdin().read_line(&mut input).ok();
    input = String::from(&input.as_str()[0..input.len() - 2]);
    match res {
        Some(_) => {
            // TODO: Better way of removing new line
            return Ok(input);
        }
        None => return Err(format!("Can't read the user input")),
    }
}

pub fn remove_password(name: &str) -> bool {
    if !is_data_exists(name) {
        return false;
    }

    if let Err(_) = remove_dir_all(format!("{}/{}", &ORIGIN_PATH(), name)) {
        return false;
    }

    true
}

pub fn get_origin_metadata() -> Result<String, &'static str> {
    let mut f_key: String = String::new();
    if let Ok(mut v) = File::open(format!("{}/meta", &ORIGIN_PATH())) {
        if let Err(_) = v.read_to_string(&mut f_key) {
            return Err("There is a problem in origin structure, please do the `psm --init` again");
        }
    } // TODO: Else
    return Ok(f_key);
}

// TODO: Create a new type for the return type
pub fn create_origin_metedata(m_key: &str) -> Result<&'static str, &'static str> {
    // Next: try to make a meta file to save metadata about origin
    let origin_meta = File::create(format!("{}/meta", &ORIGIN_PATH()));
    if let Ok(mut meta_file) = origin_meta {
        if let Err(_) = meta_file.write_all(get_hash(&m_key).as_bytes()) {
            return Err("Can't add data to the origin's metadata file");
        }
    } else {
        return Err("Can't create the metadata file for origin");
    }
    Ok("Meta file created for your origin")
}

pub fn is_data_exists(name: &str) -> bool {
    if Path::new(format!("{}/{}", &ORIGIN_PATH(), name).as_str()).exists() {
        return true;
    }
    return false;
}

pub fn remove_origin() -> bool {
    if !is_origin_exists() {
        return false;
    }
    if let Err(_) = remove_dir_all(&ORIGIN_PATH()) {
        return false;
    }

    return true;
}

pub fn Info(text: &str) {
    println!("[INFO] {text}");
}

pub fn Warn(text: &str) {
    println!("[WARN] {text}");
}

pub fn Error(text: &str) {
    println!("\n[ERROR] {text}");
}

// Check is there is any global .pass folder
pub fn is_origin_exists() -> bool {
    if Path::new(&ORIGIN_PATH()).exists() {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample: String = String::from("This is the text\nThis is the new line");
        let l_data = sample.split("\n").collect::<Vec<&str>>();
        // println!("This is the first line {}", l_data[0]);
    }
}
