
trait HasHealth {
    fn is_dead(&self) -> bool;
    fn remaining_health(&self) -> u32;
    fn take_damage(&self, amount: u32) -> Self;
}

trait CanShoot {
    fn shoot_at<T: HasHealth>(&self, target: &T) -> T;
}

pub struct Soldier {
    health: u32,
    name: &'static str,
    damage: u32,
}

impl Soldier {
    fn with_new_health(&self, new_health: u32) -> Soldier {
        return Soldier {
            health: new_health,
            name: self.name, damage: self.damage};
    }
}

impl HasHealth for Soldier {
    fn is_dead(&self) -> bool {
        self.health <= 0
    }
    fn remaining_health(&self) -> u32 {
        self.health
    }
    fn take_damage(&self, amount: u32) -> Self {
        self.with_new_health(self.health - amount)
    }
}


impl CanShoot for Soldier {
    fn shoot_at<T: HasHealth>(&self, target: &T) -> T {
        target.take_damage(self.damage)
    }
}

pub struct Armored {
    wear: u32,
    armor: u32,
    name: &'static str,
    damage: u32,
}

impl Armored {
    fn with_new_wear(&self, new_wear: u32) -> Armored {
        return Armored {
            wear: new_wear,
            armor: self.armor, name: self.name,
            damage: self.damage};
    }
}

impl HasHealth for Armored {
    fn is_dead(&self) -> bool {
        self.wear <= 0
    }
    fn remaining_health(&self) -> u32 {
        self.wear
    }
    fn take_damage(&self, amount: u32) -> Self {
        self.with_new_wear(
            self.wear -
                (if self.armor > amount { 0 } else { amount - self.armor }))
    }
}

impl CanShoot for Armored {
    fn shoot_at<T: HasHealth>(&self, target: &T) -> T {
        target.take_damage(self.damage)
    }
}

pub fn simulate() {
    let x1 = Soldier {health: 100, name: "X_1", damage: 50};
    let x2 = Soldier {health: 100, name: "X_2", damage: 10};
    let x3 = Soldier {health: 100, name: "X_3", damage: 10};
    let y1 = Soldier {health: 100, name: "Y_1", damage: 10};
    let y2 = Soldier {health: 100, name: "Y_2", damage: 10};
    let y3 = Armored {wear: 200, armor: 5, name: "Y_TANK", damage: 50};

    let y3b: Armored = x1.shoot_at(&y3);
    let x2b: Soldier = y3b.shoot_at(&x2);

    println!("X2 = {}", x2b.health);
    println!("Y3 = {}", y3b.wear);
}
