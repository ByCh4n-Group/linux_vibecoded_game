use rand::Rng;
use tetra::Event;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::{Context, State};

use crate::combat::CombatData;
use crate::defs::{SCREEN_HEIGHT, SCREEN_WIDTH, Scene};
use crate::discord_rpc::DiscordRpc;
use crate::player::PlayerState;
use crate::system::SystemState;
use crate::texts::TextResources;
use crate::world::WorldState;

pub struct GameOverState {
    pub current_stat_index: usize,
    pub stat_pos: Vec2<f32>,
    pub stat_alpha: f32,
    pub stat_fading_in: bool,
    pub selected_option: usize,

    // Dynamic Message
    pub message_text: String,
    pub message_alpha: f32,
    pub message_fading_in: bool,
    pub message_timer: f32,
}

impl GameOverState {
    pub fn new() -> Self {
        Self {
            current_stat_index: 0,
            stat_pos: Vec2::new(50.0, 50.0),
            stat_alpha: 0.0,
            stat_fading_in: true,
            selected_option: 0,

            message_text: String::new(),
            message_alpha: 0.0,
            message_fading_in: true,
            message_timer: 0.0,
        }
    }
}

pub struct GameState {
    pub scene: Scene,
    pub previous_scene: Scene,
    pub font: Font,
    pub texts: TextResources,

    pub system: SystemState,
    pub player: PlayerState,
    pub world: WorldState,

    pub discord: DiscordRpc,

    pub boot_state: crate::scenes::boot::BootState,
    pub menu_state: crate::scenes::menu::MenuState,
    pub game_over_state: GameOverState,

    // Transition
    pub transition_timer: f32,
    pub session_started: bool,

    // Combat
    pub combat_data: CombatData,
    pub heart_texture: Option<Texture>,
    pub bone_texture: Option<Texture>,
    pub fade_alpha: f32,
    pub fade_out: bool,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Try to load a font.
        let font_paths = [
            "resources/font.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/truetype/freefont/FreeMono.ttf",
            "/usr/share/fonts/liberation/LiberationMono-Regular.ttf",
            "C:\\Windows\\Fonts\\consola.ttf", // Just in case
        ];

        let mut font = None;
        for path in &font_paths {
            #[allow(clippy::collapsible_if)]
            if std::path::Path::new(path).exists() {
                if let Ok(f) = Font::vector(ctx, path, 16.0) {
                    font = Some(f);
                    break;
                }
            }
        }

        let font = match font {
            Some(f) => f,
            None => panic!(
                "Could not find a suitable font! Please place 'font.ttf' in the 'resources' folder."
            ),
        };

        let mut boot_state = crate::scenes::boot::BootState::new();
        boot_state
            .boot_lines
            .push("Starting VibeCoded Linux version 6.9.420...".to_string());
        boot_state.boot_text_cache.push(None);

        let mut world = WorldState::new();
        let system = SystemState::new(ctx)?;
        
        // Initialize texts based on language
        let texts = match system.language {
            crate::defs::Language::English => TextResources::new_english(),
            crate::defs::Language::Turkish => TextResources::new_turkish(),
        };
        world.gaster_dialogues = texts.gaster_dialogues.clone();

        let mut discord = DiscordRpc::new("1456952639702040659");
        discord.update_status("Booting up...", "Starting System");

        let mut menu_state = crate::scenes::menu::MenuState::new();

        // If no users, default to Create Save
        if system.users.is_empty() {
            menu_state.selected_index = 1;
        }

        Ok(GameState {
            scene: Scene::Boot,
            previous_scene: Scene::Boot,
            font,
            texts,

            system,
            player: PlayerState::new(),
            world,

            discord,

            boot_state,
            menu_state,
            game_over_state: GameOverState::new(),

            transition_timer: 0.0,
            session_started: false,

            combat_data: CombatData::new(),
            heart_texture: None,
            bone_texture: None,
            fade_alpha: 0.0,
            fade_out: false,
        })
    }

    pub fn assign_asset(&mut self, index: usize, asset: crate::assets::LoadedAsset) {
        use crate::assets::LoadedAsset;
        match (index, asset) {
            (0, LoadedAsset::Texture(t)) => self.player.texture_front = Some(t),
            (1, LoadedAsset::Texture(t)) => self.player.texture_left = Some(t),
            (2, LoadedAsset::Texture(t)) => self.player.texture_right = Some(t),
            (3, LoadedAsset::Texture(t)) => self.world.bg_texture = Some(t),
            (4, LoadedAsset::Texture(t)) => self.world.npc_gaster_standing = Some(t),
            (5, LoadedAsset::Texture(t)) => self.world.npc_gaster_talking = Some(t),
            (6, LoadedAsset::Texture(t)) => self.world.rarity_texture = Some(t),
            (7, LoadedAsset::Texture(t)) => self.world.eilish_texture = Some(t),
            (8, LoadedAsset::Texture(t)) => self.world.sans_texture = Some(t),
            (9, LoadedAsset::Texture(t)) => self.world.sans_combat_texture = Some(t),
            (10, LoadedAsset::Texture(t)) => self.world.sans_shrug_texture = Some(t),
            (11, LoadedAsset::Texture(t)) => self.world.sans_handshake_texture = Some(t),
            (12, LoadedAsset::Texture(t)) => self.heart_texture = Some(t),
            (13, LoadedAsset::Texture(t)) => self.world.musicbox_texture = Some(t),
            (14, LoadedAsset::Sound(s)) => self.world.music_track = Some(s),
            (15, LoadedAsset::Texture(t)) => self.world.ayasofya_giris_texture = Some(t),
            (16, LoadedAsset::Texture(t)) => self.world.ayasofya_ici_texture = Some(t),
            (17, LoadedAsset::Texture(t)) => self.bone_texture = Some(t),
            (18, LoadedAsset::Texture(t)) => self.player.texture_fes = Some(t),
            (19, LoadedAsset::Texture(t)) => self.player.texture_takke = Some(t),
            _ => {
                println!("Warning: Asset index {} mismatch or unhandled", index);
            }
        }
    }

    pub fn is_asset_loaded(&self, index: usize) -> bool {
        match index {
            0 => self.player.texture_front.is_some(),
            1 => self.player.texture_left.is_some(),
            2 => self.player.texture_right.is_some(),
            3 => self.world.bg_texture.is_some(),
            4 => self.world.npc_gaster_standing.is_some(),
            5 => self.world.npc_gaster_talking.is_some(),
            6 => self.world.rarity_texture.is_some(),
            7 => self.world.eilish_texture.is_some(),
            8 => self.world.sans_texture.is_some(),
            9 => self.world.sans_combat_texture.is_some(),
            10 => self.world.sans_shrug_texture.is_some(),
            11 => self.world.sans_handshake_texture.is_some(),
            12 => self.heart_texture.is_some(),
            13 => self.world.musicbox_texture.is_some(),
            14 => self.world.music_track.is_some(),
            15 => self.world.ayasofya_giris_texture.is_some(),
            16 => self.world.ayasofya_ici_texture.is_some(),
            17 => self.bone_texture.is_some(),
            18 => self.player.texture_fes.is_some(),
            19 => self.player.texture_takke.is_some(),
            _ => false,
        }
    }
}

impl State for GameState {
    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        crate::input_handler::handle_event(ctx, self, event);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.scene != self.previous_scene {
            let details = match self.scene {
                Scene::Boot => "Booting up...",
                Scene::Menu => "In Main Menu",
                Scene::TransitionToDesktop => "Loading Desktop...",
                Scene::Desktop => "Exploring Desktop",
                Scene::CombatTransition => "Encounter!",
                Scene::Combat => "Fighting!",
                Scene::KernelPanic => "System Crash!",
                Scene::AyasofyaInside => "Visiting Ayasofya",
            };
            let state = match self.scene {
                Scene::Boot => "System Initialization",
                Scene::Menu => "Selecting Option",
                Scene::TransitionToDesktop => "Please wait...",
                Scene::Desktop => "Wandering around",
                Scene::CombatTransition => "Preparing for battle",
                Scene::Combat => "In Combat",
                Scene::KernelPanic => "Critical Error",
                Scene::AyasofyaInside => "Praying",
            };
            self.discord.update_status(details, state);
            self.previous_scene = self.scene;
        }

        match self.scene {
            Scene::Boot => {
                crate::scenes::boot::update(ctx, self)?;
            }
            Scene::Menu => {
                crate::scenes::menu::update(ctx, self)?;
            }
            Scene::TransitionToDesktop => {
                self.transition_timer += 1.0;
                if self.transition_timer > 120.0 {
                    // 2 seconds fade
                    self.scene = Scene::Desktop;
                }
            }
            Scene::Desktop => {
                crate::scenes::desktop::update(ctx, self)?;
            }
            Scene::CombatTransition => {
                if self.fade_out {
                    self.fade_alpha += 0.02;
                    if self.fade_alpha >= 1.0 {
                        self.fade_alpha = 1.0;
                        self.scene = Scene::Combat;
                        self.fade_out = false;
                        // Reset combat data
                        self.combat_data = CombatData::new();
                    }
                }
            }
            Scene::Combat => {
                crate::scenes::combat::update(ctx, self)?;
            }
            Scene::KernelPanic => {
                // Update Stats Animation
                let mut rng = rand::rng();
                if self.game_over_state.stat_fading_in {
                    self.game_over_state.stat_alpha += 0.02;
                    if self.game_over_state.stat_alpha >= 1.5 {
                        // Go a bit over 1.0 for a pause
                        self.game_over_state.stat_fading_in = false;
                    }
                } else {
                    self.game_over_state.stat_alpha -= 0.02;
                    if self.game_over_state.stat_alpha <= 0.0 {
                        self.game_over_state.stat_fading_in = true;
                        self.game_over_state.current_stat_index =
                            (self.game_over_state.current_stat_index + 1) % 4; // 4 stats

                        // Generate position avoiding the center box
                        // Center Box: X: 150-650, Y: 150-450 (Approx)
                        let pos;
                        loop {
                            let p = Vec2::new(
                                rng.random_range(50.0..SCREEN_WIDTH as f32 - 150.0),
                                rng.random_range(50.0..SCREEN_HEIGHT as f32 - 50.0),
                            );

                            let in_center_x = p.x > 150.0 && p.x < 650.0;
                            let in_center_y = p.y > 150.0 && p.y < 450.0;

                            if !(in_center_x && in_center_y) {
                                pos = p;
                                break;
                            }
                        }
                        self.game_over_state.stat_pos = pos;
                    }
                }

                // Update Dynamic Message Animation
                if self.game_over_state.message_text.is_empty() {
                    // Select message based on stats
                    let teblig = if let Some(u) = &self.system.current_user {
                        u.teblig_count
                    } else {
                        0
                    };
                    let tekfir = if let Some(u) = &self.system.current_user {
                        u.tekfir_count
                    } else {
                        0
                    };

                    println!(
                        "DEBUG: Game Over Stats - Teblig: {}, Tekfir: {}",
                        teblig, tekfir
                    );

                    let messages = if teblig > tekfir {
                        &self.texts.game_over_messages.teblig_high
                    } else if tekfir > teblig {
                        &self.texts.game_over_messages.tekfir_high
                    } else {
                        &self.texts.game_over_messages.equal
                    };

                    self.game_over_state.message_text =
                        messages[rng.random_range(0..messages.len())].clone();
                }

                if self.game_over_state.message_fading_in {
                    self.game_over_state.message_alpha += 0.01;
                    if self.game_over_state.message_alpha >= 1.0 {
                        self.game_over_state.message_fading_in = false;
                        self.game_over_state.message_timer = 2.0; // Hold for 2 seconds
                    }
                } else if self.game_over_state.message_timer > 0.0 {
                    self.game_over_state.message_timer -= 0.016;
                } else {
                    self.game_over_state.message_alpha -= 0.01;
                    if self.game_over_state.message_alpha <= 0.0 {
                        self.game_over_state.message_fading_in = true;
                        self.game_over_state.message_text.clear(); // Pick new message
                    }
                }
            }
            Scene::AyasofyaInside => {
                crate::scenes::ayasofya::update(ctx, self)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        match self.scene {
            Scene::Boot => {
                crate::scenes::boot::draw(ctx, self)?;
            }
            Scene::Menu | Scene::TransitionToDesktop => {
                crate::scenes::menu::draw(ctx, self)?;
            }
            Scene::Desktop => {
                crate::scenes::desktop::draw(ctx, self)?;
            }
            Scene::CombatTransition => {
                // Draw Desktop underneath
                crate::scenes::desktop::draw(ctx, self)?;

                // Draw fade
                let fade_rect = Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
                )
                .unwrap();
                fade_rect.draw(
                    ctx,
                    DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, self.fade_alpha)),
                );
            }
            Scene::Combat => {
                crate::scenes::combat::draw(ctx, self)?;
            }
            Scene::KernelPanic => {
                graphics::clear(ctx, Color::BLACK);

                // Draw Title
                let title = &self.texts.ui.game_over_title;
                let mut title_text = Text::new(title, self.font.clone());
                let title_bounds = title_text.get_bounds(ctx).unwrap();
                let title_scale = 2.0;
                let title_width = title_bounds.width * title_scale;

                title_text.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new((SCREEN_WIDTH as f32 - title_width) / 2.0, 200.0))
                        .color(Color::RED)
                        .scale(Vec2::new(title_scale, title_scale)),
                );

                // Draw Username
                let username = if let Some(user) = &self.system.current_user {
                    format!("{}{}", self.texts.ui.user_label, user.username)
                } else {
                    format!("{}Unknown", self.texts.ui.user_label)
                };
                let mut user_text = Text::new(username, self.font.clone());
                let user_bounds = user_text.get_bounds(ctx).unwrap();
                user_text.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(
                            (SCREEN_WIDTH as f32 - user_bounds.width) / 2.0,
                            260.0,
                        ))
                        .color(Color::WHITE),
                );

                // Draw Dynamic Message
                if !self.game_over_state.message_text.is_empty() {
                    let mut msg_text =
                        Text::new(&self.game_over_state.message_text, self.font.clone());
                    let msg_bounds = msg_text.get_bounds(ctx).unwrap();
                    msg_text.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(
                                (SCREEN_WIDTH as f32 - msg_bounds.width) / 2.0,
                                300.0,
                            ))
                            .color(Color::rgba(
                                1.0,
                                0.0,
                                0.0,
                                self.game_over_state.message_alpha,
                            )), // Red flashing
                    );
                }

                // Draw Stats (Randomly appearing)
                let stats = [
                    format!(
                        "Teblig: {}",
                        if let Some(u) = &self.system.current_user {
                            u.teblig_count
                        } else {
                            0
                        }
                    ),
                    format!(
                        "Cihad: {}",
                        if let Some(u) = &self.system.current_user {
                            u.cihad_count
                        } else {
                            0
                        }
                    ),
                    format!(
                        "Tekfir: {}",
                        if let Some(u) = &self.system.current_user {
                            u.tekfir_count
                        } else {
                            0
                        }
                    ),
                    format!(
                        "Stage: {}",
                        if let Some(u) = &self.system.current_user {
                            u.current_stage
                        } else {
                            1
                        }
                    ),
                ];

                if self.game_over_state.current_stat_index < stats.len() {
                    let mut stat_text = Text::new(
                        &stats[self.game_over_state.current_stat_index],
                        self.font.clone(),
                    );
                    stat_text.draw(
                        ctx,
                        DrawParams::new()
                            .position(self.game_over_state.stat_pos)
                            .color(Color::rgba(
                                1.0,
                                1.0,
                                1.0,
                                self.game_over_state.stat_alpha.min(1.0),
                            )),
                    );
                }

                // Draw Options
                let options = [&self.texts.ui.return_menu, &self.texts.ui.quit_game];
                let total_width = 400.0; // Approximate width for both options
                let start_x = (SCREEN_WIDTH as f32 - total_width) / 2.0;

                let mut x_offset = start_x;
                for (i, opt) in options.iter().enumerate() {
                    let color = if i == self.game_over_state.selected_option {
                        Color::rgb(1.0, 1.0, 0.0) // Yellow
                    } else {
                        Color::rgb(0.5, 0.5, 0.5) // Gray
                    };
                    let mut text = Text::new(*opt, self.font.clone());
                    text.draw(
                        ctx,
                        DrawParams::new()
                            .position(Vec2::new(x_offset, 400.0))
                            .color(color),
                    );
                    x_offset += 250.0; // Spacing
                }
            }
            Scene::AyasofyaInside => {
                crate::scenes::ayasofya::draw(ctx, self)?;
            }
        }

        Ok(())
    }
}
