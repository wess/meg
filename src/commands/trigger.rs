//
// trigger.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//


use std::{
  process::Command,
  io::{
    Write,
    stdout,
    stderr
  }
};

use crate::{config::{Config, task::Task}, console_panic};

#[derive(Debug, Clone)]
pub struct Trigger {}


impl Trigger {
  pub fn run(task:String) {
    let config = Config::read().unwrap();
    let tasks = match config.tasks {
      Some(t) => t,
      None => console_panic!("No task {} was found.", task)
    };

    let calling:Vec<Task> = 
    tasks
    .into_iter()
    .filter(|t| t.name == task)
    .collect();

    if calling.len() > 0 {
      let callee = calling.first().unwrap();

      let command = callee.command.clone();
      let args = callee.args.clone();
      let silent = callee.silent.clone();

      let result =
        Command::new(command.clone())
        .args(args)
        .output()
        .expect(format!("Failed to run {}", command).as_str());
        

      if false == silent {
        stdout().write_all(&result.stdout).unwrap();
        stderr().write_all(&result.stderr).unwrap();
      }
    }
  }
}
