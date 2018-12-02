use ggez::{
  Context,
  GameResult
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;

use settings::interactables::switch::*;
use animation::Animation;
use animation::Facing;
use super::Interactable;
use super::animations::switch;
use persons::Person;

enum State {
  On,
  Off,
  TurningOn,
  TurningOff
}

struct SwitchAnimations {
  pub on:          Animation,
  pub off:         Animation,
  pub turning_on:  Animation,
  pub turning_off: Animation
}

impl SwitchAnimations {
  pub fn new(ctx: &mut Context) -> Self {
    Self {
      on:          switch::new_on_animation(ctx),
      off:         switch::new_off_animation(ctx),
      turning_on:  switch::new_turning_on_animation(ctx),
      turning_off: switch::new_turning_off_animation(ctx)
    }
  }
}

pub struct Switch {
  point:            Point,
  size:             Size,
  origin:           Origin,
  state:            State,
  animations:       SwitchAnimations,
}

impl Switch {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    Self {
      point,
      size,
      origin:       Origin::TopLeft,
      state:        State::On,
      animations:   SwitchAnimations::new(ctx),
    }
  }

  fn animation(&self) -> &Animation {
    match self.state {
      State::On         => &self.animations.on,
      State::Off        => &self.animations.off,
      State::TurningOn  => &self.animations.turning_on,
      State::TurningOff => &self.animations.turning_off
    }
  }

  fn animation_mut(&mut self) -> &mut Animation {
    match self.state {
      State::On         => &mut self.animations.on,
      State::Off        => &mut self.animations.off,
      State::TurningOn  => &mut self.animations.turning_on,
      State::TurningOff => &mut self.animations.turning_off
    }
  }
}

impl Mask for Switch {
  fn point(&self)         -> &Point { &self.point }
  fn point_mut(&mut self) -> &mut Point { &mut self.point }
  fn size(&self)          -> &Size { &self.size }
  fn origin(&self)        -> &Origin { &self.origin }
}

impl Entity for Switch {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    match self.state {
      State::TurningOn if self.animation().played() >= 1 => {
        self.animation_mut().reset();
        self.state = State::On;
      }
      State::TurningOff => {
        self.animation_mut().reset();
        self.state = State::Off;
      }
      _ => ()
    };
    self.animation_mut().update();
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.animation().draw(ctx, self.point(), self.size(), &Facing::default())
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.animation().draw_offset(ctx, self.point(), self.size(), &Facing::default(), offset)
  }
}

impl Interactable for Switch {
  fn trigger<T: Person>(&mut self, person: &mut T) {
    match &self.state {
      State::On  => self.state = State::TurningOff,
      State::Off => self.state = State::TurningOn,
      _          => ()
    };
  }
}
