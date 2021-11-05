//
// handle.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/10/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Handle {
  pub command:String,
  pub paths:Vec<PathBuf>,
}
