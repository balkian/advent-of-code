use lazy_static::lazy_static;
use sscanf::scanf;
use std::cmp::{max, Ordering};

use std::collections::BinaryHeap;

pub fn parse(input: &str) -> Player {
    let (hp, damage, armor) = scanf!(
        input.trim(),
        "Hit Points: {}\nDamage: {}\nArmor: {}",
        usize,
        usize,
        usize
    )
    .unwrap();
    Player {
        hp,
        damage,
        armor,
        items: vec![],
        spent: 0,
    }
}

fn part(boss: &Player, first: bool) -> usize {
    let player = Player::new(100, 0, 0);
    let mut candidates: BinaryHeap<Player> = BinaryHeap::new();
    candidates.push(player);
    let mut last: Option<Player> = None;
    while let Some(next) = candidates.pop() {
        for opt in SHOP.iter() {
            if let Some(cand) = next.equip(opt) {
                candidates.push(cand);
            }
        }
        if next.wins(boss) {
            if first {
                return next.spent;
            }
        } else if !first {
            last = Some(next)
        }
    }
    if first {
        panic!("solution not found");
    }
    last.unwrap().spent
}

pub fn part1(boss: &Player) -> usize {
    part(boss, true)
}

pub fn part2(boss: &Player) -> usize {
    part(boss, false)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Ring,
    Weapon,
    Armor,
}

use Type::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    itype: Type,
    name: &'static str,
    cost: usize,
    damage: usize,
    armor: usize,
}

macro_rules! shopify {
    ($($itype:ident, $name:ident, $cost:expr, $damage:tt, $armor:tt;)*) => {
        vec![
            $(Item{
                itype: $itype,
                name: stringify!($name),
            cost: $cost,
            damage: $damage,
            armor: $armor
        },)*]
    }
}

lazy_static! {
    static ref SHOP: Vec<Item> = shopify![
            Weapon,	dagger,	8,	4,	0;
            Weapon,	shortsword,	10,	5,	0;
            Weapon,	warhammer,	25,	6,	0;
            Weapon,	longsword,	40,	7,	0;
            Weapon,	greataxe,	74,	8,	0;
            Armor,	leather,	13,	0,	1;
            Armor,	chainmail,	31,	0,	2;
            Armor,	splintmail,	53,	0,	3;
            Armor,	bandedmail,	75,	0,	4;
            Armor,	platemail,	102,	0,	5;
            Ring,	damage_1,	25,	1,	0;
            Ring,	damage_2,	50,	2,	0;
            Ring,	damage_3,	100,	3,	0;
            Ring,	defense_1,	20,	0,	1;
            Ring,	defense_2,	40,	0,	2;
            Ring,	defense_3,	80,	0,	3;];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    hp: usize,
    damage: usize,
    armor: usize,
    items: Vec<&'static Item>,
    spent: usize,
}

impl Player {
    fn new(hp: usize, damage: usize, armor: usize) -> Self {
        Player {
            hp,
            damage,
            armor,
            items: vec![],
            spent: 0,
        }
    }

    fn equip(&self, item: &'static Item) -> Option<Self> {
        if self.items.contains(&item) {
            return None;
        }
        let count = self.items.iter().filter(|f| f.itype == item.itype).count();
        let count_weapons = self.items.iter().filter(|f| f.itype == Weapon).count();
        match item.itype {
            Ring if count == 2 => None,
            Armor if count > 0 => None,
            Weapon if count > 0 => None,
            Ring | Armor if count_weapons < 1 => None,
            _ => {
                let mut next = self.clone();
                next.items.push(item);
                next.spent += item.cost;
                next.armor += item.armor;
                next.damage += item.damage;
                Some(next)
            }
        }
    }

    fn wins(&self, other: &Self) -> bool {
        let theirdamage = max(1, other.damage.saturating_sub(self.armor));
        let mut myturns = self.hp / theirdamage;
        if myturns * theirdamage < self.hp {
            myturns += 1;
        }

        let mydamage = max(1, self.damage.saturating_sub(other.armor));
        let mut theirturns = other.hp / max(1, mydamage);
        if theirturns * mydamage < other.hp {
            theirturns += 1;
        }
        myturns >= theirturns
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        other.spent.cmp(&self.spent)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_example() {
    let mut player = Player::new(8, 5, 5);
    let boss = Player::new(12, 7, 2);
    assert_eq!(player.wins(&boss), true);
    player.damage -= 2;
    assert_eq!(player.wins(&boss), false);
}
