//
// server.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use serde::{
  Serialize,
  Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
  pub port:u64,
  pub folder:String,
}

impl Server {
  pub fn default() -> Self {
    Self {
      port: 3000,
      folder: "./public".to_string()
    }
  }
}