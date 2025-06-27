use std::collections::HashMap;

use crate::core::{create_callback, init_callback, remove_origin_callback};

type ArgCallback = fn(&HashMap<&'static str, ArgAction>) -> Result<String, String>;

#[derive(Debug, Clone)]
pub struct ArgAction {
    key: &'static str,
    value: String,
    only_key: bool,
    used: bool,
    callback: ArgCallback,
    description: &'static str,
    order: usize,
}

impl ArgAction {
    fn new(
        key: &'static str,
        callback: ArgCallback,
        only_key: bool,
        description: &'static str,
    ) -> (&'static str, ArgAction) {
        (
            key,
            ArgAction {
                key,
                value: format!("{}", "TMP"),
                callback,
                used: false,
                only_key,
                description,
                order: 0,
            },
        )
    }

    pub fn isActive(&self) -> bool {
        self.used
    }

    fn active(&mut self) {
        self.used = true;
    }

    pub fn get_key(&self) -> &str {
        self.key
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn set_order(&mut self, value: usize) {
        self.order = value;
    }

    pub fn get_order(&self) -> usize {
        return self.order;
    }

    // TODO: Use custome callback instead of static method
    pub fn validate_value(&self, deps: Vec<&'static str>) -> Result<(), String> {
        if (!deps.contains(&self.get_value().as_str())) {
            return Err(format!("Can't validate the command mode!"));
        }
        Ok(())
    }

    pub fn get_desc(&self) -> &'static str {
        return self.description;
    }

    pub fn get_value(&self) -> String {
        format!("{}", self.value)
    }

    pub fn call(&self, ref_action: &HashMap<&'static str, Self>) -> Result<String, String> {
        (self.callback)(ref_action)
    }
}

pub fn get_arg_by_order(p_args: &HashMap<&'static str, ArgAction>, order: usize) -> Option<String> {
    for (name, value) in p_args {
        if value.get_order() == order {
            return Some((*name).to_string());
        }
    }
    return None;
}

pub fn createDoc(p_args: &HashMap<&'static str, ArgAction>) -> String {
    let mut output: String = String::new();

    for (name, value) in p_args {
        let name = *name;
        let desc = value.get_desc();
        output = format!("{output}\n{name}\t\t{desc}");
    }

    output
}

fn version_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    Ok(format!("{}", crate::VERSION))
}

fn get_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    Ok(format!("3.0.1"))
}

fn name_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    Ok(config.get("name").unwrap().get_value())
}

fn description_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    Ok(config.get("description").unwrap().get_value())
}

fn key_callback(config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
    Ok(config.get("key").unwrap().get_value())
}

pub fn argument_parser(
    args: Vec<String>,
) -> Result<(HashMap<&'static str, ArgAction>, String), String> {
    let list_of_keys: Vec<(&'static str, ArgAction)> = vec![
        ArgAction::new(
            "init",
            init_callback,
            true,
            "To initilize the password environment",
        ),
        ArgAction::new(
            "version",
            version_callback,
            true,
            "print the program's version",
        ),
        ArgAction::new("get", get_callback, true, "get the password by name"),
        ArgAction::new(
            "remove",
            remove_origin_callback,
            false,
            "remove data [name, all, date]",
        ),
        ArgAction::new(
            "name",
            name_callback,
            false,
            "set a name for password [text]",
        ),
        ArgAction::new("create", create_callback, true, "create a new password"),
        ArgAction::new(
            "description",
            description_callback,
            false,
            "set description for password [text]",
        ),
        ArgAction::new(
            "key",
            key_callback,
            false,
            "set key to encrypt the passwords [text]",
        ),
    ];

    let mut keys: HashMap<&'static str, ArgAction> = HashMap::from_iter(list_of_keys);
    let mut mode: bool = true;
    let mut arg_map: HashMap<String, String> = HashMap::new();
    let mut key_tmp: String = String::new();
    let mut args_l = args;
    args_l.remove(0); // the program path
    let master_arg = args_l[0].to_string();
    let mut order_counter: usize = 0;
    for v in args_l {
        if mode == true {
            let key_obj: Option<&mut ArgAction> = keys.get_mut(v.as_str());
            if let Some(x) = key_obj {
                arg_map.insert(String::from(x.get_key()), format!("TMP"));
                // x.set_value(v.clone());
                order_counter += 1;
                x.set_order(order_counter);
                x.active();
                if x.only_key == false {
                    mode = false;
                }
                key_tmp = v;
            }
        } else if mode == false {
            let key_obj: Option<&mut ArgAction> = keys.get_mut(key_tmp.as_str());
            if let Some(x) = key_obj {
                // arg_map.insert(String::from(x.get_key()), format!("TMP"));
                if x.isActive() {
                    x.set_value(v.clone());
                }
                // println!("{:?}", x);
            }
            // arg_map.insert(key_tmp.clone(), v);
            key_tmp = String::default();
            mode = true
        }
    }
    println!("{:?}", keys);
    Ok((keys, master_arg))
}
