#[test]
fn default() {
    let player = Player::new();
    dbg!(&player);
    assert_eq!(player.damage, 0_u32);
}

#[derive(Debug, Default)]
struct Player {
    health: Health,
    damage: u32,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct Health {
    amount: u32,
    rate: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self { amount: 100, rate: 0.01 }
    }
}
