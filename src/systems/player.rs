use crate::components::Player;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, mut transforms, time, input): Self::SystemData) {
        // Iterate over all planks and move them according to the input the user
        // provided.
        for (player, transform) in (&mut players, &mut transforms).join() {
            // let opt_movement = match paddle.side {
            //     Side::Left => input.axis_value("left_paddle"),
            //     Side::Right => input.axis_value("right_paddle"),
            // };
            let opt_movement_x = input.axis_value("move_x");
            if let Some(movement) = opt_movement_x {
                // use crate::ARENA_HEIGHT;
                transform.prepend_translation_x(
                    player.velocity * time.delta_seconds() * movement as f32,
                );
            }

            let opt_movement_y = input.axis_value("move_y");
            if let Some(movement) = opt_movement_y {
                transform.prepend_translation_y(
                    player.velocity * time.delta_seconds() * movement as f32,
                );
            }

            let jump = input.action_is_down("jump").unwrap_or(false);
            if jump {
                player.accelerate(1.);
            } else {
                player.accelerate(-1.);
            }

            // // We make sure the paddle remains in the arena.
            // let paddle_y = transform.translation().y;
            // transform.set_translation_y(
            //     paddle_y
            //         .max(paddle.height * 0.5)
            //         .min(ARENA_HEIGHT - paddle.height * 0.5),
            // );
            // }
        }
    }
}
