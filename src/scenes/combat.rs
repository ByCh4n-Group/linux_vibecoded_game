use tetra::Context;
use tetra::graphics::{self, Color, DrawParams, Rectangle};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::input::{self, Key};
use tetra::math::Vec2;
use rand::Rng;

use crate::game_state::GameState;
use crate::combat::CombatTurn;
use crate::defs::Scene;

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    if state.fade_alpha > 0.0 {
        state.fade_alpha -= 0.02;
    }

    match state.combat_data.turn {
        CombatTurn::Menu => {
            if input::is_key_pressed(ctx, Key::Left) {
                if state.combat_data.menu_selection > 0 {
                    state.combat_data.menu_selection -= 1;
                }
            }
            if input::is_key_pressed(ctx, Key::Right) {
                if state.combat_data.menu_selection < 2 {
                    state.combat_data.menu_selection += 1;
                }
            }
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) || input::is_key_pressed(ctx, Key::F) {
                match state.combat_data.menu_selection {
                    0 => { // Fight
                        state.combat_data.turn = CombatTurn::Fighting;
                        state.combat_data.timer = 0.0;
                        state.combat_data.attack_bar_active = true;
                        state.combat_data.attack_bar_pos = 50.0;
                        state.combat_data.action_text = "".to_string();
                    }
                    1 => { // Act
                        state.combat_data.turn = CombatTurn::Acting;
                        state.combat_data.action_text = "Check: Sans 1 ATK 1 DEF.\nThe easiest enemy. Can only deal 1 damage.".to_string();
                    }
                    2 => { // Mercy
                        state.combat_data.turn = CombatTurn::Mercy;
                        state.combat_data.action_text = "You spared Sans.".to_string();
                    }
                    _ => {}
                }
            }
        }
        CombatTurn::Fighting => {
            if state.combat_data.attack_bar_active {
                 state.combat_data.attack_bar_pos += state.combat_data.attack_bar_speed;
                 
                 // Box is 50 to 750 (width 700). Center is 400.
                 if state.combat_data.attack_bar_pos > 750.0 {
                     // Miss
                     state.combat_data.attack_bar_active = false;
                     state.combat_data.action_text = "MISS".to_string();
                     state.combat_data.timer = 0.0;
                 } else if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) {
                     state.combat_data.attack_bar_active = false;
                     let dist = (state.combat_data.attack_bar_pos - 400.0).abs();
                     let damage = if dist < 20.0 { 
                         100 
                     } else if dist < 100.0 { 
                         (100.0 - dist) as i32 
                     } else { 
                         0 
                     };
                     
                     if damage > 0 {
                         state.combat_data.action_text = format!("{} DMG", damage);
                         state.combat_data.sans_shake = 10.0;
                     } else {
                         state.combat_data.action_text = "MISS".to_string();
                     }
                     state.combat_data.timer = 0.0;
                 }
            } else {
                // Show result
                if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) {
                    state.combat_data.turn = CombatTurn::SansTurn;
                    state.combat_data.timer = 0.0;
                    state.combat_data.dialogue_text = "heh heh heh...".to_string();
                }
            }
        }
        CombatTurn::Acting | CombatTurn::Mercy => {
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) || input::is_key_pressed(ctx, Key::F) {
                if let CombatTurn::Mercy = state.combat_data.turn {
                    // End combat on mercy for now
                    state.scene = Scene::Desktop;
                    state.player_pos.x = 700.0; // Move player away so they don't re-trigger immediately
                } else {
                    state.combat_data.turn = CombatTurn::SansTurn;
                    state.combat_data.timer = 0.0;
                    state.combat_data.dialogue_text = "heh heh heh...".to_string();
                }
            }
        }
        CombatTurn::SansTurn => {
            state.combat_data.timer += 1.0;
            
            // Simple bone attack logic (placeholder)
            // In a real implementation, we would have a list of projectiles
            // For now, just drain health slowly if timer is in a certain range
            if state.combat_data.timer > 30.0 && state.combat_data.timer < 90.0 {
                state.player_health -= 0.1;
                if state.player_health < 0.0 {
                    state.player_health = 0.0;
                    // Game Over logic here?
                }
            }

            if state.combat_data.timer > 120.0 {
                state.combat_data.turn = CombatTurn::Menu;
                state.combat_data.dialogue_text = "You feel your sins crawling on your back.".to_string();
            }
        }
    }
    
    if state.combat_data.sans_shake > 0.0 {
        state.combat_data.sans_shake -= 0.5;
    }

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    graphics::clear(ctx, Color::BLACK);

    // Draw Sans
    let shake_x = if state.combat_data.sans_shake > 0.0 {
        rand::thread_rng().gen_range(-5.0..5.0)
    } else {
        0.0
    };
    
    if let Some(sans_texture) = &state.sans_combat_texture {
        let s_width = sans_texture.width() as f32;
        let s_height = sans_texture.height() as f32;
        let s_origin = Vec2::new(s_width / 2.0, s_height / 2.0);
        
        sans_texture.draw(ctx, DrawParams::new()
            .position(Vec2::new(400.0 + shake_x, 200.0))
            .origin(s_origin)
            .scale(Vec2::new(2.0, 2.0))
        );
    }

    // Draw UI Box
    let box_rect = Rectangle::new(50.0, 320.0, 700.0, 150.0);
    let box_mesh = Mesh::rectangle(ctx, ShapeStyle::Stroke(4.0), box_rect)?;
    box_mesh.draw(ctx, DrawParams::new().color(Color::WHITE));

    // Draw Text inside box
    let text_pos = Vec2::new(70.0, 340.0);
    match state.combat_data.turn {
        CombatTurn::Menu => {
            let mut t = Text::new(&state.combat_data.dialogue_text, state.font.clone());
            t.draw(ctx, DrawParams::new().position(text_pos).color(Color::WHITE));
        }
        CombatTurn::Fighting => {
            if state.combat_data.attack_bar_active {
                // Draw target area
                let target_rect = Rectangle::new(380.0, 320.0, 40.0, 150.0);
                let target_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, target_rect)?;
                target_mesh.draw(ctx, DrawParams::new().color(Color::rgb(0.5, 0.5, 0.5)));

                // Draw moving bar
                let bar_rect = Rectangle::new(state.combat_data.attack_bar_pos, 320.0, 10.0, 150.0);
                let bar_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bar_rect)?;
                bar_mesh.draw(ctx, DrawParams::new().color(Color::WHITE));
            } else {
                let mut t = Text::new(&state.combat_data.action_text, state.font.clone());
                t.draw(ctx, DrawParams::new().position(text_pos).color(Color::WHITE));
            }
        }
        CombatTurn::Acting | CombatTurn::Mercy => {
            let mut t = Text::new(&state.combat_data.action_text, state.font.clone());
            t.draw(ctx, DrawParams::new().position(text_pos).color(Color::WHITE));
        }
        CombatTurn::SansTurn => {
             // During enemy turn, maybe show some attack visuals?
             // For now, just show health draining
             let mut t = Text::new("Sans is attacking!", state.font.clone());
             t.draw(ctx, DrawParams::new().position(text_pos).color(Color::RED));
        }
    }

    // Draw Buttons (Fight, Act, Item, Mercy)
    let buttons = ["FIGHT", "ACT", "ITEM", "MERCY"];
    for (i, btn) in buttons.iter().enumerate() {
        let x = 100.0 + i as f32 * 160.0;
        let y = 500.0;
        let color = if state.combat_data.turn == CombatTurn::Menu && state.combat_data.menu_selection == i {
            Color::rgb(1.0, 1.0, 0.0) // Yellow
        } else {
            Color::rgb(1.0, 0.5, 0.0) // Orange
        };
        
        let mut t = Text::new(*btn, state.font.clone());
        t.draw(ctx, DrawParams::new().position(Vec2::new(x, y)).color(color));
    }

    // Draw Player Health
    let hp_text = format!("HP: {} / 100", state.player_health as i32);
    let mut t = Text::new(hp_text, state.font.clone());
    t.draw(ctx, DrawParams::new().position(Vec2::new(350.0, 480.0)).color(Color::WHITE));

    Ok(())
}
