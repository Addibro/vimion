use crate::types::*;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Room {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Room {
    fn center(&self) -> (usize, usize) {
        (self.y + self.h / 2, self.x + self.w / 2)
    }
    fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.w + 1
            && self.x + self.w + 1 > other.x
            && self.y < other.y + other.h + 1
            && self.y + self.h + 1 > other.y
    }
}

pub struct Level {
    pub map: Map,
    pub player_start: (usize, usize),
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub level_num: u32,
    pub lesson_idx: usize, // Which vim lesson this level teaches
}

pub fn generate_level(level_num: u32, rng: &mut ThreadRng) -> Level {
    let width = 60;
    let height = 35;
    let mut map = Map::new(width, height);

    let mut rooms: Vec<Room> = vec![];
    let max_rooms = 10 + level_num as usize * 2;

    for _ in 0..80 {
        if rooms.len() >= max_rooms { break; }
        let w = rng.gen_range(5..12);
        let h = rng.gen_range(4..9);
        let x = rng.gen_range(1..width - w - 1);
        let y = rng.gen_range(1..height - h - 1);
        let room = Room { x, y, w, h };

        if !rooms.iter().any(|r| r.intersects(&room)) {
            carve_room(&mut map, &room);
            if let Some(prev) = rooms.last() {
                connect_rooms(&mut map, prev.center(), room.center(), rng);
            }
            rooms.push(room);
        }
    }

    let map_center = (height / 2, width / 2);
    let player_room_idx = rooms.iter().enumerate()
        .min_by_key(|(_, r)| {
            let (cy, cx) = r.center();
            let dy = cy as i32 - map_center.0 as i32;
            let dx = cx as i32 - map_center.1 as i32;
            dy * dy + dx * dx
        })
        .map(|(i, _)| i)
        .unwrap_or(0);
    let player_start = rooms[player_room_idx].center();

    // Place stairs/exit in last room
    let last_center = rooms.last().unwrap().center();
    if level_num < 5 {
        map.set(last_center.0, last_center.1, Tile::Stairs);
    } else {
        map.set(last_center.0, last_center.1, Tile::Exit);
    }

    // Add torches — more spread for fog-of-war gameplay
    for room in rooms.iter().skip(1) {
        if rng.gen_bool(0.6) {
            let ty = rng.gen_range(room.y + 1..room.y + room.h - 1);
            let tx = rng.gen_range(room.x + 1..room.x + room.w - 1);
            map.set(ty, tx, Tile::Torch);
        }
    }

    // Doors at some room entrances
    for room in rooms.iter().skip(1) {
        let (_, cx) = room.center();
        if rng.gen_bool(0.3) {
            let door_y = room.y;
            let door_x = cx;
            if door_y > 0 && door_y < height - 1 {
                map.set(door_y, door_x, Tile::Door(false));
            }
        }
    }

    // Spawn enemies
    let mut enemies = vec![];
    for room in rooms.iter().skip(1).rev().skip(1) {
        let count = rng.gen_range(0..=(1 + level_num as usize / 2).min(3));
        for _ in 0..count {
            let ey = rng.gen_range(room.y + 1..room.y + room.h - 1);
            let ex = rng.gen_range(room.x + 1..room.x + room.w - 1);
            let enemy = match level_num {
                1 => if rng.gen_bool(0.7) { Enemy::new_goblin((ey, ex)) } else { Enemy::new_slime((ey, ex)) },
                2 => if rng.gen_bool(0.5) { Enemy::new_goblin((ey, ex)) } else { Enemy::new_orc((ey, ex)) },
                3 => if rng.gen_bool(0.5) { Enemy::new_skeleton((ey, ex)) } else { Enemy::new_orc((ey, ex)) },
                4 => if rng.gen_bool(0.4) { Enemy::new_troll((ey, ex)) } else { Enemy::new_skeleton((ey, ex)) },
                5 => Enemy::new_boss(last_center),
                _ => Enemy::new_orc((ey, ex)),
            };
            if (ey, ex) != player_start {
                enemies.push(enemy);
            }
        }
    }
    // Ensure boss on last level
    if level_num == 5 && !enemies.iter().any(|e| e.kind == EntityKind::Boss) {
        enemies.push(Enemy::new_boss(last_center));
    }

    // Spawn items
    let mut items = vec![];
    for room in rooms.iter().skip(1) {
        if rng.gen_bool(0.5) {
            let iy = rng.gen_range(room.y + 1..room.y + room.h - 1);
            let ix = rng.gen_range(room.x + 1..room.x + room.w - 1);
            let item = match rng.gen_range(0..8u32) {
                0 | 1 => Item::health_potion((iy, ix)),
                2 => Item::gold((iy, ix), rng.gen_range(5..25)),
                3 => Item::sword((iy, ix)),
                4 => Item::shield((iy, ix)),
                5 | 6 | 7 => Item::key((iy, ix)),
                _ => Item::key((iy, ix)),
            };
            items.push(item);
        }
    }
    // One scroll per level, placed in a far room to require exploration
    if rooms.len() > 2 {
        let room = &rooms[rooms.len() - 2]; // second-to-last room (near stairs)
        let iy = room.y + room.h / 2;
        let ix = room.x + room.w / 2;
        items.push(Item::scroll((iy, ix)));
    }

    let lesson_idx = ((level_num - 1) as usize * 3).min(14);

    Level {
        map,
        player_start,
        enemies,
        items,
        level_num,
        lesson_idx,
    }
}

fn carve_room(map: &mut Map, room: &Room) {
    for y in room.y..room.y + room.h {
        for x in room.x..room.x + room.w {
            map.set(y, x, Tile::Floor);
        }
    }
}

fn connect_rooms(map: &mut Map, a: (usize, usize), b: (usize, usize), rng: &mut ThreadRng) {
    let (ay, ax) = a;
    let (by, bx) = b;
    if rng.gen_bool(0.5) {
        carve_h_corridor(map, ax, bx, ay);
        carve_v_corridor(map, ay, by, bx);
    } else {
        carve_v_corridor(map, ay, by, ax);
        carve_h_corridor(map, ax, bx, by);
    }
}

fn carve_h_corridor(map: &mut Map, x1: usize, x2: usize, y: usize) {
    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    for x in start..=end {
        if map.tiles[y][x] == Tile::Wall {
            map.set(y, x, Tile::Floor);
        }
    }
}

fn carve_v_corridor(map: &mut Map, y1: usize, y2: usize, x: usize) {
    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    for y in start..=end {
        if map.tiles[y][x] == Tile::Wall {
            map.set(y, x, Tile::Floor);
        }
    }
}
