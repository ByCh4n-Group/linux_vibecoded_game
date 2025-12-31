use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::Event;
use tetra::math::{Vec2, Vec3, Mat4};
use tetra::{Context, ContextBuilder, State};
use rand::Rng;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

// --- Game Structs ---

enum Scene {
    Boot,
    LoginUsername,
    LoginPassword,
    Menu,
    TransitionToDesktop,
    Desktop,
    Config,
    KernelPanic,
}

#[derive(PartialEq, Clone, Copy)]
enum Language {
    English,
    Turkish,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Front,
    Left,
    Right,
}

struct GameState {
    scene: Scene,
    font: Font,
    language: Language,
    
    // Boot state
    boot_lines: Vec<String>,
    boot_text_cache: Vec<Option<(Text, Option<Text>)>>,
    current_line: usize,
    current_char: usize,
    char_timer: f32,
    boot_complete_timer: f32,
    
    // Transition
    transition_timer: f32,
    session_started: bool,
    
    // Login/Menu state
    input_buffer: String,
    login_error: Option<String>,
    
    // Shell state
    shell_input_buffer: String,
    shell_history: Vec<(String, Color)>,
    shell_cursor_timer: f32,
    shell_cursor_visible: bool,
    
    cursor_timer: f32,
    cursor_visible: bool,
    
    // Timer to prevent immediate skipping of boot sequence
    boot_grace_timer: f32,

    // GUI Elements
    config_box_mesh: Mesh,
    config_shadow_mesh: Mesh,
    
    // Roguelike Game Assets & State
    player_pos: Vec2<f32>,
    player_texture_front: Texture,
    player_texture_left: Texture,
    player_texture_right: Texture,
    player_direction: Direction,
    bg_texture: Texture,
    current_stage: u8,
    player_health: f32,
    panic_report: Vec<String>,

    // NPC Gaster
    npc_gaster_standing: Texture,
    npc_gaster_talking: Texture,
    gaster_pos: Vec2<f32>,
    gaster_talking: bool,
    gaster_dialogues: Vec<String>,
    current_gaster_dialogue: String,
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
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
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

        // Load Roguelike Assets
        let player_texture_front = Texture::new(ctx, "./assets/chara1.png")?;
        let player_texture_left = Texture::new(ctx, "./assets/chara_left.png")?;
        let player_texture_right = Texture::new(ctx, "./assets/chara_right.png")?;
        let bg_texture = Texture::new(ctx, "./assets/city_bg.png")?;
        
        let npc_gaster_standing = Texture::new(ctx, "./assets/npc_gaster_standing.png")?;
        let npc_gaster_talking = Texture::new(ctx, "./assets/npc_gaster_talking.png")?;

        let boot_lines = vec![
                "Starting VibeCoded Linux version 6.9.420...".to_string(),
                "Loading kernel modules...".to_string(),
                "[  OK  ] Loaded module: vibe_core".to_string(),
                "[  OK  ] Loaded module: chill_beats".to_string(),
                "[  OK  ] Loaded module: rgb_lighting".to_string(),
                "/dev/nvme0n1p2: clean, 420/1337 files, 69/420 blocks".to_string(),
                "[  OK  ] Started Dispatch Password Requests to Console Directory Watch.".to_string(),
                "[  OK  ] Reached target Local Encrypted Volumes.".to_string(),
                "[  OK  ] Reached target Paths.".to_string(),
                "[  OK  ] Reached target Remote File Systems.".to_string(),
                "[  OK  ] Reached target Slice Units.".to_string(),
                "[  OK  ] Reached target Swap.".to_string(),
                "[  OK  ] Listening on Device-mapper event daemon FIFOs.".to_string(),
                "[  OK  ] Listening on LVM2 poll daemon socket.".to_string(),
                "[  OK  ] Listening on Process Core Dump Socket.".to_string(),
                "[  OK  ] Listening on Journal Socket.".to_string(),
                "[  OK  ] Started Remount Root and Kernel File Systems.".to_string(),
                "[  OK  ] Started Create System Users.".to_string(),
                "[  OK  ] Started Journal Service.".to_string(),
                "         Starting Flush Journal to Persistent Storage...".to_string(),
                "[  OK  ] Finished Flush Journal to Persistent Storage.".to_string(),
                "[  OK  ] Started Network Service.".to_string(),
                "[  OK  ] Reached target Network.".to_string(),
                "         Starting Network Name Resolution...".to_string(),
                "[  OK  ] Started Network Name Resolution.".to_string(),
                "[  OK  ] Reached target Host and Network Name Lookups.".to_string(),
                "[  OK  ] Started User Login Management.".to_string(),
                "[  OK  ] Started Vibe Check Service.".to_string(),
                "[ WARN ] Vibe levels fluctuating slightly.".to_string(),
                "[  OK  ] Stabilized Vibe Levels.".to_string(),
                "[  OK  ] Started Graphical Interface.".to_string(),
                "Welcome to VibeCoded Linux 1.0 LTS (tty1)".to_string(),
                " ".to_string(),
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
        })
    }

    fn generate_kernel_panic(&mut self) {
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
                                self.add_shell_message(" * Documentation:  https://vibecoded.com/help".to_string(), Color::rgb(0.4, 0.4, 1.0));
                                self.add_shell_message(" * Management:     https://vibecoded.com/manage".to_string(), Color::rgb(0.4, 0.4, 1.0));
                                self.add_shell_message(" * Support:        https://vibecoded.com/support".to_string(), Color::rgb(0.4, 0.4, 1.0));
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("System information as of Fri Dec 30 13:37:00 UTC 2025".to_string(), Color::GREEN);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("  System load:  0.00               Processes:             1337".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("  Usage of /:   69.0% of 420GB     Users logged in:       1".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("  Memory usage: 14%                IPv4 address for eth0: 192.168.1.69".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("  Swap usage:   0%".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("0 updates can be applied immediately.".to_string(), Color::GREEN);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("Last login: Fri Dec 27 12:00:00 2025 from 10.0.0.1".to_string(), Color::rgb(0.5, 0.5, 0.5));
                                self.add_shell_message("Type 'help' for a list of commands.".to_string(), Color::rgb(1.0, 1.0, 0.0));
                            }
                            Language::Turkish => {
                                self.add_shell_message("VibeCoded Linux 1.0 LTS'e Hosgeldiniz (GNU/Linux 6.9.420-vibecoded x86_64)".to_string(), Color::WHITE);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message(" * Dokumantasyon:  https://vibecoded.com/help".to_string(), Color::rgb(0.4, 0.4, 1.0));
                                self.add_shell_message(" * Yonetim:        https://vibecoded.com/manage".to_string(), Color::rgb(0.4, 0.4, 1.0));
                                self.add_shell_message(" * Destek:         https://vibecoded.com/support".to_string(), Color::rgb(0.4, 0.4, 1.0));
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("Sistem bilgisi: Cum Ara 30 13:37:00 UTC 2025".to_string(), Color::GREEN);
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("  Sistem yuku:    0.00               Islemler:              1337".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("  Disk kullanimi: %69 / 420GB        Giris yapanlar:        1".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("  Bellek:         %14                eth0 IPv4 adresi:      192.168.1.69".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("  Takas alani:    %0".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                self.add_shell_message("".to_string(), Color::WHITE);
                                self.add_shell_message("0 guncelleme hemen uygulanabilir.".to_string(), Color::GREEN);
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
                                        self.add_shell_message("  startx      Start the graphical desktop environment (Game)".to_string(), Color::GREEN);
                                        self.add_shell_message("  config      Open system configuration".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  logout      Log out of the system".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  reboot      Reboot the system".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  shutdown    Power off the system".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  clear       Clear the terminal screen".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  whoami      Print effective userid".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  uname -a    Print system information".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                    }
                                    Language::Turkish => {
                                        self.add_shell_message("GNU bash, surum 5.0.17(1)-release (x86_64-pc-linux-gnu)".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                        self.add_shell_message("Bu kabuk komutlari dahili olarak tanimlanmistir. Listeyi gormek icin `help' yazin.".to_string(), Color::rgb(0.7, 0.7, 0.7));
                                        self.add_shell_message("".to_string(), Color::WHITE);
                                        self.add_shell_message("  startx      Grafik masaustu ortamini baslat (Oyun)".to_string(), Color::GREEN);
                                        self.add_shell_message("  config      Sistem yapilandirmasini ac".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  logout      Sistemden cikis yap".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  reboot      Sistemi yeniden baslat".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  shutdown    Sistemi kapat".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  clear       Terminal ekranini temizle".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  whoami      Gecerli kullanici kimligini yazdir".to_string(), Color::rgb(0.0, 1.0, 1.0));
                                        self.add_shell_message("  uname -a    Sistem bilgilerini yazdir".to_string(), Color::rgb(0.0, 1.0, 1.0));
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
                self.char_timer += 1.0; 

                if self.current_line < self.boot_lines.len() {
                    // State machine using current_char:
                    // 0: Init/Start
                    // 1: Show Text Only
                    // 2: Show Full (Text + Prefix)
                    
                    if self.current_char == 0 {
                        self.current_char = 1;
                        self.char_timer = 0.0;
                    } else if self.current_char == 1 {
                        // Wait a bit showing only text
                        if self.char_timer > 2.0 { 
                            self.current_char = 2;
                            self.char_timer = 0.0;
                        }
                    } else if self.current_char == 2 {
                        // Wait a bit showing full line before moving on
                        if self.char_timer > 1.0 {
                            // Cache the line
                            let line = &self.boot_lines[self.current_line];
                            let cached = if line.starts_with("[  OK  ]") {
                                let ok_part = Text::new("[  OK  ]", self.font.clone());
                                let rest = Text::new(&line[8..], self.font.clone());
                                Some((ok_part, Some(rest)))
                            } else if line.starts_with("[ WARN ]") {
                                let warn_part = Text::new("[ WARN ]", self.font.clone());
                                let rest = Text::new(&line[8..], self.font.clone());
                                Some((warn_part, Some(rest)))
                            } else if line.starts_with("[ FAILED ]") {
                                let fail_part = Text::new("[ FAILED ]", self.font.clone());
                                let rest = Text::new(&line[10..], self.font.clone());
                                Some((fail_part, Some(rest)))
                            } else {
                                let text = Text::new(line, self.font.clone());
                                Some((text, None))
                            };
                            self.boot_text_cache[self.current_line] = cached;

                            self.current_line += 1;
                            self.current_char = 0;
                            self.char_timer = 0.0;
                        }
                    }
                } else {
                    self.boot_complete_timer += 1.0;
                    if self.boot_complete_timer > 60.0 {
                        self.scene = Scene::LoginUsername;
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
                let speed = 2.0;
                if input::is_key_down(ctx, Key::W) || input::is_key_down(ctx, Key::Up) {
                    self.player_pos.y -= speed;
                    self.player_direction = Direction::Front;
                }
                if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
                    self.player_pos.y += speed;
                    self.player_direction = Direction::Front;
                }
                if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
                    self.player_pos.x -= speed;
                    self.player_direction = Direction::Left;
                }
                if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
                    self.player_pos.x += speed;
                    self.player_direction = Direction::Right;
                }

                // Stage Transition Logic
                if self.player_pos.x > SCREEN_WIDTH as f32 {
                    self.current_stage += 1;
                    if self.current_stage > 3 {
                        self.current_stage = 1;
                    }
                    self.player_pos.x = 0.0;
                } else if self.player_pos.x < 0.0 {
                    // Optional: Go back? For now just clamp or wrap
                    if self.current_stage > 1 {
                        self.current_stage -= 1;
                        self.player_pos.x = SCREEN_WIDTH as f32;
                    } else {
                        self.player_pos.x = 0.0;
                    }
                }

                // Dead Space Logic (Stage 3, Right Side)
                if self.current_stage == 3 && self.player_pos.x > 500.0 {
                    self.player_health -= 0.5; // Damage multiplier
                    
                    if self.player_health <= 0.0 {
                        // Game Over -> Kernel Panic
                        self.generate_kernel_panic();
                        self.scene = Scene::KernelPanic;
                        self.session_started = false;
                    }
                }

                // Gaster Interaction (Stage 2)
                if self.current_stage == 2 {
                    // Simple distance check
                    let dx = self.player_pos.x - self.gaster_pos.x;
                    let dy = self.player_pos.y - self.gaster_pos.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 100.0 {
                        if input::is_key_pressed(ctx, Key::F) {
                            self.gaster_talking = !self.gaster_talking;
                            if self.gaster_talking {
                                let mut rng = rand::thread_rng();
                                let idx = rng.gen_range(0..self.gaster_dialogues.len());
                                self.current_gaster_dialogue = self.gaster_dialogues[idx].clone();
                            }
                        }
                    } else {
                        if self.gaster_talking {
                            self.gaster_talking = false;
                        }
                    }
                }
            }
            Scene::Config => {
                // Config logic
            }
            Scene::KernelPanic => {
                if input::is_key_pressed(ctx, Key::Enter) {
                    self.reset();
                }
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
                        } else if line.starts_with("[ WARN ]") {
                            (Some("[ WARN ]"), Some(Color::rgb(1.0, 0.5, 0.0)), &line[8..])
                        } else if line.starts_with("[ FAILED ]") {
                            (Some("[ FAILED ]"), Some(Color::RED), &line[10..])
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
                                
                                let w = p_text.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
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
                graphics::clear(ctx, Color::BLACK);
                
                // Draw background (scaled to fill screen)
                // Assuming bg is not exactly screen size, we scale it
                let bg_width = self.bg_texture.width() as f32;
                let bg_height = self.bg_texture.height() as f32;
                let scale_x = SCREEN_WIDTH as f32 / bg_width;
                let scale_y = SCREEN_HEIGHT as f32 / bg_height;
                
                self.bg_texture.draw(ctx, DrawParams::new()
                    .position(Vec2::new(0.0, 0.0))
                    .scale(Vec2::new(scale_x, scale_y))
                    .color(if self.current_stage == 1 { Color::WHITE } 
                           else if self.current_stage == 2 { Color::rgb(0.8, 0.8, 1.0) } // Blueish tint
                           else { Color::rgb(1.0, 0.8, 0.8) }) // Reddish tint
                );

                // Draw Gaster (Stage 2)
                if self.current_stage == 2 {
                    let gaster_texture = if self.gaster_talking {
                        &self.npc_gaster_talking
                    } else {
                        &self.npc_gaster_standing
                    };
                    
                    let g_width = gaster_texture.width() as f32;
                    let g_height = gaster_texture.height() as f32;
                    let g_origin = Vec2::new(g_width / 2.0, g_height / 2.0);
                    
                    gaster_texture.draw(ctx, DrawParams::new()
                        .position(self.gaster_pos)
                        .origin(g_origin)
                        .scale(Vec2::new(3.0, 3.0))
                    );

                    // Interaction Prompt
                    let dx = self.player_pos.x - self.gaster_pos.x;
                    let dy = self.player_pos.y - self.gaster_pos.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 100.0 && !self.gaster_talking {
                        let prompt = "Press F to interact";
                        let mut text = Text::new(prompt, self.font.clone());
                        // Simple centering approximation if get_bounds fails or is complex
                        // But we used get_bounds before so it should be fine.
                        // However, get_bounds returns Result, so we need to handle it or unwrap.
                        // The previous code used unwrap_or, let's be safe.
                        let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);
                        
                        text.draw(ctx, DrawParams::new()
                            .position(Vec2::new(self.gaster_pos.x - width / 2.0, self.gaster_pos.y - 80.0))
                            .color(Color::rgb(1.0, 1.0, 0.0))
                        );
                    }

                    // Dialogue Box
                    if self.gaster_talking {
                        // Draw a box at the bottom
                        // We need to create meshes on the fly or cache them. 
                        // Creating meshes in draw loop is not ideal for performance but fine for this scale.
                        if let Ok(box_rect) = Mesh::rectangle(
                            ctx,
                            ShapeStyle::Fill,
                            Rectangle::new(50.0, 450.0, 700.0, 130.0),
                        ) {
                            box_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, 0.8)));
                        }
                        
                        if let Ok(border_rect) = Mesh::rectangle(
                            ctx,
                            ShapeStyle::Stroke(2.0),
                            Rectangle::new(50.0, 450.0, 700.0, 130.0),
                        ) {
                            border_rect.draw(ctx, DrawParams::new().color(Color::WHITE));
                        }

                        let mut text = Text::new(&self.current_gaster_dialogue, self.font.clone());
                        text.draw(ctx, DrawParams::new().position(Vec2::new(70.0, 470.0)).color(Color::WHITE));
                    }
                }

                // Draw Dead Space (Stage 3)
                if self.current_stage == 3 {
                    let dead_space_rect = Mesh::rectangle(
                        ctx,
                        ShapeStyle::Fill,
                        Rectangle::new(500.0, 0.0, 300.0, SCREEN_HEIGHT as f32),
                    )?;
                    dead_space_rect.draw(ctx, DrawParams::new().color(Color::rgba(1.0, 0.0, 0.0, 0.3)));
                }
                
                // Draw player
                let texture = match self.player_direction {
                    Direction::Front => &self.player_texture_front,
                    Direction::Left => &self.player_texture_left,
                    Direction::Right => &self.player_texture_right,
                };
                
                // Center the sprite on player_pos
                let width = texture.width() as f32;
                let height = texture.height() as f32;
                let origin = Vec2::new(width / 2.0, height / 2.0);
                
                // Scale up the character (e.g. 3x)
                texture.draw(ctx, DrawParams::new()
                    .position(self.player_pos)
                    .origin(origin)
                    .scale(Vec2::new(3.0, 3.0))
                );
                
                // Draw Stage Indicator
                let stage_text = format!("Stage: {}/3", self.current_stage);
                let mut text = Text::new(stage_text, self.font.clone());
                text.draw(ctx, DrawParams::new().position(Vec2::new(10.0, 10.0)).color(Color::WHITE));

                // Draw Health Bar (Top Right)
                let bar_width = 150.0;
                let bar_height = 15.0;
                let padding = 10.0;
                let bar_x = SCREEN_WIDTH as f32 - bar_width - padding;
                let bar_y = 10.0;

                let health_bar_bg = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(bar_x, bar_y, bar_width, bar_height))?;
                health_bar_bg.draw(ctx, DrawParams::new().color(Color::rgb(0.2, 0.2, 0.2)));
                
                let health_fill_width = (self.player_health / 100.0) * bar_width;
                if health_fill_width > 0.0 {
                    let health_bar_fg = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(bar_x, bar_y, health_fill_width, bar_height))?;
                    health_bar_fg.draw(ctx, DrawParams::new().color(Color::RED));
                }
                
                let hp_text = format!("HP: {:.0}%", self.player_health);
                let mut hp_display = Text::new(hp_text, self.font.clone());
                // Position text to the left of the bar or below? Let's put it inside/below
                // Or just to the left
                let hp_bounds = hp_display.get_bounds(ctx).unwrap();
                hp_display.draw(ctx, DrawParams::new().position(Vec2::new(bar_x - hp_bounds.width - 10.0, bar_y)).color(Color::WHITE));

                // Draw FPS
                let fps = tetra::time::get_fps(ctx);
                let fps_text = format!("FPS: {:.0}", fps);
                let mut fps_display = Text::new(fps_text, self.font.clone());
                fps_display.draw(ctx, DrawParams::new().position(Vec2::new(10.0, 30.0)).color(Color::rgb(1.0, 1.0, 0.0)));
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
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Linux VibeCoded Game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(false)
        .build()?
        .run(GameState::new)
}
