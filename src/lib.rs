extern crate rand;
extern crate json;
extern crate ggez;
extern crate noframe;

mod settings;
mod game;
mod wall;
mod persons;
mod level;
mod animation;
mod gravity;
mod interactables;
mod id_generator;
mod menu;
mod level_manager;
mod score;
mod frames_counter;

use std::env;
use std::path;

use ggez::{
  GameResult,
  graphics,
  event
};

use settings::meta::*;
use settings::game::*;
use game::GameState;
use interactables::Interactable;

pub fn run() -> GameResult<()> {
  let mut ctx = ggez::ContextBuilder::new(
    NAME, AUTHORS
  ).window_setup(
    ggez::conf::WindowSetup::default().title(WINDOW_TITLE)
  ).window_mode(
    ggez::conf::WindowMode::default().dimensions(
      WINDOW_SIZE.w as u32,
      WINDOW_SIZE.h as u32,
      )
  ).build()?;

  if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.filesystem.mount(&path, true);
  }

  graphics::set_background_color(&mut ctx, BG_COLOR.into());
  let mut state = GameState::new(&mut ctx, WINDOW_SIZE)?;
  state.init(&mut ctx)?;
  return event::run(&mut ctx, &mut state);
}


pub fn join_str<'a>(str_one: &'a str, str_two: &'a str) -> String {
  format!("{}{}", str_one, str_two)
}

pub fn semantic(s: &str) -> String {
  let mut upper_at = 0;
  s.chars().enumerate()
    .map( |(i, c)| {
      let l = match c {
        '_' => {
          upper_at = i + 1;
          ' '
        },
        _   => c
      };
      if i == upper_at {
        l.to_ascii_uppercase()
      } else {
        l.to_ascii_lowercase()
      }
    }
    ).collect()
}
