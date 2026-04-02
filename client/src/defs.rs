use raylib::math::{Rectangle, Vector2};
use shared::constants::MAX_HP;
use tokio::time::Instant;

#[derive(Clone)]
pub struct Player {
    pub health: u32,
    pub pos: Vector2,
    pub vel: Vector2,
    pub mouse: Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: MAX_HP,
            pos: Vector2::new(0.0, 0.0),
            vel: Vector2::new(0.0, 0.0),
            mouse: Vector2::new(0.0, 0.0),
        }
    }
}
pub struct Platform {
    pub rect: Rectangle,
}

#[derive(Clone)]
pub struct TimeStampedPlayer {
    pub player: Player,
    pub time: Instant,
}

impl TimeStampedPlayer {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            time: Instant::now(),
        }
    }
}
pub struct PlayerInterpolater {
    to_shoot: u32,
    mouse_moved: bool,
    pos_moved: bool,
    old: Option<TimeStampedPlayer>,
    new: Option<TimeStampedPlayer>,
}

impl PlayerInterpolater {
    pub fn from(p: Player) -> Self {
        Self {
            to_shoot: 0,
            mouse_moved: false,
            pos_moved: false,
            old: None,
            new: Some(TimeStampedPlayer {
                player: p,
                time: Instant::now(),
            }),
        }
    }
    pub fn inc_shots(&mut self) {
        self.to_shoot += 1;
    }

    pub fn shoot(&mut self) -> bool {
        if self.to_shoot > 0 {
            self.to_shoot -= 1;
            true
        } else {
            false
        }
    }
    pub fn health_change(&mut self, hp: u32) {
        if let Some(some) = &mut self.new {
            some.player.health = hp;
        }
    }
    pub fn update_time(&mut self, player: TimeStampedPlayer) {
        if self.new.is_some() {
            self.old = self.new.take();
        }
        self.pos_moved = false;
        self.mouse_moved = false;
        self.new = Some(player);
    }
    pub fn update_mouse(&mut self, mouse_new: Vector2) {
        if self.new.is_none() {
            self.new = Some(TimeStampedPlayer::new());
            self.pos_moved = false;
            self.mouse_moved = false;
        }
        if self.mouse_moved {
            self.update_time(self.new.clone().unwrap());
            self.pos_moved = false;
            self.mouse_moved = false;
        }
        if let Some(some) = &mut self.new {
            some.player.mouse = mouse_new;
        }
        self.mouse_moved = true;
    }

    pub fn update_pos(&mut self, pos_new: Vector2) {
        if self.new.is_none() {
            self.new = Some(TimeStampedPlayer::new());
            self.pos_moved = false;
            self.mouse_moved = false;
        }
        if self.pos_moved {
            self.update_time(self.new.clone().unwrap());
            self.pos_moved = false;
            self.mouse_moved = false;
        }
        if let Some(some) = &mut self.new {
            some.player.pos = pos_new;
        }
        self.pos_moved = true;
    }

    pub fn interpolate(&self) -> Player {
        match (&self.old, &self.new) {
            (Some(old_p), Some(new_p)) => {
                let now = Instant::now();

                let total_delta_time = (new_p.time - old_p.time).as_secs_f32();

                let time_since_old = (now - old_p.time).as_secs_f32();

                if total_delta_time <= 0.0 {
                    return new_p.player.clone();
                }

                Player {
                    health: new_p.player.health,
                    pos: ((new_p.player.pos - old_p.player.pos) / total_delta_time)
                        * time_since_old
                        + old_p.player.pos,
                    mouse: ((new_p.player.mouse - old_p.player.mouse) / total_delta_time)
                        * time_since_old
                        + old_p.player.mouse,
                    vel: new_p.player.vel.clone(),
                }
            }
            (None, Some(n)) => n.player.clone(),
            _ => Player::new(),
        }
    }
}
