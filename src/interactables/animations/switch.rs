use ggez::Context;
use animation::Animation;
use settings::interactables::*;

pub fn new_on_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "switch_on_", color)
                 ],
                 vec![
                 1000
                 ])
}

pub fn new_off_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "switch_off_", color)
                 ],
                 vec![
                 1000
                 ])
}

pub fn new_turning_on_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "switch_1_", color),
                 format!("{}{}{}.png", IMAGES, "switch_1_", color),
                 format!("{}{}{}.png", IMAGES, "switch_2_", color),
                 format!("{}{}{}.png", IMAGES, "switch_3_", color),
                 ],
                 vec![
                 250,
                 250,
                 250,
                 250
                 ])
}

pub fn new_turning_off_animation(ctx: &mut Context, color: &str) -> Animation {
  Animation::new(ctx,
                 vec![
                 format!("{}{}{}.png", IMAGES, "switch_3_", color),
                 format!("{}{}{}.png", IMAGES, "switch_3_", color),
                 format!("{}{}{}.png", IMAGES, "switch_2_", color),
                 format!("{}{}{}.png", IMAGES, "switch_1_", color)
                 ],
                 vec![
                 250,
                 250,
                 250,
                 250
                 ])
}
