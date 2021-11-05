//
// mod.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{
  env, 
  path::Path,
  fs::{
    File
  },
  io::{
    self,
    Write,
    prelude::*,
  },
  collections::HashMap
};

use serde_yaml::Value;

use serde::{
  Serialize,
  Deserialize,
};

pub mod project;
pub mod server;
pub mod watch;
pub mod task;

use crate::{console_panic};

use project::Project;
use server::Server;
use watch::{Watch, WatchPattern};
use task::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub project:Project,
  pub server:Option<Server>,
  pub watch:Option<Watch>,
  pub tasks:Option<Vec<Task>>,
  pub env:Option<HashMap<String, Value>>,
}

impl Config {
  pub fn default() -> Self {
    Self {
      project: Project::default(),
      server: None,
      watch: None,
      tasks: None,
      env: None,
    }
  }

  pub fn exists() -> bool {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join(".megfile");
    
    path.exists()
  }

  pub fn write(&self) -> Result<(), io::Error> {
    let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
    let config = serde_yaml::to_string(self).unwrap();
    let file_path = format!("{}/.megfile", cwd);
    let path = Path::new(file_path.as_str());
    
    let mut file = match File::create(&path) {
      Err(why) => console_panic!("Unable to create .megfile : {}", why),
      Ok(file) => file,
    };

    file.write_all(config.as_bytes())
  }

  pub fn read() -> Result<Self, io::Error> {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join(".megfile");

    let mut file = match File::open(&path) {
      Err(why) => console_panic!("Unable to open .megfile : {}", why),
      Ok(f) => f
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
      Err(why) => console_panic!("Unable to read .megfile : {}", why),
      Ok(_) => {}
    };

    buffer = buffer.replace("@", "[_#silent#_]");

    let data:HashMap<String, Value> = serde_yaml::from_str(&buffer).unwrap();
    let project = Self::project_config(&data);
    let server = Self::server_config(&data);
    let watch = Self::watch_config(&data);
    let tasks = Self::task_config(&data);
    let env = Self::env_config(&data);

    Ok(Config{
      project,
      server,
      watch,
      tasks,
      env,
    })
  }

  pub fn load_env_vars(config:&Config) {
    match config.env.clone() {
      Some(vars) => 
        vars
        .iter()
        .for_each(|(k,v)| env::set_var(k, v.as_str().unwrap())),
      None => return
    };
  }

  fn project_config(data:&HashMap<String, Value>) -> Project {
    let default_project = Project::default();

    let name = if data.contains_key("name") { 
      data.get("name").unwrap().as_str().unwrap().to_string()
    } else {
      default_project.name
    };

    let description = if data.contains_key("description") { 
      data.get("description").unwrap().as_str().unwrap().to_string()
    } else {
      default_project.description
    };

    let version = if data.contains_key("version") { 
      data.get("version").unwrap().as_str().unwrap().to_string()
    } else {
      default_project.version
    };


    Project{
      name,
      description,
      version
    }
  }

  fn server_config(data:&HashMap<String, Value>) -> Option<Server> {
    if data.contains_key("server") == false {
      return None;
    }

    let default_server = Server::default();
    let server = data.get("server").unwrap().as_mapping().unwrap();

    let port_key = Value::from("port");
    let port = if server.contains_key(&port_key) { 
      server.get(&port_key).unwrap().as_u64().unwrap()
    } else {
      default_server.port
    };

    let folder_key = Value::from("folder");
    let folder = if server.contains_key(&folder_key) { 
      server.get(&folder_key).unwrap().as_str().unwrap().to_string()
    } else {
      default_server.folder
    };

    Some(Server{
      port,
      folder,
    })
  }

  fn watch_config(data:&HashMap<String, Value>) -> Option<Watch> {
    if data.contains_key("watch") == false {
      return None;
    }

    let watch = data.get("watch").unwrap().as_mapping().unwrap();

    let init_key = Value::from("init");
    let init = watch.get(&init_key).unwrap().as_bool().unwrap_or(false);

    let verbose_key = Value::from("verbose");
    let verbose = watch.get(&verbose_key).unwrap().as_bool().unwrap_or(false);

    let server_key = Value::from("server");
    let server = watch.get(&server_key).unwrap().as_bool().unwrap_or(false);

    const WATCH_RESERVED_KEYS:[&'static str; 3] = ["init", "verbose", "server",];

    let patterns:Vec<WatchPattern> =
      watch.iter()
      .filter(|(k, _v)| false ==  WATCH_RESERVED_KEYS.contains(&k.as_str().unwrap()))      
      .map(|(k, v)| {
        let pattern = k.as_str().unwrap().to_string();
        let commands =
          v.as_sequence()
          .unwrap()
          .iter()
          .map(|v| v.as_str().unwrap().to_string())
          .collect();

        WatchPattern{
          pattern,
          commands
        }
      })
      .collect();

    Some(Watch{
      init,
      verbose,
      server,
      patterns
    })
  }

  fn task_config(data:&HashMap<String, Value>) -> Option<Vec<Task>> {
    const RESERVED_KEYS:[&str; 5] = [
      "name", 
      "description", 
      "watch",
      "server",
      "env",
    ];

    let tasks:Vec<Task> = 
      data
      .into_iter()
      .filter(|(k, _)| {
        let kee = k.clone();
        RESERVED_KEYS.contains(&kee.as_str()) == false
      })
      .map(|(k,v)| {
        let mut args:Vec<String> =
          v
          .as_str()
          .unwrap()
          .to_string()
          .split(" ")
          .map(|a| a.to_string())
          .collect();

        let name = k.clone();
        let mut command = args.remove(0);
        let mut silent = false;

        if command.contains("[_#silent#_]") {
          silent = true;
          command = command.replace("[_#silent#_]", "");
        }

        Task {
          name,
          command,
          args,
          silent
        }
      })
      .collect();

    Some(tasks)
  }

  fn env_config(data:&HashMap<String, Value>) ->  Option<HashMap<String, Value>>{
    if data.contains_key("env") == false {
      return None;
    }

    let env_vars = data.get("env").unwrap().as_mapping().unwrap();

    Some(env_vars
    .into_iter()
    .map(|(k,v)| (k.as_str().unwrap().to_string(), v.clone()) )
    .collect())
  }
}