#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Door(bool),   // open/closed
    Stairs,
    Chest(bool),  // opened/unopened
    Torch,
    Water,
    Grass,
    Tree,
    Exit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EntityKind {
    Goblin,
    Orc,
    Slime,
    Skeleton,
    Troll,
    Boss,
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub pos: (usize, usize),
    pub kind: EntityKind,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub xp_reward: u32,
    pub symbol: char,
    pub name: String,
}

impl Enemy {
    pub fn new_goblin(pos: (usize, usize)) -> Self {
        Self {
            pos,
            kind: EntityKind::Goblin,
            hp: 8,
            max_hp: 8,
            attack: 2,
            xp_reward: 10,
            symbol: 'g',
            name: "Goblin".to_string(),
        }
    }
    pub fn new_orc(pos: (usize, usize)) -> Self {
        Self {
            pos,
            kind: EntityKind::Orc,
            hp: 15,
            max_hp: 15,
            attack: 4,
            xp_reward: 25,
            symbol: 'O',
            name: "Orc".to_string(),
        }
    }
    pub fn new_slime(pos: (usize, usize)) -> Self {
        Self {
            pos,
            kind: EntityKind::Slime,
            hp: 5,
            max_hp: 5,
            attack: 1,
            xp_reward: 7,
            symbol: 's',
            name: "Slime".to_string(),
        }
    }
    pub fn new_skeleton(pos: (usize, usize)) -> Self {
        Self {
            pos,
            kind: EntityKind::Skeleton,
            hp: 12,
            max_hp: 12,
            attack: 3,
            xp_reward: 18,
            symbol: 'k',
            name: "Skeleton".to_string(),
        }
    }
    pub fn new_troll(pos: (usize, usize)) -> Self {
        Self {
            pos,
            kind: EntityKind::Troll,
            hp: 30,
            max_hp: 30,
            attack: 7,
            xp_reward: 50,
            symbol: 'T',
            name: "Troll".to_string(),
        }
    }
    pub fn new_boss(pos: (usize, usize)) -> Self {
        Self {
            pos,
            kind: EntityKind::Boss,
            hp: 60,
            max_hp: 60,
            attack: 10,
            xp_reward: 200,
            symbol: 'D',
            name: "Dragon".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
    HealthPotion,
    Sword,
    Shield,
    Gold(u32),
    Key,
    Scroll,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub pos: (usize, usize),
    pub kind: ItemKind,
    pub symbol: char,
    pub name: String,
}

impl Item {
    pub fn health_potion(pos: (usize, usize)) -> Self {
        Self { pos, kind: ItemKind::HealthPotion, symbol: '!', name: "Health Potion".to_string() }
    }
    pub fn sword(pos: (usize, usize)) -> Self {
        Self { pos, kind: ItemKind::Sword, symbol: '/', name: "Iron Sword".to_string() }
    }
    pub fn shield(pos: (usize, usize)) -> Self {
        Self { pos, kind: ItemKind::Shield, symbol: ')', name: "Wooden Shield".to_string() }
    }
    pub fn gold(pos: (usize, usize), amount: u32) -> Self {
        Self { pos, kind: ItemKind::Gold(amount), symbol: '$', name: format!("Gold ({})", amount) }
    }
    pub fn key(pos: (usize, usize)) -> Self {
        Self { pos, kind: ItemKind::Key, symbol: 'k', name: "Key".to_string() }
    }
    pub fn scroll(pos: (usize, usize)) -> Self {
        Self { pos, kind: ItemKind::Scroll, symbol: '?', name: "Scroll of Knowledge".to_string() }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: (usize, usize),
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub xp: u32,
    pub level: u32,
    pub gold: u32,
    pub has_sword: bool,
    pub has_shield: bool,
    pub keys: u32,
    pub potions: u32,
    pub vim_lessons_learned: Vec<String>,
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            hp: 20,
            max_hp: 20,
            attack: 3,
            defense: 1,
            xp: 0,
            level: 1,
            gold: 0,
            has_sword: false,
            has_shield: false,
            keys: 0,
            potions: 0,
            vim_lessons_learned: vec![],
        }
    }

    pub fn xp_to_next_level(&self) -> u32 {
        self.level * 50
    }

    pub fn try_level_up(&mut self) -> bool {
        if self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level += 1;
            self.max_hp += 5;
            self.hp = (self.hp + 5).min(self.max_hp);
            self.attack += 1;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct VimLesson {
    pub key: &'static str,
    pub description: &'static str,
    pub example: &'static str,
    pub challenge: &'static str,
}

pub fn all_lessons() -> Vec<VimLesson> {
    vec![
        VimLesson { key: "w", description: "Jump WORD forward", example: "Press w to move to next word", challenge: "What key jumps forward one word?" },
        VimLesson { key: "b", description: "Jump WORD backward", example: "b goes back a word", challenge: "What key jumps backward one word?" },
        VimLesson { key: "e", description: "End of WORD", example: "e lands on word end", challenge: "What key moves to end of current word?" },
        VimLesson { key: "0", description: "Start of LINE", example: "0 teleports to row start", challenge: "What key moves to start of line?" },
        VimLesson { key: "$", description: "End of LINE", example: "$ teleports to row end", challenge: "What key moves to end of line?" },
        VimLesson { key: "gg", description: "Go to TOP", example: "gg jumps to top of screen", challenge: "What keys jump to top of file?" },
        VimLesson { key: "G", description: "Go to BOTTOM", example: "G jumps to bottom", challenge: "What key jumps to bottom of file?" },
        VimLesson { key: "f", description: "FIND char on line", example: "f finds a tile ahead", challenge: "What key finds a character ahead on line?" },
        VimLesson { key: "H", description: "High (top of screen)", example: "H moves to top area", challenge: "What key moves to top of screen?" },
        VimLesson { key: "M", description: "Middle of screen", example: "M centers vertically", challenge: "What key moves to middle of screen?" },
        VimLesson { key: "L", description: "Low (bottom of screen)", example: "L jumps to bottom area", challenge: "What key moves to bottom of screen?" },
    ]
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameScreen {
    Title,
    Playing,
    LevelUp,
    LessonPopup(usize), // index into all_lessons
    VimChallenge(usize, String), // lesson index, player input so far
    GameOver,
    Victory,
    Help,
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub message: String,
    pub turn: u32,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![Tile::Wall; width]; height],
            visible: vec![vec![false; width]; height],
            explored: vec![vec![false; width]; height],
        }
    }

    pub fn set(&mut self, y: usize, x: usize, tile: Tile) {
        self.tiles[y][x] = tile;
    }

    pub fn get(&self, y: usize, x: usize) -> &Tile {
        &self.tiles[y][x]
    }

    pub fn is_walkable(&self, y: usize, x: usize) -> bool {
        matches!(
            self.tiles[y][x],
            Tile::Floor | Tile::Door(true) | Tile::Stairs | Tile::Grass | Tile::Torch | Tile::Exit
        )
    }

    pub fn reveal_area(&mut self, cy: usize, cx: usize, radius: usize) {
        let r = radius as i32;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let ny = cy as i32 + dy;
                    let nx = cx as i32 + dx;
                    if ny >= 0 && nx >= 0 && ny < self.height as i32 && nx < self.width as i32 {
                        self.explored[ny as usize][nx as usize] = true;
                    }
                }
            }
        }
    }

    pub fn update_fov(&mut self, py: usize, px: usize, radius: usize) {
        // Reset visibility
        for row in self.visible.iter_mut() {
            for v in row.iter_mut() {
                *v = false;
            }
        }
        // Simple circular FOV
        let r = radius as i32;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let ny = py as i32 + dy;
                    let nx = px as i32 + dx;
                    if ny >= 0 && nx >= 0 && ny < self.height as i32 && nx < self.width as i32 {
                        let (ny, nx) = (ny as usize, nx as usize);
                        // Simple ray cast
                        if self.has_los(py, px, ny, nx) {
                            self.visible[ny][nx] = true;
                            self.explored[ny][nx] = true;
                        }
                    }
                }
            }
        }
    }

    fn has_los(&self, y0: usize, x0: usize, y1: usize, x1: usize) -> bool {
        let (y, x) = (y0 as i32, x0 as i32);
        let (dy, dx) = (y1 as i32 - y, x1 as i32 - x);
        let steps = dy.abs().max(dx.abs());
        if steps == 0 { return true; }
        let (fy, fx) = (dy as f32 / steps as f32, dx as f32 / steps as f32);
        let mut cy = y as f32;
        let mut cx = x as f32;
        for i in 0..steps {
            cy += fy;
            cx += fx;
            let (ty, tx) = (cy.round() as usize, cx.round() as usize);
            if ty >= self.height || tx >= self.width { return false; }
            if i < steps - 1 && matches!(self.tiles[ty][tx], Tile::Wall) {
                return false;
            }
        }
        true
    }
}
