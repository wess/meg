//
// trigger.rs
// Magpie
// 
// Author: Wess Cope (me@wess.io)
// Created: 05/27/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{
  io::{
    Write,
    stdout,
    stderr,
  },
  process::Command
};

use crate::{config::{Config, task::Task}, console_panic};


#[derive(Debug, Clone, PartialEq)]
pub struct Trigger {
  pub command:String,
}


impl Trigger {
  pub fn new(command:String) -> Self {
    Self{command}
  }

  pub fn exec(&self) {
    let config = Config::read().unwrap();
    let actions:Vec<Task> = 
      config.tasks
      .unwrap()
      .into_iter()
      .filter(|a| a.name.to_lowercase() == self.command.to_lowercase())
      .collect();

    if actions.len() == 0 {
      console_panic!("No action '{}' found.", self.command);
    }

    for action in actions {
      let command = action.command.clone();

      let result = Command::new(command)
      .args(action.args)
      .output()
      .expect("failed to execute command");
      
      
      if false == action.silent {
        stdout().write_all(&result.stdout).unwrap();
        stderr().write_all(&result.stderr).unwrap();
      }
      
    } 
  }
}