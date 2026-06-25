use crate::types::*;
use crate::levelgen::*;
use rand::Rng;

pub struct GameState {
    pub player: Player,
    pub map: Map,
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub log: Vec<LogEntry>,
    pub turn: u32,
    pub current_level: u32,
    pub screen: GameScreen,
    pub pending_lesson: Option<usize>,
    pub scrolls_read: u32,
    pub camera: (usize, usize), // top-left of viewport
    pub viewport_w: usize,
    pub viewport_h: usize,
}

impl GameState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let level = generate_level(1, &mut rng);
        let player = Player::new(level.player_start);

        let mut state = Self {
            player,
            map: level.map,
            enemies: level.enemies,
            items: level.items,
            log: vec![],
            turn: 0,
            current_level: 1,
            screen: GameScreen::Playing,
            pending_lesson: None,
            scrolls_read: 0,
            camera: (0, 0),
            viewport_w: 50,
            viewport_h: 28,
        };

        state.map.update_fov(state.player.pos.0, state.player.pos.1, 8);
        state.push_log("⚔  Welcome to VimQuest! Use hjkl to move.".to_string());
        state.push_log("📜 Find scrolls to learn powerful Vim motions!".to_string());
        state.center_camera();
        state
    }

    pub fn push_log(&mut self, msg: String) {
        self.log.push(LogEntry { message: msg, turn: self.turn });
        if self.log.len() > 200 {
            self.log.remove(0);
        }
    }

    pub fn center_camera(&mut self) {
        let (py, px) = self.player.pos;
        let half_h = self.viewport_h / 2;
        let half_w = self.viewport_w / 2;
        let cam_y = if py >= half_h { py - half_h } else { 0 };
        let cam_x = if px >= half_w { px - half_w } else { 0 };
        self.camera = (
            cam_y.min(self.map.height.saturating_sub(self.viewport_h)),
            cam_x.min(self.map.width.saturating_sub(self.viewport_w)),
        );
    }

    /// Try to move player by (dy, dx). Returns true if a turn was consumed.
    pub fn move_player(&mut self, dy: i32, dx: i32) -> bool {
        let (py, px) = self.player.pos;
        let ny = py as i32 + dy;
        let nx = px as i32 + dx;

        if ny < 0 || nx < 0 || ny >= self.map.height as i32 || nx >= self.map.width as i32 {
            return false;
        }
        let (ny, nx) = (ny as usize, nx as usize);

        // Check for enemy at target
        if let Some(idx) = self.enemies.iter().position(|e| e.pos == (ny, nx)) {
            self.attack_enemy(idx);
            return true;
        }

        // Check tile walkability
        let tile = self.map.get(ny, nx).clone();
        match tile {
            Tile::Wall => {
                self.push_log("🧱 Blocked by a wall.".to_string());
                false
            }
            Tile::Door(false) => {
                self.map.set(ny, nx, Tile::Door(true));
                self.push_log("🚪 You open the door.".to_string());
                self.player.pos = (ny, nx);
                self.finish_move();
                true
            }
            Tile::Stairs => {
                self.push_log("🪜 You descend the stairs...".to_string());
                self.next_level();
                true
            }
            Tile::Exit => {
                self.screen = GameScreen::Victory;
                true
            }
            _ => {
                self.player.pos = (ny, nx);
                self.pick_up_items();
                self.finish_move();
                true
            }
        }
    }

    fn finish_move(&mut self) {
        self.map.update_fov(self.player.pos.0, self.player.pos.1, 8);
        self.center_camera();
        self.enemies_act();
        self.turn += 1;
    }

    fn attack_enemy(&mut self, idx: usize) {
        let mut rng = rand::thread_rng();
        let player_atk = self.player.attack + if self.player.has_sword { 3 } else { 0 };
        let damage = (player_atk + rng.gen_range(0..3)).max(1);
        let enemy_name = self.enemies[idx].name.clone();
        self.enemies[idx].hp -= damage as i32;
        self.push_log(format!("⚔  You hit {} for {} damage!", enemy_name, damage));

        if self.enemies[idx].hp <= 0 {
            let xp = self.enemies[idx].xp_reward;
            let gold_drop = rng.gen_range(1..8u32);
            self.push_log(format!("💀 {} is defeated! +{} XP, +{} gold", enemy_name, xp, gold_drop));
            self.player.xp += xp;
            self.player.gold += gold_drop;
            self.enemies.remove(idx);

            if self.player.try_level_up() {
                self.screen = GameScreen::LevelUp;
                self.push_log(format!("✨ Level Up! You are now level {}!", self.player.level));
            }
        } else {
            // Enemy counter-attacks
            let def = self.player.defense + if self.player.has_shield { 2 } else { 0 };
            let e_atk = self.enemies[idx].attack;
            let dmg = (e_atk - def + rng.gen_range(0..2)).max(0);
            if dmg > 0 {
                self.player.hp -= dmg;
                self.push_log(format!("🗡  {} hits you for {} damage!", enemy_name, dmg));
            } else {
                self.push_log(format!("🛡  {}'s attack glances off!", enemy_name));
            }
            self.check_player_dead();
        }
        self.finish_move();
    }

    fn enemies_act(&mut self) {
        let player_pos = self.player.pos;
        let mut rng = rand::thread_rng();

        for i in 0..self.enemies.len() {
            let (ey, ex) = self.enemies[i].pos;
            if !self.map.visible[ey][ex] { continue; }

            // Move toward player
            let (py, px) = (player_pos.0 as i32, player_pos.1 as i32);
            let dy = (py - ey as i32).signum();
            let dx = (px - ex as i32).signum();

            let ny = (ey as i32 + dy) as usize;
            let nx = (ex as i32 + dx) as usize;

            // If adjacent to player, attack
            if (ny, nx) == player_pos {
                let def = self.player.defense + if self.player.has_shield { 2 } else { 0 };
                let dmg = (self.enemies[i].attack - def + rng.gen_range(0..2)).max(0);
                let name = self.enemies[i].name.clone();
                if dmg > 0 {
                    self.player.hp -= dmg;
                    self.push_log(format!("🗡  {} attacks for {} damage!", name, dmg));
                }
                self.check_player_dead();
            } else {
                // Try to move toward player
                let can_move = ny < self.map.height
                    && nx < self.map.width
                    && self.map.is_walkable(ny, nx)
                    && !self.enemies.iter().enumerate()
                        .any(|(j, e)| j != i && e.pos == (ny, nx));

                if can_move {
                    self.enemies[i].pos = (ny, nx);
                }
            }
        }
    }

    fn pick_up_items(&mut self) {
        let pos = self.player.pos;
        let picked: Vec<usize> = self.items.iter().enumerate()
            .filter(|(_, item)| item.pos == pos)
            .map(|(i, _)| i)
            .collect();

        for &idx in picked.iter().rev() {
            let item = self.items.remove(idx);
            match &item.kind {
                ItemKind::HealthPotion => {
                    self.player.potions += 1;
                    self.push_log(format!("🧪 Picked up {}. Press 'p' to use.", item.name));
                }
                ItemKind::Sword => {
                    self.player.has_sword = true;
                    self.push_log("⚔  Found an Iron Sword! Attack +3".to_string());
                }
                ItemKind::Shield => {
                    self.player.has_shield = true;
                    self.push_log("🛡  Found a Wooden Shield! Defense +2".to_string());
                }
                ItemKind::Gold(amt) => {
                    self.player.gold += amt;
                    self.push_log(format!("💰 Picked up {} gold! Total: {}", amt, self.player.gold));
                }
                ItemKind::Key => {
                    self.player.keys += 1;
                    self.push_log("🗝  Found a key!".to_string());
                }
                ItemKind::Scroll => {
                    self.scrolls_read += 1;
                    let lessons = all_lessons();
                    let lesson_idx = (self.scrolls_read as usize - 1).min(lessons.len() - 1);
                    let lesson = &lessons[lesson_idx];
                    if !self.player.vim_lessons_learned.contains(&lesson.key.to_string()) {
                        self.player.vim_lessons_learned.push(lesson.key.to_string());
                    }
                    self.push_log(format!(
                        "📜 You read a scroll and learn: '{}' - {}",
                        lesson.key, lesson.description
                    ));
                    self.pending_lesson = Some(lesson_idx);
                    self.screen = GameScreen::LessonPopup(lesson_idx);
                }
            }
        }
    }

    fn check_player_dead(&mut self) {
        if self.player.hp <= 0 {
            self.player.hp = 0;
            self.screen = GameScreen::GameOver;
        }
    }

    pub fn use_potion(&mut self) {
        if self.player.potions > 0 {
            self.player.potions -= 1;
            let heal = 10;
            self.player.hp = (self.player.hp + heal).min(self.player.max_hp);
            self.push_log(format!("🧪 You drink a potion and restore {} HP!", heal));
        } else {
            self.push_log("❌ No potions left!".to_string());
        }
    }

    pub fn next_level(&mut self) {
        self.current_level += 1;
        let mut rng = rand::thread_rng();
        let level = generate_level(self.current_level, &mut rng);

        // Preserve player but update position
        self.player.pos = level.player_start;
        self.map = level.map;
        self.enemies = level.enemies;
        self.items = level.items;

        self.map.update_fov(self.player.pos.0, self.player.pos.1, 8);
        self.center_camera();

        if self.current_level == 5 {
            self.push_log("🐉 You hear a mighty roar... The DRAGON awaits!".to_string());
        } else {
            self.push_log(format!("🪜 Entered dungeon level {}!", self.current_level));
        }
    }

    // VIM JUMP MOTIONS -------------------------------------------------------

    /// 'w' - jump forward one "word" (skip floor tiles to next wall gap or enemy)
    pub fn jump_word_forward(&mut self) -> bool {
        let (py, px) = self.player.pos;
        let mut nx = px + 1;
        while nx < self.map.width - 1 {
            if self.map.is_walkable(py, nx) {
                nx += 1;
            } else {
                break;
            }
        }
        let target = (py, nx.saturating_sub(1));
        self.teleport_to(target)
    }

    /// 'b' - jump backward one "word"
    pub fn jump_word_back(&mut self) -> bool {
        let (py, px) = self.player.pos;
        let mut nx = px.saturating_sub(1);
        while nx > 0 {
            if self.map.is_walkable(py, nx) {
                nx = nx.saturating_sub(1);
            } else {
                break;
            }
        }
        let target = (py, (nx + 1).min(px));
        self.teleport_to(target)
    }

    /// '$' - jump to end of current row (rightmost walkable tile)
    pub fn jump_line_end(&mut self) -> bool {
        let (py, px) = self.player.pos;
        let mut nx = self.map.width - 1;
        while nx > px {
            if self.map.is_walkable(py, nx) { break; }
            nx = nx.saturating_sub(1);
        }
        self.teleport_to((py, nx))
    }

    /// '0' - jump to start of current row
    pub fn jump_line_start(&mut self) -> bool {
        let (py, px) = self.player.pos;
        let mut nx = 0;
        while nx < px {
            if self.map.is_walkable(py, nx) { break; }
            nx += 1;
        }
        self.teleport_to((py, nx))
    }

    /// 'gg' - jump to top of map (topmost walkable tile in same column)
    pub fn jump_top(&mut self) -> bool {
        let (_, px) = self.player.pos;
        let mut ny = 0;
        while ny < self.map.height - 1 {
            if self.map.is_walkable(ny, px) { break; }
            ny += 1;
        }
        self.teleport_to((ny, px))
    }

    /// 'G' - jump to bottom of map
    pub fn jump_bottom(&mut self) -> bool {
        let (_, px) = self.player.pos;
        let mut ny = self.map.height - 1;
        while ny > 0 {
            if self.map.is_walkable(ny, px) { break; }
            ny = ny.saturating_sub(1);
        }
        self.teleport_to((ny, px))
    }

    /// 'H' - High: jump to top area of viewport
    pub fn jump_screen_high(&mut self) -> bool {
        let (_, px) = self.player.pos;
        let cam_y = self.camera.0;
        let mut ny = cam_y + 2;
        while ny < self.map.height - 1 {
            if self.map.is_walkable(ny, px) { break; }
            ny += 1;
        }
        self.teleport_to((ny, px))
    }

    /// 'M' - Middle: jump to middle of viewport
    pub fn jump_screen_mid(&mut self) -> bool {
        let (_, px) = self.player.pos;
        let cam_y = self.camera.0;
        let mid_y = cam_y + self.viewport_h / 2;
        let mut ny = mid_y;
        while ny < self.map.height - 1 {
            if self.map.is_walkable(ny, px) { break; }
            ny += 1;
        }
        self.teleport_to((ny, px))
    }

    /// 'L' - Low: jump to bottom area of viewport
    pub fn jump_screen_low(&mut self) -> bool {
        let (_, px) = self.player.pos;
        let cam_y = self.camera.0;
        let low_y = (cam_y + self.viewport_h).min(self.map.height - 1).saturating_sub(2);
        let mut ny = low_y;
        while ny > 0 {
            if self.map.is_walkable(ny, px) { break; }
            ny = ny.saturating_sub(1);
        }
        self.teleport_to((ny, px))
    }

    /// 'e' - end of word: like w but stops on last walkable before wall
    pub fn jump_word_end(&mut self) -> bool {
        let (py, px) = self.player.pos;
        let mut nx = px + 1;
        while nx < self.map.width.saturating_sub(2) {
            if self.map.is_walkable(py, nx) && !self.map.is_walkable(py, nx + 1) {
                break;
            }
            nx += 1;
        }
        self.teleport_to((py, nx.min(self.map.width - 1)))
    }

    fn teleport_to(&mut self, target: (usize, usize)) -> bool {
        let (ty, tx) = target;
        if ty < self.map.height && tx < self.map.width && self.map.is_walkable(ty, tx) {
            // Check for enemy
            if let Some(idx) = self.enemies.iter().position(|e| e.pos == (ty, tx)) {
                self.attack_enemy(idx);
                return true;
            }
            self.player.pos = (ty, tx);
            self.pick_up_items();
            self.finish_move();
            true
        } else {
            false
        }
    }
}
