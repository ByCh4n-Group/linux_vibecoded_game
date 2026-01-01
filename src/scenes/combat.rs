use tetra::Context;
use tetra::graphics::{self, Color, DrawParams, Rectangle};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::input::{self, Key};
use tetra::math::Vec2;
use rand::Rng;

use crate::game_state::GameState;
use crate::combat::{CombatTurn, Bone};
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
                if state.combat_data.menu_selection < 3 {
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
                        let acts = [
                            "Check: Sans 1 ATK 1 DEF.\nThe easiest enemy. Can only deal 1 damage.",
                            "You told a joke about a skeleton.\nSans smiled.",
                            "You asked Sans to stop fighting.\nHe didn't respond.",
                            "You insulted Sans.\nHe just shrugged.",
                            "You looked at Sans.\nHe's still smiling."
                        ];
                        let mut rng = rand::thread_rng();
                        state.combat_data.action_text = acts[rng.gen_range(0..acts.len())].to_string();
                    }
                    2 => { // Item
                        state.combat_data.turn = CombatTurn::Acting; // Reuse acting state for now
                        state.combat_data.action_text = "You ate the Legendary Hero.\nYou recovered 40 HP!".to_string();
                        state.player_health = (state.player_health + 40.0).min(100.0);
                    }
                    3 => { // Mercy
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
                    
                    let jokes = [
                        "heh heh heh...",
                        "you're gonna have a bad time.",
                        "it's a beautiful day outside.",
                        "birds are singing, flowers are blooming...",
                        "on days like these, kids like you...",
                        "should be burning in hell.",
                        "take it easy, kid.",
                        "don't you have anything better to do?",
                        "i'm rooting for ya, kid.",
                        "geeeeeet dunked on!"
                    ];
                    let mut rng = rand::thread_rng();
                    state.combat_data.dialogue_text = jokes[rng.gen_range(0..jokes.len())].to_string();
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
                    
                    let jokes = [
                        "heh heh heh...",
                        "you're gonna have a bad time.",
                        "it's a beautiful day outside.",
                        "birds are singing, flowers are blooming...",
                        "on days like these, kids like you...",
                        "should be burning in hell.",
                        "take it easy, kid.",
                        "don't you have anything better to do?",
                        "i'm rooting for ya, kid.",
                        "geeeeeet dunked on!"
                    ];
                    let mut rng = rand::thread_rng();
                    state.combat_data.dialogue_text = jokes[rng.gen_range(0..jokes.len())].to_string();
                }
            }
        }
        CombatTurn::SansTurn => {
            if state.combat_data.timer == 0.0 {
                state.combat_data.heart_pos = Vec2::new(400.0, 395.0); // Center of box
                state.combat_data.heart_velocity = Vec2::zero();
                state.combat_data.bones.clear();
                
                // Randomize Attack Mode (Blue or Red)
                let mut rng = rand::thread_rng();
                state.combat_data.is_blue_mode = rng.gen_bool(0.5);
            }
            state.combat_data.timer += 1.0;

            // Physics & Movement (Universal Gravity for both modes)
            let speed = 4.0;
            
            // Gravity (Stronger)
            state.combat_data.heart_velocity.y += 0.9; 
            
            // Jump (Snappier)
            if input::is_key_pressed(ctx, Key::Up) && state.combat_data.can_jump {
                state.combat_data.heart_velocity.y = -13.0;
                state.combat_data.can_jump = false;
            }
            
            // Fast Fall
            if input::is_key_down(ctx, Key::Down) && !state.combat_data.can_jump {
                state.combat_data.heart_velocity.y += 1.5;
            }

            // Horizontal movement
            if input::is_key_down(ctx, Key::Left) { state.combat_data.heart_pos.x -= speed; }
            if input::is_key_down(ctx, Key::Right) { state.combat_data.heart_pos.x += speed; }
            
            // Apply velocity
            state.combat_data.heart_pos += state.combat_data.heart_velocity;

            // Floor collision (Box bottom is ~470)
            if state.combat_data.heart_pos.y > 440.0 {
                state.combat_data.heart_pos.y = 440.0;
                state.combat_data.heart_velocity.y = 0.0;
                state.combat_data.can_jump = true;
            }

            // Clamp Heart to Box
            state.combat_data.heart_pos.x = state.combat_data.heart_pos.x.clamp(55.0, 735.0);
            state.combat_data.heart_pos.y = state.combat_data.heart_pos.y.clamp(325.0, 455.0);

            // Spawn Bones (Complex Pattern)
            if state.combat_data.timer % 40.0 == 0.0 {
                let mut rng = rand::thread_rng();
                
                if state.combat_data.is_blue_mode {
                    // Blue Mode Patterns (Jump/Duck)
                    let pattern = rng.gen_range(0..3);
                    match pattern {
                        0 => { // Right to Left (Low)
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(800.0, 420.0),
                                size: Vec2::new(20.0, 50.0),
                                velocity: Vec2::new(-6.0, 0.0),
                            });
                        },
                        1 => { // Left to Right (High)
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(-50.0, 350.0),
                                size: Vec2::new(20.0, 60.0),
                                velocity: Vec2::new(6.0, 0.0),
                            });
                        },
                        2 => { // Both sides
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(800.0, 440.0),
                                size: Vec2::new(20.0, 30.0),
                                velocity: Vec2::new(-5.0, 0.0),
                            });
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(-50.0, 440.0),
                                size: Vec2::new(20.0, 30.0),
                                velocity: Vec2::new(5.0, 0.0),
                            });
                        },
                        _ => {}
                    }
                } else {
                    // Red Mode Patterns (Dodge Gaps)
                    // Bones come from right, full height but with a gap
                    let gap_y = rng.gen_range(330.0..440.0);
                    let gap_size = 60.0;
                    
                    // Top part
                    state.combat_data.bones.push(Bone {
                        pos: Vec2::new(800.0, 320.0),
                        size: Vec2::new(20.0, gap_y - 320.0),
                        velocity: Vec2::new(-5.0, 0.0),
                    });
                    
                    // Bottom part
                    state.combat_data.bones.push(Bone {
                        pos: Vec2::new(800.0, gap_y + gap_size),
                        size: Vec2::new(20.0, 470.0 - (gap_y + gap_size)),
                        velocity: Vec2::new(-5.0, 0.0),
                    });
                }
            }

            // Update Bones & Collision
            let heart_rect = Rectangle::new(state.combat_data.heart_pos.x, state.combat_data.heart_pos.y, 10.0, 10.0);
            
            let bones = &mut state.combat_data.bones;
            let mut hit = false;

            let mut i = 0;
            while i < bones.len() {
                let velocity = bones[i].velocity;
                bones[i].pos += velocity;
                
                let bone_rect = Rectangle::new(
                    bones[i].pos.x, 
                    bones[i].pos.y, 
                    bones[i].size.x, 
                    bones[i].size.y
                );

                if heart_rect.intersects(&bone_rect) {
                    hit = true;
                }

                // Remove if out of bounds
                if bones[i].pos.x < -50.0 || bones[i].pos.x > 850.0 {
                    bones.remove(i);
                } else {
                    i += 1;
                }
            }

            if hit {
                state.player_health -= 1.0;
            }

            if state.player_health <= 0.0 {
                state.player_health = 0.0;
                // Game Over logic could go here
            }

            if state.combat_data.timer > 400.0 { // Survival time
                state.combat_data.turn = CombatTurn::Menu;
                state.combat_data.dialogue_text = "You feel your sins crawling on your back.".to_string();
                state.combat_data.bones.clear();
                state.combat_data.is_blue_mode = false; // Reset to red for menu
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
            .scale(Vec2::new(4.0, 4.0))
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
             // Draw Dialogue Bubble
             let bubble_rect = Rectangle::new(450.0, 100.0, 200.0, 80.0);
             let bubble_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bubble_rect)?;
             bubble_mesh.draw(ctx, DrawParams::new().color(Color::WHITE));
             
             let bubble_border = Mesh::rectangle(ctx, ShapeStyle::Stroke(2.0), bubble_rect)?;
             bubble_border.draw(ctx, DrawParams::new().color(Color::BLACK));
 
             let mut t = Text::new("You're gonna\nhave a bad time.", state.font.clone());
             t.draw(ctx, DrawParams::new().position(Vec2::new(460.0, 110.0)).color(Color::BLACK));

             // Draw Heart
             // Clip to box
             graphics::set_scissor(ctx, Rectangle::new(50, 320, 700, 150));

             if let Some(heart_tex) = &state.heart_texture {
                 heart_tex.draw(ctx, DrawParams::new()
                    .position(state.combat_data.heart_pos)
                    .scale(Vec2::new(0.1, 0.1)) // Scaled down further
                    .color(Color::RED)
                 );
             } else {
                 // Fallback
                 let heart_rect = Rectangle::new(state.combat_data.heart_pos.x, state.combat_data.heart_pos.y, 10.0, 10.0);
                 let heart_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect)?;
                 heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
             }

             // Draw Bones
             for bone in &state.combat_data.bones {
                 if let Some(bone_tex) = &state.bone_texture {
                     // Stretch bone texture to fit size
                     // Assuming bone texture is vertical
                     let scale_x = bone.size.x / bone_tex.width() as f32;
                     let scale_y = bone.size.y / bone_tex.height() as f32;
                     
                     bone_tex.draw(ctx, DrawParams::new()
                        .position(bone.pos)
                        .scale(Vec2::new(scale_x, scale_y))
                        .color(Color::WHITE)
                     );
                 } else {
                     let bone_rect = Rectangle::new(bone.pos.x, bone.pos.y, bone.size.x, bone.size.y);
                     let bone_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bone_rect)?;
                     bone_mesh.draw(ctx, DrawParams::new().color(Color::WHITE));
                 }
             }
             
             graphics::reset_scissor(ctx);
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

        // Draw Heart Cursor
        if state.combat_data.turn == CombatTurn::Menu && state.combat_data.menu_selection == i {
             if let Some(heart_tex) = &state.heart_texture {
                 heart_tex.draw(ctx, DrawParams::new()
                    .position(Vec2::new(x - 20.0, y + 5.0))
                    .scale(Vec2::new(0.1, 0.1))
                    .color(Color::RED)
                 );
             } else {
                 let heart_rect = Rectangle::new(x - 20.0, y + 5.0, 10.0, 10.0);
                 let heart_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect)?;
                 heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
             }
        }
    }

    // Draw Player Health (Native Bar Style - Top Right)
    // HP Text
    let mut hp_label = Text::new("HP", state.font.clone());
    hp_label.draw(ctx, DrawParams::new().position(Vec2::new(550.0, 20.0)).color(Color::WHITE));

    // HP Bar Background (Red)
    let max_bar_width = 100.0; 
    let bar_bg_rect = Rectangle::new(590.0, 25.0, max_bar_width, 20.0);
    let bar_bg_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bar_bg_rect)?;
    bar_bg_mesh.draw(ctx, DrawParams::new().color(Color::RED));

    // HP Bar Foreground (Yellow)
    let current_bar_width = (state.player_health / 100.0) * max_bar_width;
    if current_bar_width > 0.0 {
        let bar_fg_rect = Rectangle::new(590.0, 25.0, current_bar_width, 20.0);
        let bar_fg_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bar_fg_rect)?;
        bar_fg_mesh.draw(ctx, DrawParams::new().color(Color::rgb(1.0, 1.0, 0.0)));
    }

    // HP Numbers
    let hp_text = format!("{}/100", state.player_health as i32);
    let mut t = Text::new(hp_text, state.font.clone());
    t.draw(ctx, DrawParams::new().position(Vec2::new(700.0, 20.0)).color(Color::WHITE));

    Ok(())
}
