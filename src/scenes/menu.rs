use crate::defs::{SCREEN_HEIGHT, SCREEN_WIDTH, Scene};
use crate::game_state::GameState;
use rand::Rng;
use tetra::Context;
use tetra::graphics::text::Text;
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::math::Vec2;

#[derive(PartialEq, Clone, Copy)]
pub enum MenuSubState {
    Main,
    SaveSelect,
    CreateSave,
    Settings,
    Credits,
}

pub struct SnowParticle {
    pub pos: Vec2<f32>,
    pub speed: f32,
    pub size: f32,
}

pub struct MenuState {
    pub sub_state: MenuSubState,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub snow_particles: Vec<SnowParticle>,
    pub title_blink_timers: Vec<f32>,
    pub input_buffer: String,
    pub error_message: Option<String>,

    // Chase Animation
    pub chasers: Vec<ChaseEntity>,
}

pub struct ChaseEntity {
    pub pos: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub rotation: f32,
    pub scale: f32,
    pub is_sans: bool,
}

impl MenuState {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut snow_particles = Vec::new();
        for _ in 0..100 {
            snow_particles.push(SnowParticle {
                pos: Vec2::new(
                    rng.random_range(0.0..SCREEN_WIDTH as f32),
                    rng.random_range(0.0..SCREEN_HEIGHT as f32),
                ),
                speed: rng.random_range(0.5..2.0),
                size: rng.random_range(1.0..3.0),
            });
        }

        let title_len = "Gorkitale".len();
        let mut title_blink_timers = Vec::new();
        for _ in 0..title_len {
            title_blink_timers.push(rng.random_range(0.0..1.0));
        }

        Self {
            sub_state: MenuSubState::Main,
            options: vec![
                "Start Game".to_string(),
                "Create Save".to_string(),
                "Select Save".to_string(),
                "Settings".to_string(),
                "Credits".to_string(),
                "Quit Game".to_string(),
            ],
            selected_index: 0,
            snow_particles,
            title_blink_timers,
            input_buffer: String::new(),
            error_message: None,

            chasers: Vec::new(),
        }
    }
}

pub fn update(_ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Update Snow
    let mut rng = rand::rng();
    for particle in &mut state.menu_state.snow_particles {
        particle.pos.y += particle.speed;
        if particle.pos.y > SCREEN_HEIGHT as f32 {
            particle.pos.y = -10.0;
            particle.pos.x = rng.random_range(0.0..SCREEN_WIDTH as f32);
        }
    }

    // Update Title Blink
    for timer in &mut state.menu_state.title_blink_timers {
        *timer -= 0.02; // Speed of blink
        if *timer <= 0.0 {
            *timer = rng.random_range(0.1..1.5); // Random reset
        }
    }

    // Update Chase Animation
    // Spawn new chasers
    if state.menu_state.chasers.len() < 5 && rng.random_bool(0.02) {
        let is_sans = rng.random_bool(0.5);
        let start_side = rng.random_bool(0.5); // true = left, false = right

        let x = if start_side {
            -50.0
        } else {
            SCREEN_WIDTH as f32 + 50.0
        };
        let y = rng.random_range(50.0..SCREEN_HEIGHT as f32 - 50.0);

        let target_x = if start_side {
            SCREEN_WIDTH as f32 + 100.0
        } else {
            -100.0
        };
        let target_y = rng.random_range(50.0..SCREEN_HEIGHT as f32 - 50.0);

        let dx = target_x - x;
        let dy = target_y - y;
        let dist = (dx * dx + dy * dy).sqrt();
        let speed = rng.random_range(3.0..6.0);

        let velocity = Vec2::new((dx / dist) * speed, (dy / dist) * speed);
        let rotation = dy.atan2(dx); // Calculate rotation based on velocity vector
        let scale = rng.random_range(0.7..1.3); // Random scale

        state.menu_state.chasers.push(ChaseEntity {
            pos: Vec2::new(x, y),
            velocity,
            rotation,
            scale,
            is_sans,
        });
    }

    // Update chasers
    state.menu_state.chasers.retain_mut(|chaser| {
        chaser.pos += chaser.velocity;

        // Remove if far off screen
        chaser.pos.x > -100.0
            && chaser.pos.x < SCREEN_WIDTH as f32 + 100.0
            && chaser.pos.y > -100.0
            && chaser.pos.y < SCREEN_HEIGHT as f32 + 100.0
    });

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Draw Snow
    for particle in &state.menu_state.snow_particles {
        let rect = Rectangle::new(particle.pos.x, particle.pos.y, particle.size, particle.size);
        let mesh = tetra::graphics::mesh::Mesh::rectangle(
            ctx,
            tetra::graphics::mesh::ShapeStyle::Fill,
            rect,
        )?;
        mesh.draw(ctx, DrawParams::new().color(Color::WHITE));
    }

    // Draw Chase Animation (Background)
    for chaser in &state.menu_state.chasers {
        let texture = if chaser.is_sans {
            state.world.sans_texture.clone()
        } else if chaser.velocity.x < 0.0 {
            state.player.texture_left.clone()
        } else {
            state.player.texture_right.clone()
        };

        if let Some(tex) = texture {
            let width = tex.width() as f32;
            let height = tex.height() as f32;
            let origin = Vec2::new(width / 2.0, height / 2.0);

            tex.draw(
                ctx,
                DrawParams::new()
                    .position(chaser.pos)
                    .origin(origin)
                    .rotation(chaser.rotation)
                    .scale(Vec2::new(chaser.scale, chaser.scale))
                    .color(Color::rgba(1.0, 1.0, 1.0, 0.5)),
            );
        }
    }

    match state.menu_state.sub_state {
        MenuSubState::Main => draw_main_menu(ctx, state),
        MenuSubState::SaveSelect => draw_save_select(ctx, state),
        MenuSubState::CreateSave => draw_create_save(ctx, state),
        MenuSubState::Settings => draw_settings(ctx, state),
        MenuSubState::Credits => draw_credits(ctx, state),
    }?;

    // Draw Transition Fade
    if state.scene == Scene::TransitionToDesktop {
        let alpha = (state.transition_timer / 120.0).min(1.0);
        let fade_rect = tetra::graphics::mesh::Mesh::rectangle(
            ctx,
            tetra::graphics::mesh::ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
        )?;
        fade_rect.draw(
            ctx,
            DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, alpha)),
        );
    }

    Ok(())
}

fn draw_main_menu(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Draw Title "Gorkitale"
    let title = "Gorkitale";
    let start_x = (SCREEN_WIDTH as f32 / 2.0) - (title.len() as f32 * 20.0); // Approx centering
    let start_y = 100.0;

    for (i, char) in title.chars().enumerate() {
        let timer = state.menu_state.title_blink_timers[i];
        let alpha = if timer > 0.2 { 1.0 } else { 0.3 }; // Blink effect

        let mut text = Text::new(char.to_string(), state.font.clone());
        let pos = Vec2::new(start_x + (i as f32 * 40.0), start_y);
        let color = Color::rgba(1.0, 1.0, 1.0, alpha);

        // Simulate Bold by drawing multiple times with slight offsets
        for offset_x in 0..=1 {
            for offset_y in 0..=1 {
                text.draw(
                    ctx,
                    DrawParams::new()
                        .position(pos + Vec2::new(offset_x as f32, offset_y as f32))
                        .scale(Vec2::new(2.0, 2.0))
                        .color(color),
                );
            }
        }
    }

    // Draw Current User Info
    if let Some(user) = state.system.users.first() {
        let user_text = format!("Current Profile: {}", user.username);
        let mut text = Text::new(user_text, state.font.clone());
        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(20.0, 20.0))
                .color(Color::rgb(0.7, 0.7, 0.7)),
        );
    } else {
        let mut text = Text::new("No Profile Selected", state.font.clone());
        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(20.0, 20.0))
                .color(Color::rgb(0.7, 0.7, 0.7)),
        );
    }

    // Draw Options
    let menu_start_y =
        (SCREEN_HEIGHT as f32 / 2.0) - (state.menu_state.options.len() as f32 * 20.0); // Center vertically
    let menu_start_x = (SCREEN_WIDTH as f32 / 2.0) - 100.0; // Fixed X position for left alignment, slightly offset from center

    let users_exist = !state.system.users.is_empty();

    for (i, option) in state.menu_state.options.iter().enumerate() {
        let is_start_game = i == 0;
        let disabled = is_start_game && !users_exist;

        let color = if disabled {
            Color::rgb(0.3, 0.3, 0.3) // Dark Gray
        } else if i == state.menu_state.selected_index {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };

        let prefix = if i == state.menu_state.selected_index {
            "> "
        } else {
            "  "
        };
        let mut text = Text::new(format!("{}{}", prefix, option), state.font.clone());

        // Left aligned at fixed X
        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(menu_start_x, menu_start_y + (i as f32 * 40.0)))
                .color(color),
        );
    }

    Ok(())
}

fn draw_save_select(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Select Profile", state.font.clone());
    title.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(300.0, 50.0))
            .scale(Vec2::new(1.5, 1.5)),
    );

    let start_y = 150.0;

    // List users
    for (i, user) in state.system.users.iter().enumerate() {
        let color = if i == state.menu_state.selected_index {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };

        let prefix = if i == state.menu_state.selected_index {
            "> "
        } else {
            "  "
        };
        let mut text = Text::new(format!("{}{}", prefix, user.username), state.font.clone());
        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(200.0, start_y + (i as f32 * 30.0)))
                .color(color),
        );
    }

    // "Press Esc to go back"
    let mut hint = Text::new("Press Esc to go back", state.font.clone());
    hint.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(200.0, 500.0))
            .color(Color::rgb(0.5, 0.5, 0.5)),
    );

    Ok(())
}

fn draw_create_save(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Create New Profile", state.font.clone());
    title.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(250.0, 100.0))
            .scale(Vec2::new(1.5, 1.5)),
    );

    let mut prompt = Text::new("Enter Name:", state.font.clone());
    prompt.draw(ctx, DrawParams::new().position(Vec2::new(250.0, 200.0)));

    // Handle text wrapping/scrolling
    let display_text = if state.menu_state.input_buffer.len() > 20 {
        let start = state.menu_state.input_buffer.len() - 20;
        format!("...{}_", &state.menu_state.input_buffer[start..])
    } else {
        format!("{}_", state.menu_state.input_buffer)
    };

    let mut input = Text::new(display_text, state.font.clone());
    input.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(250.0, 240.0))
            .color(Color::rgb(1.0, 1.0, 0.0)),
    );

    if let Some(err) = &state.menu_state.error_message {
        let mut err_text = Text::new(err, state.font.clone());
        err_text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(250.0, 300.0))
                .color(Color::RED),
        );
    }

    let mut hint = Text::new("Press Enter to Confirm, Esc to Cancel", state.font.clone());
    hint.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(200.0, 500.0))
            .color(Color::rgb(0.5, 0.5, 0.5)),
    );

    Ok(())
}

fn draw_settings(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Settings", state.font.clone());
    title.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(300.0, 50.0))
            .scale(Vec2::new(1.5, 1.5)),
    );

    let options = ["Language", "Volume"];
    let start_y = 200.0;

    for (i, opt) in options.iter().enumerate() {
        let color = if i == state.menu_state.selected_index {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };

        let prefix = if i == state.menu_state.selected_index {
            "> "
        } else {
            "  "
        };

        let value = match i {
            0 => format!("{:?}", state.system.language),
            1 => format!("{:.0}%", state.system.volume * 100.0),
            _ => "".to_string(),
        };

        let mut text = Text::new(format!("{}{}: < {} >", prefix, opt, value), state.font.clone());
        text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(200.0, start_y + (i as f32 * 40.0)))
                .color(color),
        );
    }

    let mut hint = Text::new("Press Esc to go back", state.font.clone());
    hint.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(200.0, 500.0))
            .color(Color::rgb(0.5, 0.5, 0.5)),
    );

    Ok(())
}

fn draw_credits(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Credits", state.font.clone());
    title.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(300.0, 50.0))
            .scale(Vec2::new(1.5, 1.5)),
    );

    let credits = [
        "Developed by: VibeCoded",
        "Engine: Tetra (Rust)",
        "Art: ...",
        "Music: ...",
    ];

    for (i, line) in credits.iter().enumerate() {
        let mut text = Text::new(*line, state.font.clone());
        text.draw(
            ctx,
            DrawParams::new().position(Vec2::new(200.0, 150.0 + (i as f32 * 30.0))),
        );
    }

    let mut hint = Text::new("Press Esc to go back", state.font.clone());
    hint.draw(
        ctx,
        DrawParams::new()
            .position(Vec2::new(200.0, 500.0))
            .color(Color::rgb(0.5, 0.5, 0.5)),
    );

    Ok(())
}
