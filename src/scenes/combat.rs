use rand::Rng;
use tetra::Context;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{self, Color, DrawParams, Rectangle};
use tetra::input::{self, Key};
use tetra::math::Vec2;

use crate::combat::{Bone, CombatTurn};
use crate::defs::Scene;
use crate::game_state::GameState;

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    if state.fade_alpha > 0.0 {
        state.fade_alpha -= 0.02;
    }

    match state.combat_data.turn {
        CombatTurn::Menu => {
            if input::is_key_pressed(ctx, Key::Left) && state.combat_data.menu_selection > 0 {
                state.combat_data.menu_selection -= 1;
            }
            if input::is_key_pressed(ctx, Key::Right) && state.combat_data.menu_selection < 3 {
                state.combat_data.menu_selection += 1;
            }
            if input::is_key_pressed(ctx, Key::Z)
                || input::is_key_pressed(ctx, Key::Enter)
                || input::is_key_pressed(ctx, Key::F)
            {
                match state.combat_data.menu_selection {
                    0 => {
                        // Cihad (Attack)
                        state.combat_data.turn = CombatTurn::Fighting;
                        state.combat_data.timer = 0.0;
                        state.combat_data.attack_bar_active = true;
                        state.combat_data.attack_bar_pos = 50.0;
                        state.combat_data.action_text = "".to_string();
                    }
                    1 => {
                        // Tekfir (Act)
                        state.combat_data.turn = CombatTurn::TekfirSubMenu;
                        state.combat_data.sub_menu_selection = 0;
                    }
                    2 => {
                        // Item
                        state.combat_data.turn = CombatTurn::ItemSubMenu;
                        state.combat_data.sub_menu_selection = 0;
                    }
                    3 => {
                        // Tebliğ (Mercy)
                        state.combat_data.turn = CombatTurn::TebligSubMenu;
                        state.combat_data.sub_menu_selection = 0;
                    }
                    _ => {}
                }
            }
        }
        CombatTurn::TekfirSubMenu => {
            if input::is_key_pressed(ctx, Key::Up) && state.combat_data.sub_menu_selection > 0 {
                state.combat_data.sub_menu_selection -= 1;
            }
            if input::is_key_pressed(ctx, Key::Down) && state.combat_data.sub_menu_selection < 8 {
                state.combat_data.sub_menu_selection += 1;
            }
            if input::is_key_pressed(ctx, Key::X) {
                state.combat_data.turn = CombatTurn::Menu;
            }
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) {
                state.combat_data.turn = CombatTurn::ResultText;

                // Update stats
                if let Some(user) = &mut state.system.current_user {
                    user.tekfir_count += 1;
                }
                state.system.save_users();

                let mut rng = rand::rng();
                match state.combat_data.sub_menu_selection {
                    0 => {
                        // Müşrik
                        let texts = [
                            "Ona Müşrik dedin.\nSana güldü.",
                            "Ona Müşrik dedin.\n'Sen de kimsin?' dedi.",
                            "Ona Müşrik dedin.\nUmursamadı bile.",
                        ];
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    1 => {
                        // Fasık
                        let texts = [
                            "Ona Fasık dedin.\nUmursamadı.",
                            "Ona Fasık dedin.\nEsneyerek cevap verdi.",
                            "Ona Fasık dedin.\nSana acıyarak baktı.",
                        ];
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    2 => {
                        // Münafık
                        let texts = &state.texts.combat_actions.munafik;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    3 => {
                        // Kafir
                        let texts = &state.texts.combat_actions.kafir;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    4 => {
                        // Zındık
                        let texts = &state.texts.combat_actions.zindik;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    5 => {
                        // Tağut
                        let texts = &state.texts.combat_actions.tagut;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    6 => {
                        // Deccal
                        let texts = &state.texts.combat_actions.deccal;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    7 => {
                        // Ebu Cehil
                        let texts = &state.texts.combat_actions.ebu_cehil;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    8 => {
                        // Yecüc
                        let texts = &state.texts.combat_actions.yecuc;
                        state.combat_data.action_text =
                            texts[rng.random_range(0..texts.len())].to_string();
                    }
                    _ => {}
                }
            }
        }
        CombatTurn::ItemSubMenu => {
            if input::is_key_pressed(ctx, Key::Up) && state.combat_data.sub_menu_selection > 0 {
                state.combat_data.sub_menu_selection -= 1;
            }
            if input::is_key_pressed(ctx, Key::Down) && state.combat_data.sub_menu_selection < 3 {
                state.combat_data.sub_menu_selection += 1;
            }
            if input::is_key_pressed(ctx, Key::X) {
                state.combat_data.turn = CombatTurn::Menu;
            }
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) {
                state.combat_data.turn = CombatTurn::ResultText;
                match state.combat_data.sub_menu_selection {
                    0 => {
                        // Zemzem
                        state.combat_data.action_text =
                            "Zemzem içtin.\nCanın 50 arttı!".to_string();
                        state.player.health = (state.player.health + 50.0).min(100.0);
                    }
                    1 => {
                        // Hurma
                        state.combat_data.action_text = "Hurma yedin.\nCanın 20 arttı!".to_string();
                        state.player.health = (state.player.health + 20.0).min(100.0);
                    }
                    2 => {
                        // Zeytin
                        state.combat_data.action_text =
                            "Zeytin yedin.\nCanın 10 arttı!".to_string();
                        state.player.health = (state.player.health + 10.0).min(100.0);
                    }
                    3 => {
                        // Ayetel Kürsi
                        state.combat_data.action_text =
                            "Ayetel Kürsi okudun.\nCanın tamamen doldu!".to_string();
                        state.player.health = 100.0;
                    }
                    _ => {}
                }
            }
        }
        CombatTurn::TebligSubMenu => {
            if input::is_key_pressed(ctx, Key::Up) && state.combat_data.sub_menu_selection > 0 {
                state.combat_data.sub_menu_selection -= 1;
            }
            if input::is_key_pressed(ctx, Key::Down) && state.combat_data.sub_menu_selection < 1 {
                state.combat_data.sub_menu_selection += 1;
            }
            if input::is_key_pressed(ctx, Key::X) {
                state.combat_data.turn = CombatTurn::Menu;
            }
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) {
                match state.combat_data.sub_menu_selection {
                    0 => {
                        // Tebliğ Et
                        state.combat_data.turn = CombatTurn::ResultText;

                        // Update stats
                        if let Some(user) = &mut state.system.current_user {
                            user.teblig_count += 1;
                        }
                        state.system.save_users();

                        let acts = [
                            "Ona İslam'ı anlattın.\nSana güldü.",
                            "Tövbe etmesini söyledin.\nUmursamadı.",
                            "Cehennem ateşinden bahsettin.\nOmuz silkti.",
                            "Ona hidayet diledin.\nHala sırıtıyor.",
                            "Ona Kuran okudun.\nRahatsız oldu.",
                            "Ona hadis anlattın.\nKulaklarını tıkadı.",
                            "Ona ölümü hatırlattın.\nÜrperdi ama belli etmedi.",
                            "Ona cenneti anlattın.\n'İlgilenmiyorum' dedi.",
                            "Ona selam verdin.\nAlmadı.",
                            "Ona dua ettin.\nGözlerini devirdi.",
                            "Ona zemzem ikram ettin.\n'Kola yok mu?' dedi.",
                            "Ona misvak uzattın.\n'Diş fırçam var' dedi.",
                            "Ona takke takmaya çalıştın.\nKafasını çekti.",
                            "Ona tesbih hediye ettin.\nBoncuk sandı.",
                            "Ona Cuma mesajı attın.\nEngelledi.",
                            "Ona ilahi dinlettin.\nKulaklığını taktı.",
                            "Ona hurma verdin.\nÇekirdeğini sana attı.",
                            "Ona gül suyu sıktın.\n'Alerjim var' dedi.",
                            "Ona seccade serdin.\nÜstüne bastı.",
                            "Ona ezan okudun.\n'Sesin kötü' dedi.",
                        ];
                        let mut rng = rand::rng();
                        state.combat_data.action_text =
                            acts[rng.random_range(0..acts.len())].to_string();
                    }
                    1 => {
                        // Kaç
                        state.scene = Scene::Desktop;
                        state.player.pos.x = 700.0;
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
                    state.combat_data.turn = CombatTurn::ResultText;
                } else if input::is_key_pressed(ctx, Key::Z)
                    || input::is_key_pressed(ctx, Key::Enter)
                {
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
                        state.combat_data.action_text = format!("CİHAD! {} HASAR", damage);

                        // Update stats
                        if let Some(user) = &mut state.system.current_user {
                            user.cihad_count += 1;
                        }
                        state.system.save_users();

                        state.combat_data.sans_shake = 10.0;
                        state.combat_data.sans_hp -= damage;
                        if state.combat_data.sans_hp <= 0 {
                            state.combat_data.sans_hp = 0;
                            state.combat_data.dialogue_text =
                                "welp... i'm going to grillby's.".to_string();
                            state.combat_data.turn = CombatTurn::ResultText; // Or a win state
                        }
                    } else {
                        state.combat_data.action_text = "MISS".to_string();
                    }
                    state.combat_data.timer = 0.0;
                    state.combat_data.turn = CombatTurn::ResultText;
                }
            }
        }
        CombatTurn::ResultText => {
            if input::is_key_pressed(ctx, Key::Z)
                || input::is_key_pressed(ctx, Key::Enter)
                || input::is_key_pressed(ctx, Key::F)
            {
                if state.combat_data.sans_hp <= 0 {
                    // Victory transition
                    state.scene = Scene::Desktop;
                    state.player.pos.x = 700.0;
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
                        "geeeeeet dunked on!",
                    ];
                    let mut rng = rand::rng();
                    state.combat_data.dialogue_text =
                        jokes[rng.random_range(0..jokes.len())].to_string();
                }
            }
        }
        CombatTurn::SansTurn => {
            if state.combat_data.timer == 0.0 {
                state.combat_data.heart_pos = Vec2::new(400.0, 395.0); // Center of box
                state.combat_data.heart_velocity = Vec2::zero();
                state.combat_data.bones.clear();

                // Randomize Attack Mode (0: Gravity, 1: Free Flight)
                let mut rng = rand::rng();
                state.combat_data.mode = rng.random_range(0..2);
            }
            state.combat_data.timer += 1.0;

            // Physics & Movement
            let speed = 4.0;

            if state.combat_data.mode == 0 {
                // Gravity Mode
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
                if input::is_key_down(ctx, Key::Left) {
                    state.combat_data.heart_pos.x -= speed;
                }
                if input::is_key_down(ctx, Key::Right) {
                    state.combat_data.heart_pos.x += speed;
                }

                // Apply velocity
                state.combat_data.heart_pos += state.combat_data.heart_velocity;

                // Floor collision (Box bottom is ~470)
                if state.combat_data.heart_pos.y > 440.0 {
                    state.combat_data.heart_pos.y = 440.0;
                    state.combat_data.heart_velocity.y = 0.0;
                    state.combat_data.can_jump = true;
                }
            } else {
                // Free Flight Mode
                if input::is_key_down(ctx, Key::Left) {
                    state.combat_data.heart_pos.x -= speed;
                }
                if input::is_key_down(ctx, Key::Right) {
                    state.combat_data.heart_pos.x += speed;
                }
                if input::is_key_down(ctx, Key::Up) {
                    state.combat_data.heart_pos.y -= speed;
                }
                if input::is_key_down(ctx, Key::Down) {
                    state.combat_data.heart_pos.y += speed;
                }
            }

            // Clamp Heart to Box (Tighter bounds)
            state.combat_data.heart_pos.x = state.combat_data.heart_pos.x.clamp(60.0, 730.0);
            state.combat_data.heart_pos.y = state.combat_data.heart_pos.y.clamp(330.0, 460.0);

            // Spawn Bones (Complex Pattern)
            if state.combat_data.timer % 40.0 == 0.0 {
                let mut rng = rand::rng();

                if state.combat_data.mode == 0 {
                    // Gravity Mode Patterns (Jump/Duck)
                    let pattern = rng.random_range(0..3);
                    match pattern {
                        0 => {
                            // Right to Left (Low)
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(800.0, 420.0),
                                size: Vec2::new(20.0, 50.0),
                                velocity: Vec2::new(-6.0, 0.0),
                            });
                        }
                        1 => {
                            // Left to Right (High) - Touching Top
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(-50.0, 320.0),
                                size: Vec2::new(20.0, 90.0),
                                velocity: Vec2::new(6.0, 0.0),
                            });
                        }
                        2 => {
                            // Both sides
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
                        }
                        _ => {}
                    }
                } else {
                    // Omni-directional Mode (Updated)
                    // 0: Left, 1: Right, 2: Top, 3: Bottom, 4: Top-Left, 5: Bottom-Left
                    let direction = rng.random_range(0..6);

                    match direction {
                        0 => {
                            // Left -> Right
                            let y_pos = rng.random_range(330.0..440.0);
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(-50.0, y_pos),
                                size: Vec2::new(100.0, 10.0), // Thinner, longer
                                velocity: Vec2::new(7.0, 0.0),
                            });
                        }
                        1 => {
                            // Right -> Left
                            let y_pos = rng.random_range(330.0..440.0);
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(800.0, y_pos),
                                size: Vec2::new(100.0, 10.0),
                                velocity: Vec2::new(-7.0, 0.0),
                            });
                        }
                        2 => {
                            // Top -> Bottom
                            let x_pos = rng.random_range(60.0..730.0);
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(x_pos, 250.0), // Above box
                                size: Vec2::new(10.0, 100.0), // Vertical
                                velocity: Vec2::new(0.0, 5.0),
                            });
                        }
                        3 => {
                            // Bottom -> Top
                            let x_pos = rng.random_range(60.0..730.0);
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(x_pos, 500.0), // Below box
                                size: Vec2::new(10.0, 100.0),
                                velocity: Vec2::new(0.0, -5.0),
                            });
                        }
                        4 => {
                            // Top-Left -> Diagonal Down-Right
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(0.0, 250.0),
                                size: Vec2::new(15.0, 60.0),
                                velocity: Vec2::new(4.0, 4.0),
                            });
                        }
                        5 => {
                            // Bottom-Left -> Diagonal Up-Right
                            state.combat_data.bones.push(Bone {
                                pos: Vec2::new(0.0, 500.0),
                                size: Vec2::new(15.0, 60.0),
                                velocity: Vec2::new(4.0, -4.0),
                            });
                        }
                        _ => {}
                    }
                }
            }

            // Update Bones & Collision
            let heart_rect = Rectangle::new(
                state.combat_data.heart_pos.x,
                state.combat_data.heart_pos.y,
                10.0,
                10.0,
            );

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
                    bones[i].size.y,
                );

                if heart_rect.intersects(&bone_rect) {
                    hit = true;
                }

                // Remove if out of bounds
                if bones[i].pos.x < -50.0
                    || bones[i].pos.x > 850.0
                    || bones[i].pos.y < 200.0
                    || bones[i].pos.y > 600.0
                {
                    bones.remove(i);
                } else {
                    i += 1;
                }
            }

            if hit {
                state.player.health -= 1.0;
            }

            if state.player.health <= 0.0 {
                state.player.health = 0.0;
                state.game_over_state = crate::game_state::GameOverState::new();
                state.scene = crate::defs::Scene::KernelPanic;

                // Reset Game State on Death
                state.world.current_stage = 1;
                state.player.pos = Vec2::new(400.0, 300.0);
                state.player.direction = crate::defs::Direction::Front;

                // Reset User Progress if logged in
                if let Some(user) = &mut state.system.current_user {
                    user.current_stage = 1;
                    // We also need to update the user in the main list
                    if let Some(idx) = state
                        .system
                        .users
                        .iter()
                        .position(|u| u.username == user.username)
                    {
                        state.system.users[idx].current_stage = 1;
                        state.system.save_users();
                    }
                }
            }

            if state.combat_data.timer > 400.0 {
                // Survival time
                state.combat_data.turn = CombatTurn::Menu;
                state.combat_data.dialogue_text =
                    "You feel your sins crawling on your back.".to_string();
                state.combat_data.bones.clear();
                state.combat_data.mode = 0; // Reset to default
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
        rand::rng().random_range(-5.0..5.0)
    } else {
        0.0
    };

    if let Some(sans_texture) = &state.world.sans_combat_texture {
        let s_width = sans_texture.width() as f32;
        let s_height = sans_texture.height() as f32;
        let s_origin = Vec2::new(s_width / 2.0, s_height / 2.0);

        sans_texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(400.0 + shake_x, 200.0))
                .origin(s_origin)
                .scale(Vec2::new(4.0, 4.0)),
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
            t.draw(
                ctx,
                DrawParams::new().position(text_pos).color(Color::WHITE),
            );
        }
        CombatTurn::TekfirSubMenu => {
            let options = [
                "* Müşrik",
                "* Fasık",
                "* Münafık",
                "* Kafir",
                "* Zındık",
                "* Tağut",
                "* Deccal",
                "* Ebu Cehil",
                "* Yecüc",
            ];
            for (i, opt) in options.iter().enumerate() {
                let col = i / 3;
                let row = i % 3;

                let x = 100.0 + col as f32 * 220.0;
                let y = 340.0 + row as f32 * 30.0;

                let mut t = Text::new(*opt, state.font.clone());
                t.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(x, y))
                        .color(Color::WHITE),
                );

                if state.combat_data.sub_menu_selection == i {
                    if let Some(heart_tex) = &state.heart_texture {
                        heart_tex.draw(
                            ctx,
                            DrawParams::new()
                                .position(Vec2::new(x - 30.0, y + 5.0))
                                .scale(Vec2::new(0.08, 0.08))
                                .color(Color::RED),
                        );
                    } else {
                        let heart_rect = Rectangle::new(x - 30.0, y + 5.0, 10.0, 10.0);
                        let heart_mesh =
                            Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect).unwrap();
                        heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
                    }
                }
            }
        }
        CombatTurn::ItemSubMenu => {
            let options = ["* Zemzem", "* Hurma", "* Zeytin", "* Ayetel Kürsi"];
            for (i, opt) in options.iter().enumerate() {
                let mut t = Text::new(*opt, state.font.clone());
                t.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(100.0, 340.0 + i as f32 * 30.0))
                        .color(Color::WHITE),
                );

                if state.combat_data.sub_menu_selection == i {
                    if let Some(heart_tex) = &state.heart_texture {
                        heart_tex.draw(
                            ctx,
                            DrawParams::new()
                                .position(Vec2::new(70.0, 345.0 + i as f32 * 30.0))
                                .scale(Vec2::new(0.08, 0.08))
                                .color(Color::RED),
                        );
                    } else {
                        let heart_rect = Rectangle::new(70.0, 345.0 + i as f32 * 30.0, 10.0, 10.0);
                        let heart_mesh =
                            Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect).unwrap();
                        heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
                    }
                }
            }
        }
        CombatTurn::TebligSubMenu => {
            let options = ["* Tebliğ Et", "* Kaç"];
            for (i, opt) in options.iter().enumerate() {
                let mut t = Text::new(*opt, state.font.clone());
                t.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(100.0, 340.0 + i as f32 * 30.0))
                        .color(Color::WHITE),
                );

                if state.combat_data.sub_menu_selection == i {
                    if let Some(heart_tex) = &state.heart_texture {
                        heart_tex.draw(
                            ctx,
                            DrawParams::new()
                                .position(Vec2::new(70.0, 345.0 + i as f32 * 30.0))
                                .scale(Vec2::new(0.08, 0.08))
                                .color(Color::RED),
                        );
                    } else {
                        let heart_rect = Rectangle::new(70.0, 345.0 + i as f32 * 30.0, 10.0, 10.0);
                        let heart_mesh =
                            Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect).unwrap();
                        heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
                    }
                }
            }
        }
        CombatTurn::Fighting => {
            if state.combat_data.attack_bar_active {
                // Draw Background Track
                let _track_rect = Rectangle::new(50.0, 320.0, 700.0, 150.0);
                // No fill, just the box is already there.

                // Draw "Eye" Shape Target Area
                // We simulate an eye shape with a few concentric rectangles or ellipses if possible.
                // Let's use a series of fading rectangles to create a "glow" center.

                let center_x = 400.0;
                let center_y = 395.0;

                // Outer glow
                let glow_width = 120.0;
                let glow_height = 130.0;
                let glow_rect = Rectangle::new(
                    center_x - glow_width / 2.0,
                    center_y - glow_height / 2.0,
                    glow_width,
                    glow_height,
                );
                let glow_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, glow_rect).unwrap();
                glow_mesh.draw(
                    ctx,
                    DrawParams::new().color(Color::rgba(0.5, 0.5, 0.5, 0.3)),
                );

                // Inner Target
                let target_width = 40.0;
                let target_height = 130.0;
                let target_rect = Rectangle::new(
                    center_x - target_width / 2.0,
                    center_y - target_height / 2.0,
                    target_width,
                    target_height,
                );
                let target_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, target_rect).unwrap();

                // Pulse effect for target color
                let pulse = (state.combat_data.timer * 0.1).sin().abs();
                let target_color =
                    Color::rgb(0.2 + 0.3 * pulse, 0.2 + 0.3 * pulse, 0.2 + 0.3 * pulse);
                target_mesh.draw(ctx, DrawParams::new().color(target_color));

                // Center "Perfect" line (Double line)
                let line_h = 140.0;
                let c_line1 = Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(center_x - 2.0, center_y - line_h / 2.0, 4.0, line_h),
                )
                .unwrap();
                c_line1.draw(ctx, DrawParams::new().color(Color::WHITE));

                // Moving Bar
                let bar_x = state.combat_data.attack_bar_pos;
                let bar_w = 14.0;
                let bar_h = 140.0;

                // Draw "Ghost" trails for speed effect
                for i in 1..4 {
                    let offset = i as f32 * 15.0;
                    if bar_x - offset > 50.0 {
                        let trail_rect =
                            Rectangle::new(bar_x - offset, center_y - bar_h / 2.0, bar_w, bar_h);
                        let trail_mesh =
                            Mesh::rectangle(ctx, ShapeStyle::Fill, trail_rect).unwrap();
                        trail_mesh.draw(
                            ctx,
                            DrawParams::new().color(Color::rgba(
                                1.0,
                                1.0,
                                1.0,
                                0.3 - (i as f32 * 0.08),
                            )),
                        );
                    }
                }

                // Main Bar (Inverted colors or flashing)
                let bar_rect = Rectangle::new(bar_x, center_y - bar_h / 2.0, bar_w, bar_h);
                let bar_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bar_rect).unwrap();

                // Rainbow effect if close to center
                let dist = (bar_x - center_x).abs();
                let bar_color = if dist < 20.0 {
                    let _hue = (state.combat_data.timer * 5.0) % 360.0;
                    // Simple RGB approximation for rainbow not worth importing a lib, just flash Cyan/White
                    if (state.combat_data.timer as i32 / 4) % 2 == 0 {
                        Color::rgb(0.0, 1.0, 1.0)
                    } else {
                        Color::WHITE
                    }
                } else {
                    Color::WHITE
                };

                bar_mesh.draw(ctx, DrawParams::new().color(bar_color));

                // Outline
                let bar_outline = Mesh::rectangle(ctx, ShapeStyle::Stroke(3.0), bar_rect).unwrap();
                bar_outline.draw(ctx, DrawParams::new().color(Color::BLACK));
            } else {
                let mut t = Text::new(&state.combat_data.action_text, state.font.clone());
                t.draw(
                    ctx,
                    DrawParams::new().position(text_pos).color(Color::WHITE),
                );
            }
        }
        CombatTurn::ResultText => {
            let mut t = Text::new(&state.combat_data.action_text, state.font.clone());
            t.draw(
                ctx,
                DrawParams::new().position(text_pos).color(Color::WHITE),
            );
        }
        CombatTurn::SansTurn => {
            // Draw Dialogue Bubble
            let bubble_rect = Rectangle::new(450.0, 100.0, 200.0, 80.0);
            let bubble_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bubble_rect).unwrap();
            bubble_mesh.draw(ctx, DrawParams::new().color(Color::WHITE));

            let bubble_border = Mesh::rectangle(ctx, ShapeStyle::Stroke(2.0), bubble_rect).unwrap();
            bubble_border.draw(ctx, DrawParams::new().color(Color::BLACK));

            let mut t = Text::new("You're gonna\nhave a bad time.", state.font.clone());
            t.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(460.0, 110.0))
                    .color(Color::BLACK),
            );

            // Draw Heart
            // Clip to box
            graphics::set_scissor(ctx, Rectangle::new(50, 320, 700, 150));

            if let Some(heart_tex) = &state.heart_texture {
                heart_tex.draw(
                    ctx,
                    DrawParams::new()
                        .position(state.combat_data.heart_pos)
                        .scale(Vec2::new(0.1, 0.1)) // Scaled down further
                        .color(Color::RED),
                );
            } else {
                // Fallback
                let heart_rect = Rectangle::new(
                    state.combat_data.heart_pos.x,
                    state.combat_data.heart_pos.y,
                    10.0,
                    10.0,
                );
                let heart_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect).unwrap();
                heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
            }

            // Draw Bones
            for bone in &state.combat_data.bones {
                if let Some(bone_tex) = &state.bone_texture {
                    // Stretch bone texture to fit size
                    // Assuming bone texture is vertical
                    let scale_x = bone.size.x / bone_tex.width() as f32;
                    let scale_y = bone.size.y / bone_tex.height() as f32;

                    bone_tex.draw(
                        ctx,
                        DrawParams::new()
                            .position(bone.pos)
                            .scale(Vec2::new(scale_x, scale_y))
                            .color(Color::WHITE),
                    );
                } else {
                    let bone_rect =
                        Rectangle::new(bone.pos.x, bone.pos.y, bone.size.x, bone.size.y);
                    let bone_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bone_rect).unwrap();
                    bone_mesh.draw(ctx, DrawParams::new().color(Color::WHITE));
                }
            }

            graphics::reset_scissor(ctx);
        }
    }

    // Draw Buttons (Fight, Act, Item, Mercy)
    let buttons = ["CİHAD", "TEKFİR", "ITEM", "TEBLİĞ"];
    for (i, btn) in buttons.iter().enumerate() {
        let x = 100.0 + i as f32 * 160.0;
        let y = 500.0;
        let color = if state.combat_data.turn == CombatTurn::Menu
            && state.combat_data.menu_selection == i
        {
            Color::rgb(1.0, 1.0, 0.0) // Yellow
        } else {
            Color::rgb(1.0, 0.5, 0.0) // Orange
        };

        let mut t = Text::new(*btn, state.font.clone());
        t.draw(
            ctx,
            DrawParams::new().position(Vec2::new(x, y)).color(color),
        );

        // Draw Heart Cursor
        if state.combat_data.turn == CombatTurn::Menu && state.combat_data.menu_selection == i {
            if let Some(heart_tex) = &state.heart_texture {
                heart_tex.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(x - 25.0, y + 2.0))
                        .scale(Vec2::new(0.08, 0.08))
                        .color(Color::RED),
                );
            } else {
                let heart_rect = Rectangle::new(x - 20.0, y + 5.0, 10.0, 10.0);
                let heart_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, heart_rect).unwrap();
                heart_mesh.draw(ctx, DrawParams::new().color(Color::RED));
            }
        }
    }

    // Draw Sans Health (Top Left)
    // HP Text
    let mut sans_hp_label = Text::new("SANS HP", state.font.clone());
    sans_hp_label.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(20.0, 20.0))
            .color(Color::WHITE),
    );

    // HP Bar Background (Dark Gray)
    let sans_max_bar_width = 200.0;
    let sans_bar_bg_rect = Rectangle::new(120.0, 25.0, sans_max_bar_width, 20.0);
    let sans_bar_bg_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, sans_bar_bg_rect)?;
    sans_bar_bg_mesh.draw(ctx, DrawParams::new().color(Color::rgb(0.2, 0.2, 0.2)));

    // HP Bar Foreground (Blue)
    let sans_current_bar_width = (state.combat_data.sans_hp as f32
        / state.combat_data.sans_max_hp as f32)
        * sans_max_bar_width;
    if sans_current_bar_width > 0.0 {
        let sans_bar_fg_rect = Rectangle::new(120.0, 25.0, sans_current_bar_width, 20.0);
        let sans_bar_fg_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, sans_bar_fg_rect)?;
        sans_bar_fg_mesh.draw(ctx, DrawParams::new().color(Color::rgb(0.0, 0.5, 1.0))); // Blue
    }

    // Draw Player Health (Native Bar Style - Top Right)
    // HP Text
    let mut hp_label = Text::new("HP", state.font.clone());
    hp_label.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(550.0, 20.0))
            .color(Color::WHITE),
    );

    // HP Bar Background (Red)
    let max_bar_width = 100.0;
    let bar_bg_rect = Rectangle::new(590.0, 25.0, max_bar_width, 20.0);
    let bar_bg_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bar_bg_rect)?;
    bar_bg_mesh.draw(ctx, DrawParams::new().color(Color::RED));

    // HP Bar Foreground (Yellow)
    let current_bar_width = (state.player.health / 100.0) * max_bar_width;
    if current_bar_width > 0.0 {
        let bar_fg_rect = Rectangle::new(590.0, 25.0, current_bar_width, 20.0);
        let bar_fg_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, bar_fg_rect)?;
        bar_fg_mesh.draw(ctx, DrawParams::new().color(Color::rgb(1.0, 1.0, 0.0)));
    }

    // HP Numbers
    let hp_text = format!("{}/100", state.player.health as i32);
    let mut t = Text::new(hp_text, state.font.clone());
    t.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(700.0, 20.0))
            .color(Color::WHITE),
    );

    Ok(())
}
