use rand::Rng;
use tetra::Context;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{self, Key};
use tetra::math::Vec2;

use crate::game_state::GameState;

pub fn update(ctx: &mut Context, state: &mut GameState) {
    if state.world.current_stage != 4 {
        return;
    }

    let dx = state.player.pos.x - state.world.eilish_pos.x;
    let dy = state.player.pos.y - state.world.eilish_pos.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 120.0 {
        if input::is_key_pressed(ctx, Key::F) {
            state.world.eilish_talking = true;
            state.world.eilish_dialogue_timer = 300.0; // 5 seconds

            let dialogues = [
                "Don't go to the dead space!",
                "It drains your health...",
                "I heard strange noises from there.",
                "Why are we here?",
                "Do you like my hair?",
                "Linux is complicated...",
                "Have you tried turning it off and on again?",
                "sudo rm -rf / ... just kidding!",
            ];
            let mut rng = rand::rng();
            state.world.eilish_current_dialogue =
                dialogues[rng.random_range(0..dialogues.len())].to_string();
        }
    } else {
        // Close textbox when out of range
        if state.world.eilish_talking {
            state.world.eilish_talking = false;
        }
    }

    if state.world.eilish_talking {
        state.world.eilish_dialogue_timer -= 1.0;
        if state.world.eilish_dialogue_timer <= 0.0 {
            state.world.eilish_talking = false;
        }
    }
}

pub fn draw(ctx: &mut Context, state: &GameState) -> tetra::Result {
    if state.world.current_stage != 4 {
        return Ok(());
    }

    // Draw Eilish
    if let Some(eilish_texture) = &state.world.eilish_texture {
        let e_width = eilish_texture.width() as f32;
        let e_height = eilish_texture.height() as f32;
        let e_origin = Vec2::new(e_width / 2.0, e_height / 2.0);

        eilish_texture.draw(
            ctx,
            DrawParams::new()
                .position(state.world.eilish_pos)
                .origin(e_origin)
                .scale(Vec2::new(0.1, 0.1)),
        );
    }

    // Interaction Prompt
    let dx = state.player.pos.x - state.world.eilish_pos.x;
    let dy = state.player.pos.y - state.world.eilish_pos.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 120.0 {
        let prompt = "Press F to Talk";
        let mut text = Text::new(prompt, state.font.clone());
        let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);

        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(
                    state.world.eilish_pos.x - width / 2.0,
                    state.world.eilish_pos.y - 60.0,
                ))
                .color(Color::GREEN),
        );
    }

    // Draw Dialogue
    if state.world.eilish_talking {
        // Draw Box
        if let Ok(box_rect) = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(50.0, 450.0, 700.0, 130.0),
        ) {
            box_rect.draw(
                ctx,
                DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, 0.8)),
            );
        }

        if let Ok(border_rect) = Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(2.0),
            Rectangle::new(50.0, 450.0, 700.0, 130.0),
        ) {
            border_rect.draw(ctx, DrawParams::new().color(Color::WHITE));
        }

        let mut text = Text::new(&state.world.eilish_current_dialogue, state.font.clone());
        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(70.0, 470.0))
                .color(Color::WHITE),
        );
    }

    Ok(())
}
