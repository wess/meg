//
// result.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::result;

pub type MegResult<T> = result::Result<T, Box<dyn std::error::Error + Send + Sync>>;