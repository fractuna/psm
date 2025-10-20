use std::{cell::RefCell, collections::HashMap};

use crate::{
    core::{
        create_callback, get_callback, init_callback, password_callback, remove_origin_callback,
    },
    VERSION,
};

type ArgCallback = fn(&HashMap<&'static str, ArgAction>) -> Result<String, String>;

#[derive(Debug, Clone)]
pub struct ArgAction {
    key: &'static str,
    value: String,
    only_key: bool,
    used: bool,
    callback: Option<ArgCallback>,
    next: String,
    description: &'static str,
    order: usize,
    priority: usize,
}

impl ArgAction {
    fn new(
        key: &'static str,
        callback: Option<ArgCallback>,
        only_key: bool,
        description: &'static str,
        priority: usize,
    ) -> (&'static str, ArgAction) {
        (
            key,
            ArgAction {
                key,
                value: format!("{}", "TMP"),
                callback,
                used: false,
                only_key,
                next: format!(""),
                description,
                order: 0,
                priority,
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

    pub fn call_next(&self, config: &HashMap<&'static str, ArgAction>) -> Result<String, String> {
        config.get(self.get_next()).unwrap().call(config)
    }

    pub fn get_priority(&self) -> usize {
        self.priority
    }

    pub fn set_next(&mut self, next: String) {
        self.next = next;
    }

    pub fn get_next(&self) -> &str {
        self.next.as_str()
    }

    pub fn set_priority(&mut self, value: usize) {
        self.priority = value;
    }

    pub fn set_order(&mut self, value: usize) {
        self.order = value;
    }

    pub fn get_order(&self) -> usize {
        return self.order;
    }

    // pub fn addDep(&mut self, dep: &ArgAction) -> Result<(), &'static str> {
    //     let inner = self.deps.borrow_mut();
    //     inner.insert(dep.get_key(), dep.clone());
    //     Ok(())
    // }

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
        if let Some(x) = self.callback.as_ref() {
            return (x)(ref_action);
        }
        return Err(format!("There is not any callback like this"));
        // (self.callback)(ref_action)
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
    // if let Ok(x) = config.get("version").unwrap().call_next(config) {
    //     return Ok(format!("This is the result: {}", x));
    // }
    Ok(format!("Version {}", VERSION))
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
            Some(init_callback),
            true,
            "To initilize the password environment",
            1,
        ),
        ArgAction::new(
            "get",
            Some(get_callback),
            false,
            "To get the password information",
            1,
        ),
        ArgAction::new(
            "version",
            Some(version_callback),
            true,
            "print the program's version",
            1,
        ),
        ArgAction::new(
            "get",
            Some(get_callback),
            true,
            "get the password by name",
            1,
        ),
        ArgAction::new(
            "password",
            Some(password_callback),
            false,
            "set the password",
            0,
        ),
        ArgAction::new(
            "remove",
            Some(remove_origin_callback),
            false,
            "remove data [name, all, date]",
            1,
        ),
        ArgAction::new(
            "name",
            Some(name_callback),
            false,
            "set a name for password [text]",
            0,
        ),
        ArgAction::new(
            "create",
            Some(create_callback),
            true,
            "create a new password",
            1,
        ),
        ArgAction::new(
            "description",
            Some(description_callback),
            false,
            "set description for password [text]",
            0,
        ),
        ArgAction::new(
            "key",
            Some(key_callback),
            false,
            "set key to encrypt the passwords [text]",
            0,
        ),
        ArgAction::new("all", None, true, "select all", 0),
    ];

    let mut keys: RefCell<HashMap<&'static str, ArgAction>> =
        RefCell::new(HashMap::from_iter(list_of_keys));

    let mut mode: bool = true;
    let mut arg_map: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    let mut key_tmp: String = String::new();
    let mut args_l = args;
    args_l.remove(0); // the program path
    let master_arg = args_l[0].to_string();
    let mut order_counter: usize = 0;
    for v in args_l {
        if mode == true {
            let mut b_keys = keys.borrow_mut();
            let key_obj: Option<&mut ArgAction> = b_keys.get_mut(v.as_str());
            if let Some(x) = key_obj {
                // let arg_map_shared = arg_map.get_mut();
                // arg_map_shared.insert(String::from(x.get_key()), format!("TMP"));
                // x.set_value(v.clone());
                order_counter += 1;
                x.set_order(order_counter);
                x.active();
                if x.only_key == false {
                    mode = false;
                }

                if key_tmp != "" {
                    b_keys
                        .get_mut(key_tmp.as_str())
                        .unwrap()
                        .set_next(v.clone());
                }

                key_tmp = v;
            } else {
                return Err(format!("Please provide valid arguents"));
            }
        } else if mode == false {
            let mut b_keys = keys.borrow_mut();
            let key_obj: Option<&mut ArgAction> = b_keys.get_mut(key_tmp.as_str());
            if let Some(x) = key_obj {
                // If 'V' itself is a argument passed by keys
                // Then we need to store the v object inside deps of the
                // wanted key

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

    let keys_to_remove: Vec<&str> = {
        let map_ref = keys.borrow();
        map_ref
            .iter()
            .filter(|(_, obj)| !obj.isActive())
            .map(|(k, _)| *k)
            .collect()
    };
    {
        let mut map_mut = keys.borrow_mut();
        for key in keys_to_remove {
            map_mut.remove(&key);
        }
    }

    Ok((keys.into_inner(), master_arg))
}
