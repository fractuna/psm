use chrono::prelude::*;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs, result};

// use crate::args::{CfgField, Config};
use crate::args::{get_arg_by_order, ArgAction};
use crate::password::{self};
use crate::util::{self, is_origin_exists, origin_add, remove_origin};
use aes_gcm::aead::Nonce;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use base64::prelude::*;

const NONCE_SIZE_SAMPLE: usize = 12;

fn add_new_password(
    p_name: &str,
    p_desc: &str,
    p_pass: &str,
    key_field: &Vec<u8>,
) -> Result<String, String> {
    // println!("{} - {}", config.new_name.name, config.new_name.value);
    let date: String = format!("2024"); // TODO!()
                                        // Check if there is required things active
    let actual_key = key_field.as_slice();
    // Use unwrap since we check them before
    let base = new_password(p_name, p_pass, &date, p_desc, actual_key);

    if let Err(err) = origin_add(&base) {
        return Err(format!("Add Password: {}", err));
    }

    Ok(format!(
        "Successfully Encrypted your password: {:?}",
        base.value
    ))
}

pub fn generate_random_key() -> Vec<u8> {
    // let mut rng: &[u8; 32] = &[32; 32];
    let asd = Aes256Gcm::generate_key(OsRng);
    let bts = asd.as_slice();
    // println!("{}", bts.len());
    // println!("{:?}", BASE64_STANDARD.encode(asd));
    // let n_key: String = String::from_utf8(asd).unwrap();
    Vec::from(bts)
}

pub fn new_password(
    name: &str,
    password: &str,
    date: &str,
    description: &str,
    key: &[u8],
) -> password::Password {
    // TODO: Find a better way that cloning the value
    let mut base: password::Password =
        password::Password::new(name.to_string(), description.to_string(), date.to_string());
    let gna = Key::<Aes256Gcm>::from_slice(key);
    let g_key = Aes256Gcm::new(gna);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = g_key.encrypt(&nonce, password.as_bytes().as_ref());
    // TODO: Check the ciphertext Result enum
    base.set_value(ciphertext.unwrap(), nonce);
    base.is_enc = true;

    println!(
        "This is the generated password bytes: {:?}",
        password.as_bytes(),
    );
    // println!("This is the Text {:?}", ciphertext);
    // println!("{:?}", ciphertext);
    base
}

// Show the password to the user
pub fn show_password(name: &str, key: &str) -> Result<(), String> {
    // Get the password by its name
    let upr: Result<password::Password, String> = util::origin_show(&name.to_string());
    let key = BASE64_STANDARD.decode(key);
    if let Err(err) = key {
        return Err(format!(
            "[!] Can't parse the key cause {}. Please provide a valid key",
            err,
        ));
    }
    let key = key.unwrap();
    // println!("{}", key.len());

    // Show/print the actual metadata and data
    if let Err(err) = upr {
        return Err(format!("File: {}", err));
    }
    let mut upr: password::Password = upr.unwrap();

    // TODO: Check the unwrap
    let f_pass = BASE64_STANDARD.decode(&upr.value).unwrap();
    let f_pass_res = f_pass.as_slice();

    // println!("This is the read key from args: {:?}", key.as_slice());
    // println!("This is the read password base64 from file: {}", &upr.value);
    // println!("This is the read password from file: {:?}", f_pass_res);

    let nonce_res = &f_pass_res[..NONCE_SIZE_SAMPLE];
    let f_pass_res = &f_pass_res[NONCE_SIZE_SAMPLE..];

    // Descript the data base on key
    let gna = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let g_key = Aes256Gcm::new(gna);
    let nonce = Nonce::<Aes256Gcm>::clone_from_slice(nonce_res);
    // let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = g_key.decrypt(&nonce, f_pass_res.as_ref());
    // Show the result data nad metadata togethera

    if let Err(err) = ciphertext {
        return Err(format!("Encryption: {}", err));
    }
    let result_pass = ciphertext.unwrap();
    upr.is_enc = false;

    println!(
        "name: {}\ndescription: {}\ndate: {}\npassword: {}",
        upr.name,
        upr.description,
        upr.date,
        String::from_utf8(result_pass).unwrap()
    );

    Ok(())
}

// TODO: Wait for this function, cause we change to new args reader
// pub fn print_list_passwords(field: &CfgField) -> Result<(), String> {
//     todo!("Print the list of user passwords's metadata");
// }

// A simple check to see if the user input is AES key or Hash Value
pub fn validate_user_key(key: &str) -> Result<(), String> {
    let mut f_key: String = String::new();
    let hashed_key = format!("{:x}", md5::compute(key));

    // check the legnth of key
    if key.len() != 44 {
        return Err(format!("The key is not valid!"));
    }

    if let Ok(mut v) = fs::File::open("./.pass/meta") {
        if let Err(_) = v.read_to_string(&mut f_key) {
            return Err(format!(
                "There is a problem in origin structure, please do the `psm --init` again"
            ));
        }
    }

    if (hashed_key != f_key) {
        return Err(format!(
            "Your key is not same as what you made before. please give the original key"
        ));
    }

    return Ok(());
}

fn check_deps(config: &HashMap<&'static str, ArgAction>, deps: Vec<&'static str>) -> bool {
    for i in deps {
        if config.get(i).unwrap().isActive() == false {
            return false;
        }
    }
    return true;
}

fn check_deps_partial(arg: &str, deps: Vec<&'static str>) -> bool {
    let sucsess: bool = false;
    for i in deps {
        if i == arg {
            return true;
        }
    }
    false
}

// TODO needs to be cheked
pub fn remove_origin_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    let remove_obj = config.get("remove").unwrap();
    match remove_obj.get_value().as_str() {
        "all" => {
            if !util::remove_origin() {
                return Err(format!("There was a problem when removing your origin"));
            }
            return Ok(format!("Successfully removed the origin environment"));
        }
        &_ => {
            let pass_obj = config.get("name").unwrap().call(config);

            if let Err(_) = pass_obj {
                return Err(format!("There is a problem when running name argument"));
            }

            if !util::remove_password(pass_obj.unwrap().as_str()) {
                return Err(format!("There is a problem when removing your password"));
            }

            return Ok(format!(
                "Successfully removed your {} password",
                remove_obj.get_value()
            ));
        }
    };
}

pub fn password_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    return Ok(config.get("password").unwrap().get_value());
}

pub fn create_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    // First check if origin is initilized
    if (!util::is_origin_exists()) {
        return Err(format!(
            "Please first initialize the program with 'psm init'"
        ));
    }

    let deps = vec!["name", "description", "key"];
    if !check_deps(config, deps) {
        return Err(format!("Can't process because of the deps!"));
    }

    let name = config.get("name").unwrap().call(config).unwrap();
    let pass_value = config.get("password").unwrap().call(config).unwrap();
    let description = config.get("description").unwrap().get_value();
    let key = config.get("key").unwrap().get_value();

    println!("Creating password with this information:\nName: {}\nPassword: {}\ndescription: {}\nkey: ****", name, pass_value, description);

    // Check the key validation
    if let Err(err) = validate_user_key(&key) {
        return Err(format!(
            "Can't validate your key, because {}",
            err.to_string()
        ));
    }

    let cur_date = Utc::now();
    let mut base: password::Password =
        password::Password::new(name.clone(), description.clone(), cur_date.to_string());

    let m_key = BASE64_STANDARD.decode(&key);
    if let Err(err) = m_key {
        return Err(format!("Can't read the key, cause {}", err.to_string()));
    }

    let m_key = m_key.unwrap();

    if let Err(err) = add_new_password(
        name.as_str(),
        description.as_str(),
        pass_value.as_str(),
        &m_key,
    ) {
        return Err(format!(
            "Can't create new password cause: {}",
            err.to_string()
        ));
    }

    Ok(format!("your password has been sucsessfully created"))
}

pub fn get_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    let deps = vec!["key", "name"];
    if (!check_deps(config, deps)) {
        return Err(format!("Please provid valid arguments"));
    }

    let key_obj = config.get("key").unwrap();
    let name_obj = config.get("name").unwrap();

    if let Err(err) = validate_user_key(key_obj.get_value().as_str()) {
        return Err(format!("Key: {err}"));
    }

    if let Err(err) = show_password(name_obj.get_value().as_str(), key_obj.get_value().as_str()) {
        return Err(format!("Print: {}", err));
    }

    Ok(format!(""))
}

// A function to create/initialize the origin environment
pub fn init_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    // No deps for this function

    // First, check if there is already a origin dir
    if util::is_origin_exists() {
        return Err(format!("You already have the origin, please remove the old one if you want to use this command\nNotice: you can remove the origin by using 'psm remove all'"));
    }

    // Second: Try to create the origin dir
    if let Err(err) = fs::create_dir("./.pass") {
        return Err(format!("Can't make the origin folder"));
    }

    // Next: try to make a new key and base64 it
    // TODO: Make a single function to generate key
    let n_key = generate_random_key();
    let m_key = BASE64_STANDARD.encode(n_key);

    // Next: try to make a meta file to save metadata about origin
    let origin_meta = fs::File::create("./.pass/meta");
    if let Ok(mut meta_file) = origin_meta {
        if let Err(_) = meta_file.write_all(util::get_hash(&m_key).as_bytes()) {
            return Err(format!("Can't add data to the origin's metadata file"));
        }
    } else {
        return Err(format!("Can't create the metadata file for origin"));
    }

    // Print out the key and succsess message
    println!();
    return Ok(format!(
        "Sucsessfully generated the origin :D\nThis is your new key: {}\n",
        String::from(m_key),
    ));
}

pub fn process_args(
    config: &HashMap<&'static str, ArgAction>,
    master_key: &str,
) -> Result<String, String> {
    // println!("This is the master key: {}", master_key);
    // Call the proporiet function based on the master_key (main arg)
    // let g_obj = config.get(master_key);
    println!("{:?}", config);
    for i in config {
        if i.1.get_priority() >= 1 {
            let g_obj = i.1;
            // if let None = g_obj {
            //     return Err(format!("Please enter a valid argument..."));
            // }
            // let g_obj = g_obj.unwrap();
            // if g_obj.get_priority() <= 0 {
            //     return Err(format!("Please pass the correct arguments"));
            // }
            let result = g_obj.call(&config);
            if let Err(error) = result {
                return Err(error);
            }
            // let result: String = match *name {
            // println!("{}", result.unwrap());
            return Ok(result.unwrap());
        }
    }
    return Err(format!("Please provide a valid argument"));
    /*
        if  {
            if let Err(err) = fs::create_dir("./pass") {
                return Err(format!("Can't make the origin folder"));
            }
            // TODO: Make a single function to generate key
            let n_key = generate_random_key();
            let m_key = BASE64_STANDARD.encode(n_key);

            let origin_meta = fs::File::create("./pass/meta");
            if let Ok(mut meta_file) = origin_meta {
                if let Err(_) = meta_file.write_all(util::get_hash(&m_key).as_bytes()) {
                    return Err(format!("Can't add data to the origin's metadata file"));
                }
            } else {
                return Err(format!("Can't create the metadata file for origin"));
            }
            println!();
            return Ok(format!(
                "Sucsessfully generated the origin :D\nThis is your new key: {}\n",
                String::from(m_key),
            ));
        }

        if config.list_passwords.IsExists() {
            if let Err(err) = util::list_origin() {
                return Err(format!("list origin{}", err));
            }
            return Ok(String::default());
        }

        // TODO: Make a better process if statement
        if let Some(_) = config.IsNewPassword() {
            // Check for existsing of origin

            let new_pass = config.IsNewPassword();
            let new_desc = config.IsNewDescription(); // Make a better output (return)
            let l_new_name: Option<&CfgField> = config.IsNewName();
            let l_generate_pass: Option<&CfgField> = config.IsGeneratePassword();
            let l_key: Option<&CfgField> = config.IsKeyExists();

            if let None = l_new_name {
                return Err(String::from("Please Provide a name for your password"));
            }

            let mut n_key: Vec<u8> = Vec::default();
            let mut is_new_key: bool = false;
            let actual_key: &[u8] = &[0; 30];
            let m_key: Vec<u8>;

            // Check the password validity
            if let Err(err) = validate_user_key(&l_key.unwrap().value) {
                return Err(format!("Key: {err}"));
            }

            if let None = l_key {
                if let None = l_generate_pass {
                    return Err(String::from(
                        "Please Provide your main key for encryption the password",
                    ));
                } else {
                    if (util::is_origin_exists()) {
                        return Err(String::from("You already made the base key.\nPlease use the same key or make another with `psm -g`"));
                    }
                    n_key = generate_random_key();
                    is_new_key = true;
                    m_key = BASE64_STANDARD.decode(&n_key).unwrap();
                }
            } else {
                m_key = BASE64_STANDARD.decode(&l_key.unwrap().value).unwrap();
                // println!(
                //     "This is the ready to use key for make new password {:?}",
                //     actual_key
                // );
            }

            if let Err(err) = add_new_password(
                new_pass.unwrap(),
                new_desc,
                l_new_name.unwrap(),
                m_key,
                is_new_key,
            ) {
                // Ok(_) => {
                //     println!("+ Your new password added to the system");
                // }
                return Err(err);
            }
            return Ok(format!("Succsessfully update your password list"));
        } else if let Some(p_name) = config.IsNewName() {
            // Check for existsing of origin
            if !is_origin_exists() {
                return Err(String::from(
                    "Can't find the origin path. Please make one with `psm --init or psm -i`",
                ));
            }
            let key = config.IsKeyExists();
            if let None = key {
                return Err(String::from("Please provide the key"));
            }
            let key = key.unwrap();

            if let Err(err) = validate_user_key(&key.value) {
                return Err(format!("Key: {err}"));
            }

            if let Err(err) = show_password(&p_name.value, &key.value) {
                return Err(format!("Print: {}", err));
            }
            return Ok(String::default());
        } else if let Some(list_pass) = config.IsListPassword() {
            // Check for existsing of origin
            if !is_origin_exists() {
                return Err(String::from(
                    "Can't find the origin path. Please make one with `psm --init or psm -i`",
                ));
            }
            if let Err(err) = print_list_passwords(list_pass) {
                return Err(format!("Can't show the list of passwords cause: {}", err));
            }
            return Ok(String::default());
        } else if let Some(_) = config.IsRessetOrigin() {
            // Check for existsing of origin
            if !is_origin_exists() {
                return Err(String::from(
                    "Can't find the origin path. Please make one with `psm --init or psm -i`",
                ));
            }
            // TODO: remove origin recursive
            return Ok(String::from("[+] Succsessfully resset the origin"));
        } else {
            return Err(String::from("Please provide a option"));
        }
    */
}
