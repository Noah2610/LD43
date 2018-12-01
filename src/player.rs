use ggez::{
  GameResult,
  Context,
  event::Keycode
};

use noframe::geo::prelude::*;
use noframe::entity::prelude::*;
use noframe::deltatime::Deltatime;

use animation::Animation;
use settings::player::*;

#[derive(PartialEq)]
enum Axis {
  X,
  Y
}

pub struct Player {
  point:        Point,
  size:         Size,
  origin:       Origin,
  velocity:     Point,
  max_velocity: Point,
  has_moved:    Vec<Axis>,
  animation:    Animation,
  dt:           Deltatime
}

impl Player {
  pub fn new(ctx: &mut Context, point: Point, size: Size) -> Self {
    let image_filepaths: Vec<String> = vec![
      ::join_str(IMAGES, "child1_1.png"),
      ::join_str(IMAGES, "child1_2.png"),
      ::join_str(IMAGES, "child1_3.png"),
      ::join_str(IMAGES, "child1_4.png")
    ];
    let image_update_intervals_ms = vec![
      250,
      250,
      250,
      250
    ];

    let animation = Animation::new(ctx, image_filepaths, image_update_intervals_ms);

    Self {
      point,
      size,
      origin: Origin::TopLeft,
      velocity: Point::new(0.0, 0.0),
      max_velocity: Point::new(MAX_SPEED, MAX_JUMP_SPEED),
      has_moved: Vec::new(),
      animation,
      dt: Deltatime::new()
    }
  }

  pub fn keys_pressed(&mut self, keycodes: &Vec<Keycode>) {
    for keycode in keycodes {
      if let Some(point) = match keycode {
        &controls::LEFT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( -SPEED_INCREASE, 0.0 ))
          } else { None }
        }
        &controls::RIGHT => {
          if !self.has_moved(Axis::X) {
            self.moved_on_axis(Axis::X);
            Some(Point::new( SPEED_INCREASE, 0.0 ))
          } else { None }
        }
        &controls::JUMP => {
          if !self.has_moved(Axis::Y) {
            self.moved_on_axis(Axis::Y);
            Some(Point::new( 0.0, SPEED_INCREASE ))
          } else { None }
        }
        _ => None
      } {
        self.add_velocity(&point);
      }
    }
  }

  fn moved_on_axis(&mut self, axis: Axis) {
    if !self.has_moved.iter().any( |a| &axis == a ) {
      self.has_moved.push(axis);
    }
  }

  fn has_moved(&self, axis: Axis) -> bool {
    self.has_moved.iter().any( |a| &axis == a )
  }

  fn handle_velocity(&mut self) {
    let decr = SPEED_DECREASE; //* self.dt.secs();
    let decr_vel = Point::new(
      if !self.has_moved(Axis::X) {
        decr
      } else { 0.0 },
      if !self.has_moved(Axis::Y) {
        decr
      } else { 0.0 }
    );
    self.decrease_velocity(&decr_vel);
    self.has_moved.clear();
  }
}

impl Mask for Player {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

impl Entity for Player {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.handle_velocity();
    self.animation.update()?;
    self.dt.update();
    Ok(())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    self.animation.draw(ctx, &self.point, &self.size)
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    self.animation.draw_offset(ctx, &self.point, &self.size, offset)
  }
}

impl Velocity for Player {
  fn velocity(&self) -> &Point {
    &self.velocity
  }
  fn usable_velocity(&self) -> Point {
    self.velocity.mult_axes_by(self.dt.secs())
  }
  fn velocity_mut(&mut self) -> &mut Point {
    &mut self.velocity
  }
  fn max_velocity(&self) -> Point {
    self.max_velocity.clone()
  }
}

impl Movement for Player { }
