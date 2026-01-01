use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::audio::{Sound, SoundInstance};
use tetra::input::{self, Key};
use tetra::Event;
use tetra::math::{Vec2, Vec3, Mat4};
use tetra::{Context, State};
use rand::Rng;

use crate::defs::{Scene, Language, Direction, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::combat::CombatData;

pub struct GameState {
    pub scene: Scene,
    pub font: Font,
    pub language: Language,
    
    // Boot state
    pub boot_lines: Vec<String>,
    pub boot_text_cache: Vec<Option<(Text, Option<Text>)>>,
    pub current_line: usize,
    pub current_char: usize,
    pub char_timer: f32,
    pub boot_complete_timer: f32,
    
    // Transition
    pub transition_timer: f32,
    pub session_started: bool,
    
    // Login/Menu state
    pub input_buffer: String,
    pub login_error: Option<String>,
    
    // Shell state
    pub shell_input_buffer: String,
    pub shell_history: Vec<(String, Color)>,
    pub shell_cursor_timer: f32,
    pub shell_cursor_visible: bool,
    
    pub cursor_timer: f32,
    pub cursor_visible: bool,
    
    // Timer to prevent immediate skipping of boot sequence
    pub boot_grace_timer: f32,

    // GUI Elements
    pub config_box_mesh: Mesh,
    pub config_shadow_mesh: Mesh,
    
    // Roguelike Game Assets & State
    pub player_pos: Vec2<f32>,
    pub player_texture_front: Option<Texture>,
    pub player_texture_left: Option<Texture>,
    pub player_texture_right: Option<Texture>,
    pub player_direction: Direction,
    pub bg_texture: Option<Texture>,
    pub current_stage: u8,
    pub player_health: f32,
    pub panic_report: Vec<String>,

    // NPC Gaster
    pub npc_gaster_standing: Option<Texture>,
    pub npc_gaster_talking: Option<Texture>,
    pub gaster_pos: Vec2<f32>,
    pub gaster_talking: bool,
    pub gaster_dialogues: Vec<String>,
    pub current_gaster_dialogue: String,

    // NPC Rarity (Stage 2)
    pub rarity_texture: Option<Texture>,
    pub rarity_pos: Vec2<f32>,
    pub rarity_alive: bool,
    pub rarity_stabbed_timer: f32,

    // NPC Eilish (Stage 3)
    pub eilish_texture: Option<Texture>,
    pub eilish_pos: Vec2<f32>,
    pub eilish_talking: bool,
    pub eilish_dialogue_timer: f32,
    pub eilish_current_dialogue: String,

    // MusicBox (Stage 1)
    pub musicbox_texture: Option<Texture>,
    pub musicbox_pos: Vec2<f32>,
    pub music_track: Option<Sound>,
    pub music_instance: Option<SoundInstance>,
    pub music_playing: bool,
    pub disco_color: Color,
    pub disco_timer: f32,

    // Sans & Combat
    #[allow(dead_code)]
    pub sans_texture: Option<Texture>,
    pub sans_combat_texture: Option<Texture>,
    #[allow(dead_code)]
    pub sans_shrug_texture: Option<Texture>,
    pub sans_handshake_texture: Option<Texture>,
    pub sans_pos: Vec2<f32>,
    pub combat_data: CombatData,
    pub heart_texture: Option<Texture>,
    pub bone_texture: Option<Texture>,
    pub fade_alpha: f32,
    pub fade_out: bool,

    // Ayasofya (Lazy Loaded)
    pub ayasofya_giris_texture: Option<Texture>,
    pub ayasofya_ici_texture: Option<Texture>,

    // Loading State
    pub loading_step: usize,
    pub loading_substep: u8,
    pub spinner_timer: f32,
    pub spinner_index: usize,
    pub spinner_direction: i8,
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return vec!["".to_string()];
    }
    chars.chunks(max_chars)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
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
            if std::path::Path::new(path).exists() {
                if let Ok(f) = Font::vector(ctx, path, 16.0) {
                    font = Some(f);
                    break;
                }
            }
        }

        let font = match font {
            Some(f) => f,
            None => panic!("Could not find a suitable font! Please place 'font.ttf' in the 'resources' folder."),
        };

        // Initialize Meshes
        let config_box_mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, 600.0, 400.0),
        )?;

        let config_shadow_mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, 600.0, 400.0),
        )?;

        // Load Roguelike Assets (Moved to Scene::Boot for loading screen)
        // Initialize with None
        let player_texture_front = None;
        let player_texture_left = None;
        let player_texture_right = None;
        let bg_texture = None;
        
        let npc_gaster_standing = None;
        let npc_gaster_talking = None;

        let rarity_texture = None;
        let eilish_texture = None;

        let sans_texture = None;
        let sans_combat_texture = None;
        let sans_shrug_texture = None;
        let sans_handshake_texture = None;
        let heart_texture = None;
        let musicbox_texture = None;
        let music_track = None;

        // Lazy load these later to speed up startup
        let ayasofya_giris_texture = None;
        let ayasofya_ici_texture = None;

        let boot_lines = vec![
                "Starting VibeCoded Linux version 6.9.420...".to_string(),
        ];
        let boot_text_cache = vec![None; boot_lines.len()];

        Ok(GameState {
            scene: Scene::Boot,
            font,
            language: Language::English,
            boot_lines,
            boot_text_cache,
            current_line: 0,
            current_char: 0,
            char_timer: 0.0,
            boot_complete_timer: 0.0,
            transition_timer: 0.0,
            session_started: false,
            
            shell_input_buffer: String::new(),
            shell_history: Vec::new(),
            shell_cursor_timer: 0.0,
            shell_cursor_visible: true,

            input_buffer: String::new(),
            login_error: None,
            cursor_timer: 0.0,
            cursor_visible: true,
            boot_grace_timer: 0.0,
            
            config_box_mesh,
            config_shadow_mesh,
            
            player_pos: Vec2::new(400.0, 300.0),
            player_texture_front,
            player_texture_left,
            player_texture_right,
            player_direction: Direction::Front,
            bg_texture,
            current_stage: 1,
            player_health: 100.0,
            panic_report: Vec::new(),
            
            npc_gaster_standing,
            npc_gaster_talking,
            gaster_pos: Vec2::new(600.0, 300.0),
            gaster_talking: false,
            gaster_dialogues: vec![
                "çakar çakmaz çakan çakmak...".to_string(),
                "Beware the man who speaks in hands...".to_string(),
                "Dark, darker, yet darker...".to_string(),
                "The shadows cutting deeper...".to_string(),
                "Photon readings negative...".to_string(),
                "This next experiment seems very, very interesting...".to_string(),
                "What do you two think?".to_string(),
            ],
            current_gaster_dialogue: String::new(),
            
            rarity_texture,
            rarity_pos: Vec2::new(150.0, 300.0),
            rarity_alive: true,
            rarity_stabbed_timer: 0.0,

            eilish_texture,
            eilish_pos: Vec2::new(150.0, 300.0), // Stage 3, Left side
            eilish_talking: false,
            eilish_dialogue_timer: 0.0,
            eilish_current_dialogue: String::new(),

            musicbox_texture,
            musicbox_pos: Vec2::new(200.0, 300.0),
            music_track,
            music_instance: None,
            music_playing: false,
            disco_color: Color::WHITE,
            disco_timer: 0.0,

            sans_texture,
            sans_combat_texture,
            sans_shrug_texture,
            sans_handshake_texture,
            sans_pos: Vec2::new(600.0, 300.0), // Position in Stage 1
            combat_data: CombatData::new(),
            heart_texture,
            bone_texture: None,
            fade_alpha: 0.0,
            fade_out: false,

            ayasofya_giris_texture,
            ayasofya_ici_texture,
            loading_step: 0,
            loading_substep: 0,
            spinner_timer: 0.0,
            spinner_index: 0,
            spinner_direction: 1,
        })
    }

    pub fn generate_kernel_panic(&mut self) {
        let mut rng = rand::thread_rng();
        let reasons = [
            "Vibe check failed!",
            "Null pointer dereference in vibe_core.ko",
            "Stack overflow in chill_beats_module",
            "Out of memory: Kill process 'stress' (score 420)",
            "CPU 0: Machine Check Exception: Vibe Overload",
            "Fatal exception in interrupt handler: Bad Vibe",
            "Attempted to kill init! (exit code 0xdeadbeef)",
        ];
        let reason = reasons[rng.gen_range(0..reasons.len())];
        
        let mut lines = Vec::new();
        let max_chars = 75;

        let raw_lines = vec![
            format!("[    {:2}.{:06}] Kernel panic - not syncing: {}", rng.gen_range(10..99), rng.gen_range(0..999999), reason),
            format!("[    {:2}.{:06}] CPU: 0 PID: 420 Comm: vibecoded_game Tainted: G        W  O      6.9.420-vibecoded #1", rng.gen_range(10..99), rng.gen_range(0..999999)),
            format!("[    {:2}.{:06}] Hardware name: VibeCoded Virtual Machine/Standard PC (Q35 + ICH9, 2009), BIOS 1.0 12/31/2025", rng.gen_range(10..99), rng.gen_range(0..999999)),
            format!("[    {:2}.{:06}] Call Trace:", rng.gen_range(10..99), rng.gen_range(0..999999)),
            format!("[    {:2}.{:06}]  <TASK>", rng.gen_range(10..99), rng.gen_range(0..999999)),
        ];

        for raw in raw_lines {
            lines.extend(wrap_text(&raw, max_chars));
        }
        
        let symbols = ["dump_stack", "panic", "do_exit", "__handle_mm_fault", "do_group_exit", "get_signal", "arch_do_signal_or_restart", "exit_to_user_mode_prepare", "syscall_exit_to_user_mode", "do_syscall_64", "entry_SYSCALL_64_after_hwframe"];
        
        for sym in symbols {
            let offset = rng.gen_range(0x10..0xff);
            let size = rng.gen_range(0x100..0x500);
            let line = format!("[    {:2}.{:06}]  {}+0x{:x}/0x{:x}", rng.gen_range(10..99), rng.gen_range(0..999999), sym, offset, size);
            lines.extend(wrap_text(&line, max_chars));
        }
        
        let rip_line = format!("[    {:2}.{:06}] RIP: 0033:0x{:x}", rng.gen_range(10..99), rng.gen_range(0..999999), rng.r#gen::<u64>());
        lines.extend(wrap_text(&rip_line, max_chars));

        let task_end = format!("[    {:2}.{:06}]  </TASK>", rng.gen_range(10..99), rng.gen_range(0..999999));
        lines.extend(wrap_text(&task_end, max_chars));

        let end_panic = format!("[    {:2}.{:06}] ---[ end Kernel panic - not syncing: {} ]---", rng.gen_range(10..99), rng.gen_range(0..999999), reason);
        lines.extend(wrap_text(&end_panic, max_chars));

        lines.push("".to_string());
        lines.push("Press ENTER to reboot system...".to_string());
        
        self.panic_report = lines;
    }

    fn reset(&mut self) {
        self.scene = Scene::Boot;
        self.current_line = 0;
        self.current_char = 0;
        self.char_timer = 0.0;
        self.boot_complete_timer = 0.0;
        self.boot_grace_timer = 0.0;
        self.input_buffer.clear();
        self.login_error = None;
        self.shell_history.clear();
        self.shell_input_buffer.clear();
        self.session_started = false;
        
        // Reset Boot State
        self.boot_lines = vec!["Starting VibeCoded Linux version 6.9.420...".to_string()];
        self.boot_text_cache = vec![None];
        self.loading_step = 0;
        self.loading_substep = 0;
        self.spinner_timer = 0.0;
        self.spinner_index = 0;
        self.spinner_direction = 1;
        
        // Reset Game State
        self.player_health = 100.0;
        self.current_stage = 1;
        self.player_pos = Vec2::new(400.0, 300.0);
        self.player_direction = Direction::Front;
    }

    fn logout(&mut self) {
        self.scene = Scene::LoginUsername;
        self.input_buffer.clear();
        self.login_error = None;
        self.shell_history.clear();
        self.shell_input_buffer.clear();
        self.session_started = false;
    }

    fn add_shell_message(&mut self, text: String, color: Color) {
        let max_chars = 75; 
        let lines = wrap_text(&text, max_chars);
        for line in lines {
            self.shell_history.push((line, color));
        }
    }

    pub fn assign_asset(&mut self, index: usize, asset: crate::assets::LoadedAsset) {
        use crate::assets::LoadedAsset;
        match (index, asset) {
            (0, LoadedAsset::Texture(t)) => self.player_texture_front = Some(t),
            (1, LoadedAsset::Texture(t)) => self.player_texture_left = Some(t),
            (2, LoadedAsset::Texture(t)) => self.player_texture_right = Some(t),
            (3, LoadedAsset::Texture(t)) => self.bg_texture = Some(t),
            (4, LoadedAsset::Texture(t)) => self.npc_gaster_standing = Some(t),
            (5, LoadedAsset::Texture(t)) => self.npc_gaster_talking = Some(t),
            (6, LoadedAsset::Texture(t)) => self.rarity_texture = Some(t),
            (7, LoadedAsset::Texture(t)) => self.eilish_texture = Some(t),
            (8, LoadedAsset::Texture(t)) => self.sans_texture = Some(t),
            (9, LoadedAsset::Texture(t)) => self.sans_combat_texture = Some(t),
            (10, LoadedAsset::Texture(t)) => self.sans_shrug_texture = Some(t),
            (11, LoadedAsset::Texture(t)) => self.sans_handshake_texture = Some(t),
            (12, LoadedAsset::Texture(t)) => self.heart_texture = Some(t),
            (13, LoadedAsset::Texture(t)) => self.musicbox_texture = Some(t),
            (14, LoadedAsset::Sound(s)) => self.music_track = Some(s),
            (15, LoadedAsset::Texture(t)) => self.ayasofya_giris_texture = Some(t),
            (16, LoadedAsset::Texture(t)) => self.ayasofya_ici_texture = Some(t),
            (17, LoadedAsset::Texture(t)) => self.bone_texture = Some(t),
            _ => {
                println!("Warning: Asset index {} mismatch or unhandled", index);
            }
        }
    }
}

impl State for GameState {
    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::TextInput { text } => {
                match self.scene {
                    Scene::LoginUsername | Scene::LoginPassword => {
                        // Filter out control characters if any slip through
                        if !text.chars().any(|c: char| c.is_control()) {
                            self.input_buffer.push_str(&text);
                        }
                    }
                    Scene::Menu => {
                        if !text.chars().any(|c: char| c.is_control()) {
                            self.shell_input_buffer.push_str(&text);
                        }
                    }
                    _ => {}
                }
            }
            Event::KeyPressed { key: Key::Backspace } => {
                match self.scene {
                    Scene::LoginUsername | Scene::LoginPassword => {
                        self.input_buffer.pop();
                    }
                    Scene::Menu => {
                        self.shell_input_buffer.pop();
                    }
                    _ => {}
                }
            }
            Event::KeyPressed { key: Key::Enter } => {
                match self.scene {
                    Scene::LoginUsername => {
                        if self.input_buffer == "root" {
                            self.scene = Scene::LoginPassword;
                            self.input_buffer.clear();
                            self.login_error = None;
                        } else {
                            self.login_error = Some("Login incorrect".to_string());
                            self.input_buffer.clear();
                            // Reset to username after a short delay or immediately? 
                            // For simplicity, just clear and stay on username
                        }
                    }
                    Scene::LoginPassword => {
                        // Accept any password
                        self.scene = Scene::Menu;
                        self.input_buffer.clear();
                        
                        // Add welcome message
                        match self.language {
                            Language::English => {
                                self.add_shell_message("Welcome to VibeCoded Linux 1.0 LTS (GNU/Linux 6.9.420-vibecoded x86_64)".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message(" * Documentation:  https://help.vibecoded.com".to_string(), Color::WHITE);
                                self.add_shell_message(" * Management:     https://landscape.vibecoded.com".to_string(), Color::WHITE);
                                self.add_shell_message(" * Support:        https://ubuntu.com/advantage".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("System information as of Fri Dec 27 12:00:00 2025".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("Last login: Fri Dec 27 12:00:00 2025 from 10.0.0.1".to_string(), Color::rgb(0.5, 0.5, 0.5));
                                self.add_shell_message("Type 'help' for a list of commands.".to_string(), Color::rgb(1.0, 1.0, 0.0));
                            }
                            Language::Turkish => {
                                self.add_shell_message("VibeCoded Linux 1.0 LTS'e Hosgeldiniz (GNU/Linux 6.9.420-vibecoded x86_64)".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message(" * Belgelendirme:  https://help.vibecoded.com".to_string(), Color::WHITE);
                                self.add_shell_message(" * Yonetim:        https://landscape.vibecoded.com".to_string(), Color::WHITE);
                                self.add_shell_message(" * Destek:         https://ubuntu.com/advantage".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("Sistem bilgisi: Cum Ara 27 12:00:00 2025".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("Son giris: Cum Ara 27 12:00:00 2025 - 10.0.0.1".to_string(), Color::rgb(0.5, 0.5, 0.5));
                                self.add_shell_message("Komut listesi icin 'help' yazin.".to_string(), Color::rgb(1.0, 1.0, 0.0));
                            }
                        }
                    }
                    Scene::Menu => {
                        let cmd = self.shell_input_buffer.trim().to_string();
                        self.add_shell_message(format!("root@vibecoded:~# {}", cmd), Color::WHITE);
                        
                        match cmd.as_str() {
                            "neofetch" => {
                                let _red = Color::RED;
                                let white = Color::WHITE;
                                
                                // ASCII Heart Art
                                let art = [
                                    "  RRRR   RRRR  ",
                                    " RRRRRR RRRRRR ",
                                    "RRRRRRRRRRRRRRR",
                                    " RRRRRRRRRRRRR ",
                                    "  RRRRRRRRRRR  ",
                                    "    RRRRRRR    ",
                                    "      RRR      ",
                                    "       R       ",
                                ];
                                
                                let info = [
                                    "root@vibecoded",
                                    "--------------",
                                    "OS: VibeCoded Linux",
                                    "Host: Virtual Machine",
                                    "Kernel: 6.9.420-vibecoded",
                                    "Uptime: 1337 mins",
                                    "Shell: vibesh",
                                    "Resolution: 800x600",
                                    "DE: Tetra",
                                    "CPU: Virtual Vibe Processor",
                                    "Memory: 69MB / 420MB",
                                ];

                                for i in 0..std::cmp::max(art.len(), info.len()) {
                                    let art_line = if i < art.len() { art[i] } else { "               " };
                                    let info_text = if i < info.len() { info[i] } else { "" };
                                    
                                    let line = format!("{}  {}", art_line, info_text);
                                    // Use red for the first few lines (header) if we could, but for now just white or red based on line index?
                                    // Let's just use white for readability, or maybe red for the heart lines?
                                    // Since we can't mix colors easily per line, let's just use White.
                                    self.add_shell_message(line, white);
                                }
                                self.add_shell_message("".to_string(), white);
                            }
                            "startx" => {
                                self.scene = Scene::TransitionToDesktop;
                                self.transition_timer = 0.0;
                                self.session_started = true;
                                // Reset game state on start
                                self.player_health = 100.0;
                                self.current_stage = 1;
                                self.player_pos = Vec2::new(400.0, 300.0);
                                self.player_direction = Direction::Front;
                            }
                            "help" => {
                                match self.language {
                                    Language::English => {
                                        self.add_shell_message("GNU bash, version 5.0.17(1)-release (x86_64-pc-linux-gnu)".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                        self.add_shell_message("These shell commands are defined internally.  Type `help' to see this list.".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                        self.add_shell_message("".to_string(), Color::WHITE);
                                        self.add_shell_message("  startx      Start the game".to_string(), Color::GREEN);
                                        self.add_shell_message("  neofetch    Show system information".to_string(), Color::WHITE);
                                        self.add_shell_message("  music       Toggle background music (Disco Mode)".to_string(), Color::WHITE);
                                        self.add_shell_message("  config      Open system configuration".to_string(), Color::WHITE);
                                        self.add_shell_message("  logout      Log out of the system".to_string(), Color::WHITE);
                                        self.add_shell_message("  reboot      Reboot the system".to_string(), Color::WHITE);
                                        self.add_shell_message("  shutdown    Power off the system".to_string(), Color::WHITE);
                                        self.add_shell_message("  clear       Clear the terminal screen".to_string(), Color::WHITE);
                                        self.add_shell_message("  whoami      Print effective userid".to_string(), Color::WHITE);
                                        self.add_shell_message("  uname -a    Print system information".to_string(), Color::WHITE);
                                    }
                                    Language::Turkish => {
                                        self.add_shell_message("GNU bash, surum 5.0.17(1)-release (x86_64-pc-linux-gnu)".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                        self.add_shell_message("Bu kabuk komutlari dahili olarak tanimlanmistir. Listeyi gormek icin `help' yazin.".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                        self.add_shell_message("".to_string(), Color::WHITE);
                                        self.add_shell_message("  startx      Grafik masaustu ortamini baslat (Oyun)".to_string(), Color::GREEN);
                                        self.add_shell_message("  neofetch    Sistem bilgilerini goster".to_string(), Color::WHITE);
                                        self.add_shell_message("  music       Arka plan muzigini ac/kapat (Disko Modu)".to_string(), Color::WHITE);
                                        self.add_shell_message("  config      Sistem yapilandirmasini ac".to_string(), Color::WHITE);
                                        self.add_shell_message("  logout      Sistemden cikis yap".to_string(), Color::WHITE);
                                        self.add_shell_message("  reboot      Sistemi yeniden baslat".to_string(), Color::WHITE);
                                        self.add_shell_message("  shutdown    Sistemi kapat".to_string(), Color::WHITE);
                                        self.add_shell_message("  clear       Terminal ekranini temizle".to_string(), Color::WHITE);
                                        self.add_shell_message("  whoami      Gecerli kullanici kimligini yazdir".to_string(), Color::WHITE);
                                        self.add_shell_message("  uname -a    Sistem bilgilerini yazdir".to_string(), Color::WHITE);
                                    }
                                }
                            }
                            "config" => self.scene = Scene::Config,
                            "logout" | "exit" => self.logout(),
                            "reboot" => self.reset(),
                            "shutdown" => std::process::exit(0),
                            "clear" => self.shell_history.clear(),
                            "whoami" => self.add_shell_message("root".to_string(), Color::WHITE),
                            "uname -a" => self.add_shell_message("Linux vibecoded 6.9.420-vibecoded #1 SMP PREEMPT Fri Dec 30 13:37:00 UTC 2025 x86_64 GNU/Linux".to_string(), Color::WHITE),
                            "music" | "disco" => {
                                if self.scene == Scene::AyasofyaInside {
                                    self.add_shell_message("Music cannot be played in the mosque.".to_string(), Color::RED);
                                } else if self.music_playing {
                                    if let Some(instance) = &mut self.music_instance {
                                        instance.stop();
                                    }
                                    self.music_playing = false;
                                    self.add_shell_message("Music stopped.".to_string(), Color::WHITE);
                                } else {
                                    if let Some(track) = &self.music_track {
                                        if let Ok(instance) = track.play(ctx) {
                                            instance.set_repeating(true);
                                            self.music_instance = Some(instance);
                                            self.music_playing = true;
                                            self.add_shell_message("Music started! Disco mode activated.".to_string(), Color::GREEN);
                                        } else {
                                            self.add_shell_message("Failed to play music.".to_string(), Color::RED);
                                        }
                                    } else {
                                        self.add_shell_message("Music track not loaded.".to_string(), Color::RED);
                                    }
                                }
                            }
                            "" => {}, // Do nothing on empty enter
                            _ => {
                                match self.language {
                                    Language::English => self.add_shell_message(format!("bash: {}: command not found", cmd), Color::RED),
                                    Language::Turkish => self.add_shell_message(format!("bash: {}: komut bulunamadi", cmd), Color::RED),
                                }
                            }
                        }
                        self.shell_input_buffer.clear();
                    }
                    Scene::Config => {
                        // Toggle language on L
                        if input::is_key_pressed(ctx, Key::L) {
                            self.language = match self.language {
                                Language::English => Language::Turkish,
                                Language::Turkish => Language::English,
                            };
                        }
                        // Exit config on Enter for now
                        if input::is_key_pressed(ctx, Key::Enter) {
                            self.scene = Scene::Menu;
                        }
                    }
                    _ => {}
                }
            }
            Event::KeyPressed { key: Key::Escape } => {
                match self.scene {
                    Scene::Desktop => {
                        self.scene = Scene::Menu;
                    }
                    Scene::Config => {
                        self.scene = Scene::Menu;
                    }
                    Scene::Menu => {
                        if self.session_started {
                            self.scene = Scene::Desktop;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scene {
            Scene::Boot => {
                // Ensure cache is synced with lines (handle initial line)
                while self.boot_text_cache.len() < self.boot_lines.len() {
                     self.boot_text_cache.push(None);
                }
                for i in 0..self.boot_lines.len() {
                    if self.boot_text_cache[i].is_none() {
                        let line = &self.boot_lines[i];
                        let cached = if line.starts_with("[  OK  ]") {
                            let ok_part = Text::new("[  OK  ]", self.font.clone());
                            let rest = Text::new(&line[8..], self.font.clone());
                            Some((ok_part, Some(rest)))
                        } else if line.starts_with("[ .... ]") {
                            let wait_part = Text::new("[ .... ]", self.font.clone());
                            let rest = Text::new(&line[8..], self.font.clone());
                            Some((wait_part, Some(rest)))
                        } else if line.starts_with("[ ") && line.len() >= 8 && line.chars().nth(7) == Some(']') {
                            let spinner_part = Text::new(&line[0..8], self.font.clone());
                            let rest = Text::new(&line[8..], self.font.clone());
                            Some((spinner_part, Some(rest)))
                        } else {
                            let text = Text::new(line, self.font.clone());
                            Some((text, None))
                        };
                        self.boot_text_cache[i] = cached;
                    }
                }

                self.char_timer += 1.0;
                
                // Animate Spinner
                if self.loading_substep == 1 {
                    self.spinner_timer += 1.0;
                    if self.spinner_timer > 5.0 { // Speed of animation
                        self.spinner_timer = 0.0;
                        
                        // Update index
                        if self.spinner_direction == 1 {
                            self.spinner_index += 1;
                            if self.spinner_index >= 3 {
                                self.spinner_direction = -1;
                            }
                        } else {
                            if self.spinner_index > 0 {
                                self.spinner_index -= 1;
                            } else {
                                self.spinner_direction = 1;
                                self.spinner_index = 1;
                            }
                        }

                        // Update the last line text
                        if !self.boot_lines.is_empty() {
                            let last_idx = self.boot_lines.len() - 1;
                            let line = &self.boot_lines[last_idx];
                            if line.contains("Loading asset:") {
                                let parts: Vec<&str> = line.split("Loading asset:").collect();
                                if parts.len() > 1 {
                                    let asset_name = parts[1];
                                    let mut spinner_str = String::from("[ ");
                                    for i in 0..4 {
                                        if i == self.spinner_index {
                                            spinner_str.push('*');
                                        } else {
                                            spinner_str.push(' ');
                                        }
                                    }
                                    spinner_str.push_str(" ]");
                                    
                                    self.boot_lines[last_idx] = format!("{} Loading asset:{}", spinner_str, asset_name);
                                    
                                    // Update cache immediately to prevent blinking
                                    // Cache the FULL spinner string in part1 to ensure correct width calculation in draw
                                    let spinner_part = Text::new(&spinner_str, self.font.clone());
                                    let rest = Text::new(&self.boot_lines[last_idx][8..], self.font.clone());
                                    self.boot_text_cache[last_idx] = Some((spinner_part, Some(rest)));
                                }
                            }
                        }
                    }
                }

                // Load one asset every few frames
                if self.char_timer > 3.0 {
                    match self.loading_substep {
                        0 => {
                            // Step 0: Print "Loading..."
                            let asset_count = crate::assets::ASSET_LIST.len();
                            let asset_name = if self.loading_step < asset_count {
                                crate::assets::ASSET_LIST[self.loading_step].name
                            } else if self.loading_step == asset_count {
                                "System"
                            } else {
                                ""
                            };

                            if !asset_name.is_empty() && self.loading_step < asset_count {
                                self.boot_lines.push(format!("[ *    ] Loading asset: {}", asset_name));
                                self.boot_text_cache.push(None);
                                self.current_line = self.boot_lines.len();
                                self.loading_substep = 1;
                                self.char_timer = 0.0; // Reset to allow draw
                                self.spinner_index = 0;
                                self.spinner_direction = 1;
                            } else if self.loading_step == asset_count {
                                // Final step
                                self.boot_lines.push("Welcome to VibeCoded Linux 1.0 LTS (tty1)".to_string());
                                self.boot_text_cache.push(None);
                                self.current_line = self.boot_lines.len();
                                self.loading_step += 1;
                                self.char_timer = 0.0;
                            } else if self.loading_step == asset_count + 1 {
                                self.boot_complete_timer += 1.0;
                                if self.boot_complete_timer > 60.0 {
                                    self.scene = Scene::LoginUsername;
                                }
                            }
                        }
                        1 => {
                            // Step 1: Perform Load and Update Text
                            // Simulate delay for animation to be visible
                            if self.char_timer < 20.0 { // Wait 20 frames to show animation
                                return Ok(());
                            }

                            let mut loaded_text = String::new();
                            if self.loading_step < crate::assets::ASSET_LIST.len() {
                                let asset = crate::assets::load_asset_by_index(ctx, self.loading_step)?;
                                let name = crate::assets::ASSET_LIST[self.loading_step].name;
                                self.assign_asset(self.loading_step, asset);
                                loaded_text = format!("[  OK  ] Loaded asset: {}", name);
                            }

                            if !loaded_text.is_empty() {
                                let last_idx = self.boot_lines.len() - 1;
                                self.boot_lines[last_idx] = loaded_text;
                                
                                // Update cache immediately to prevent blinking
                                let line = &self.boot_lines[last_idx];
                                let ok_part = Text::new("[  OK  ]", self.font.clone());
                                let rest = Text::new(&line[8..], self.font.clone());
                                self.boot_text_cache[last_idx] = Some((ok_part, Some(rest)));
                                
                                self.loading_step += 1;
                                self.loading_substep = 0;
                                self.char_timer = 0.0;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Scene::LoginUsername | Scene::LoginPassword => {
                // Cursor blinking
                self.cursor_timer += 1.0;
                if self.cursor_timer > 30.0 {
                    self.cursor_timer = 0.0;
                    self.cursor_visible = !self.cursor_visible;
                }
            }
            Scene::Menu => {
                // Cursor blinking
                self.shell_cursor_timer += 1.0;
                if self.shell_cursor_timer > 30.0 {
                    self.shell_cursor_timer = 0.0;
                    self.shell_cursor_visible = !self.shell_cursor_visible;
                }
            }
            Scene::TransitionToDesktop => {
                self.transition_timer += 1.0;
                if self.transition_timer > 120.0 { // 2 seconds fade
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
            Scene::Config => {
                // Config logic
            }
            Scene::KernelPanic => {
                if input::is_key_pressed(ctx, Key::Enter) {
                    self.reset();
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
                let mut y = 20.0;
                // Only draw last 25 lines to simulate scrolling if needed, but for now just draw all
                // Or better, draw from current_line - 25
                let start_line = if self.current_line > 25 { self.current_line - 25 } else { 0 };
                
                for i in start_line..self.boot_lines.len() {
                    let line = &self.boot_lines[i];
                    if i < self.current_line {
                        // Use cache
                        if let Some((part1, part2)) = &mut self.boot_text_cache[i] {
                            if line.starts_with("[  OK  ]") {
                                part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::GREEN));
                                if let Some(p2) = part2 {
                                    let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                                    p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                                }
                            } else if line.starts_with("[ ") && line.len() >= 8 && line.chars().nth(7) == Some(']') {
                                // Spinner - Render full block in Yellow to preserve spacing
                                part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::rgb(1.0, 1.0, 0.0)));
                                
                                if let Some(p2) = part2 {
                                    let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                                    p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                                }
                            } else if line.starts_with("[ WARN ]") {
                                part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::rgb(1.0, 0.5, 0.0)));
                                if let Some(p2) = part2 {
                                    let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                                    p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                                }
                            } else if line.starts_with("[ FAILED ]") {
                                part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::RED));
                                if let Some(p2) = part2 {
                                    let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                                    p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                                }
                            } else {
                                part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                            }
                        }
                    } else if i == self.current_line {
                        // Determine parts
                        let (prefix_str, prefix_color, text_content) = if line.starts_with("[  OK  ]") {
                            (Some("[  OK  ]"), Some(Color::GREEN), &line[8..])
                        } else if line.starts_with("[ .... ]") {
                            (Some("[ .... ]"), Some(Color::WHITE), &line[8..])
                        } else if line.starts_with("[ WARN ]") {
                            (Some("[ WARN ]"), Some(Color::rgb(1.0, 0.5, 0.0)), &line[8..])
                        } else if line.starts_with("[ FAILED ]") {
                            (Some("[ FAILED ]"), Some(Color::RED), &line[10..])
                        } else if line.starts_with("[ ") && line.len() >= 8 && line.chars().nth(7) == Some(']') {
                            // This is likely our spinner or a custom status
                            (Some(&line[0..8]), Some(Color::rgb(1.0, 1.0, 0.0)), &line[8..])
                        } else {
                            (None, None, line.as_str())
                        };

                        if self.current_char == 1 {
                            // Show text only, indented
                            let indent = if prefix_str.is_some() {
                                // Approx width of "[  OK  ]" (8 chars) * char width (approx 10px?)
                                // Better to measure a dummy text
                                let mut dummy = Text::new("[  OK  ]", self.font.clone());
                                dummy.get_bounds(ctx).map(|b| b.width).unwrap_or(80.0)
                            } else {
                                0.0
                            };
                            
                            let mut t = Text::new(text_content, self.font.clone());
                            t.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + indent, y)).color(Color::WHITE));
                        } else if self.current_char == 2 {
                            // Show full
                            if let (Some(p_str), Some(p_col)) = (prefix_str, prefix_color) {
                                let mut p_text = Text::new(p_str, self.font.clone());
                                p_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(p_col));
                                
                                // Use fixed width based on standard prefix to prevent jittering during animation
                                let mut dummy = Text::new("[  OK  ]", self.font.clone());
                                let w = dummy.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);

                                let mut t = Text::new(text_content, self.font.clone());
                                t.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                            } else {
                                let mut t = Text::new(text_content, self.font.clone());
                                t.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                            }
                        }
                    }
                    y += 20.0;
                }
            }
            Scene::LoginUsername | Scene::LoginPassword => {
                let mut y = 20.0;
                
                // Draw "vibecoded login: "
                let login_prompt = "user login: ";
                
                if let Scene::LoginUsername = self.scene {
                    let full_text = format!("{}{}{}", login_prompt, self.input_buffer, if self.cursor_visible { "_" } else { "" });
                    let lines = wrap_text(&full_text, 75);
                    for line in lines {
                        let mut text = Text::new(line, self.font.clone());
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                        y += 20.0;
                    }
                } else {
                    // If password state, draw "root" as already entered
                    let full_text = format!("{}root", login_prompt);
                    let mut text = Text::new(full_text, self.font.clone());
                    text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    y += 24.0;
                    
                    let pass_prompt = "Password: ";
                    let masked_input: String = self.input_buffer.chars().map(|_| '*').collect();
                    let full_pass_text = format!("{}{}{}", pass_prompt, masked_input, if self.cursor_visible { "_" } else { "" });
                    
                    let lines = wrap_text(&full_pass_text, 75);
                    for line in lines {
                        let mut text = Text::new(line, self.font.clone());
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                        y += 20.0;
                    }
                }

                if let Some(err) = &self.login_error {
                    y += 24.0;
                    let mut err_text = Text::new(err, self.font.clone());
                    err_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::RED));
                }
            }
            Scene::Menu | Scene::TransitionToDesktop => {
                let t = if let Scene::TransitionToDesktop = self.scene {
                    (self.transition_timer / 120.0).clamp(0.0, 1.0)
                } else {
                    0.0
                };

                if t > 0.0 {
                    let scale = 1.0 + t * t * 2.0; // Strong zoom
                    // Center is 400, 300
                    let trans_to_origin = Mat4::<f32>::translation_3d(Vec3::new(-400.0, -300.0, 0.0));
                    let scaling = Mat4::<f32>::scaling_3d(Vec3::new(scale, scale, 1.0));
                    let trans_back = Mat4::<f32>::translation_3d(Vec3::new(400.0, 300.0, 0.0));
                    
                    let transform = trans_back * scaling * trans_to_origin;
                    graphics::set_transform_matrix(ctx, transform);
                }

                // Draw Shell History
                let mut y = 20.0;
                
                // Simple scrolling: if history is too long, show last N lines
                let max_lines = 28;
                let start_idx = if self.shell_history.len() > max_lines { self.shell_history.len() - max_lines } else { 0 };
                
                for (line, color) in self.shell_history.iter().skip(start_idx) {
                    let mut text = Text::new(line, self.font.clone());
                    text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(*color));
                    y += 20.0;
                }
                
                // Draw Prompt
                let prompt = format!("root@vibecoded:~# {}{}", self.shell_input_buffer, if self.shell_cursor_visible { "_" } else { "" });
                let lines = wrap_text(&prompt, 75);
                for line in lines {
                    let mut prompt_text = Text::new(line, self.font.clone());
                    prompt_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    y += 20.0;
                }
                
                // Transition Effect
                if t > 0.0 {
                    graphics::set_transform_matrix(ctx, Mat4::identity());
                    
                    let alpha = t * t * t; // Cubic ease-in
                    let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
                    fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(1.0, 1.0, 1.0, alpha)));
                }
            }
            Scene::Desktop => {
                crate::scenes::desktop::draw(ctx, self)?;
            }
            Scene::CombatTransition => {
                // Draw Desktop underneath
                crate::scenes::desktop::draw(ctx, self)?;
                
                // Draw fade
                let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
                fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, self.fade_alpha)));
            }
            Scene::Combat => {
                crate::scenes::combat::draw(ctx, self)?;
            }
            Scene::Config => {
                graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.8)); // Blue background like BIOS/Dialog
                
                // Shadow
                self.config_shadow_mesh.draw(ctx, DrawParams::new().position(Vec2::new(110.0, 110.0)).color(Color::BLACK));
                
                // Draw a box
                self.config_box_mesh.draw(ctx, DrawParams::new().position(Vec2::new(100.0, 100.0)).color(Color::rgb(0.7, 0.7, 0.7)));

                let (title_str, content_str) = match self.language {
                    Language::English => (
                        "System Configuration",
                        "Hostname: vibecoded\nKernel: 6.9.420-vibecoded\nMemory: 64MB\nLanguage: English (US) [Press L to Change]\n\n[ OK ] Save & Exit (Enter)"
                    ),
                    Language::Turkish => (
                        "Sistem Yapilandirmasi",
                        "Makine Adi: vibecoded\nCekirdek: 6.9.420-vibecoded\nBellek: 64MB\nDil: Turkce (TR) [Degistirmek icin L]\n\n[ OK ] Kaydet & Cik (Enter)"
                    ),
                };

                let mut title = Text::new(title_str, self.font.clone());
                title.draw(ctx, DrawParams::new().position(Vec2::new(300.0, 120.0)).color(Color::BLACK));
                
                let mut content = Text::new(content_str, self.font.clone());
                content.draw(ctx, DrawParams::new().position(Vec2::new(150.0, 180.0)).color(Color::BLACK));
            }
            Scene::KernelPanic => {
                graphics::clear(ctx, Color::BLACK);
                
                let mut y = 20.0;
                for (i, line) in self.panic_report.iter().enumerate() {
                    let mut text = Text::new(line, self.font.clone());
                    
                    // Make the "Press ENTER" line blink
                    if i == self.panic_report.len() - 1 {
                        // Simple blink using frame count or similar (simulated with random for now or just static)
                        // Actually, let's just make it static for stability, or use a timer if we had one.
                        // We can use `ctx.get_time().as_secs_f32()` if we want.
                        // Let's just keep it white.
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    } else {
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    }
                    y += 20.0;
                }
            }
            Scene::AyasofyaInside => {
                crate::scenes::ayasofya::draw(ctx, self)?;
            }
        }

        Ok(())
    }
}
