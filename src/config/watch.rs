//
// watch.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/10/2021
// 
// Copywrite (c) 2021 Wess.io
//


use serde::{
  Serialize,
  Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPattern {
  pub pattern:String,
  pub commands:Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watch {
  pub init:bool,
  pub verbose:bool,
  pub server:bool,
  pub patterns:Vec<WatchPattern>
}