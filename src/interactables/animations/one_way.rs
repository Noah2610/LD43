use ggez::Context;
use animation::Animation;
use settings::interactables::*;
use settings::res::MISSING_IMAGE;

pub fn new_main_animation(ctx: &mut Context) -> Animation {
  Animation::new(ctx,
                 vec![
                 ::join_str(IMAGES, "oneway.png")
                 ], vec![
                 1000
                 ])
}
