//
// project.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::env;

use serde::{
  Serialize,
  Deserialize,
};

use crate::string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
  pub name:String,
  pub description:String,
  pub version:String,
}

impl Project {
  pub fn default() -> Self {
    let name = 
      env::current_dir()
      .unwrap()
      .display()
      .to_string()
      .split("/")
      .collect::<Vec<&str>>()
      .pop()
      .unwrap_or("Shaddup Meg")
      .to_lowercase();
  
    let description = format!("{} project", name.clone());

    Self {
      name,
      description,
      version: string!("0.0.1")
    }
  }
}