//
// main.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use meg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  meg::run().await?;

  Ok(())
}