use std::collections::BinaryHeap;

use std::cmp::Ordering;

#[derive(Debug, Clone, Default)]
pub struct Game {
    boss: Boss,
    wizard: Wizard,
    mana_spent: usize,
    turns: usize,
    history: Vec<Spell>,
    effects: Vec<Effect>,
    hard: bool, // For part 2
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.mana_spent == other.mana_spent
    }
}

impl Eq for Game {}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    fn turn(&mut self) -> Vec<Game> {
        self.turns += 1;
        if self.hard && ((self.turns % 2) == 1) {
            self.wizard.hp = self.wizard.hp.saturating_sub(1);
            if self.wizard.hp == 0 {
                return vec![];
            }
        }
        self.wizard.armor = 0;
        self.apply_effects();
        if self.finished() {
            return vec![self.clone()];
        }
        if self.turns % 2 == 0 {
            self.boss.take_turn(self)
        } else {
            self.wizard.take_turn(self)
        }
    }

    fn apply_effects(&mut self) {
        let effects = std::mem::take(&mut self.effects);

        self.effects = effects
            .into_iter()
            .filter_map(|mut e| {
                e.spell.apply(self);
                e.timeout -= 1;
                if e.timeout > 0 {
                    Some(e)
                } else {
                    None
                }
            })
            .collect();
    }

    fn won(&self) -> bool {
        self.boss.hp == 0 && self.wizard.hp > 0
    }

    fn finished(&self) -> bool {
        self.wizard.hp == 0 || self.boss.hp == 0
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Boss {
    hp: usize,
    damage: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wizard {
    hp: usize,
    armor: usize,
    mana: usize,
}

impl Default for Wizard {
    fn default() -> Self {
        Wizard {
            hp: 50,
            mana: 500,
            armor: 0,
        }
    }
}

impl Wizard {
    #[allow(dead_code)]
    fn new(hp: usize, mana: usize) -> Self {
        Self {
            hp,
            mana,
            ..Default::default()
        }
    }
}

pub trait Player {
    fn take_turn(&self, game: &Game) -> Vec<Game>;
    fn inflict_damage(&mut self, damage: usize);
}

impl Player for Wizard {
    fn take_turn(&self, game: &Game) -> Vec<Game> {
        debug_assert!(self.hp > 0);
        let mut opts: Vec<Game> = vec![];
        'spells: for spell in [Missile, Drain, Shield, Poison, Recharge] {
            let cost = match spell {
                Missile => 53,
                Drain => 73,
                Shield => 113,
                Poison => 173,
                Recharge => 229,
            };
            if game.wizard.mana < cost {
                continue 'spells;
            }
            for other_effect in &game.effects {
                if other_effect.spell == spell {
                    continue 'spells;
                }
            }
            let mut new_game = game.clone();
            new_game.mana_spent += cost;
            new_game.wizard.mana -= cost;
            new_game.history.push(spell.clone());
            spell.cast(&mut new_game);
            opts.push(new_game);
        }
        opts
    }

    fn inflict_damage(&mut self, damage: usize) {
        let damage = std::cmp::max(1, damage.saturating_sub(self.armor));
        self.hp = self.hp.saturating_sub(damage);
    }
}

impl Player for Boss {
    fn take_turn(&self, game: &Game) -> Vec<Game> {
        let mut new_game = game.clone();
        new_game.wizard.inflict_damage(self.damage);
        vec![new_game]
    }

    fn inflict_damage(&mut self, damage: usize) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

impl Boss {
    #[allow(dead_code)]
    fn new(hp: usize, damage: usize) -> Self {
        Self { hp, damage }
    }
}

#[derive(Debug, Clone)]
struct Effect {
    spell: Spell,
    timeout: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Spell {
    /// costs 53 mana. It instantly does 4 damage
    Missile,
    /// costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
    Drain,
    /// costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
    Shield,
    /// costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
    Poison,
    /// costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
    Recharge,
}

use Spell::*;

impl Spell {
    fn cast(&self, game: &mut Game) {
        if matches!(self, Missile | Drain) {
            return self.apply(game);
        };
        let duration = match self {
            Recharge => 5,
            _ => 6,
        };
        game.effects.push(Effect {
            spell: self.clone(),
            timeout: duration,
        });
    }

    fn apply(&self, game: &mut Game) {
        match self {
            Missile => game.boss.inflict_damage(4),
            Drain => {
                game.boss.inflict_damage(2);
                game.wizard.hp += 2;
            }
            Shield => {
                game.wizard.armor += 7;
            }
            Poison => {
                game.boss.inflict_damage(3);
            }
            Recharge => {
                game.wizard.mana += 101;
            }
        }
    }
}

pub fn parse(input: &str) -> Game {
    // Example input:
    //
    // Hit Points: 51
    // Damage: 9
    let lines: Vec<&str> = input.lines().collect();
    let boss = Boss {
        hp: lines[0][12..].parse().unwrap(),
        damage: lines[1][8..].parse().unwrap(),
    };
    Game {
        boss,
        ..Default::default()
    }
}

fn solve(wizard: Wizard, boss: Boss, hard: bool) -> Game {
    let mut candidates: BinaryHeap<Game> = BinaryHeap::new();
    candidates.push(Game {
        boss,
        wizard,
        hard,
        ..Default::default()
    });

    while let Some(mut game) = candidates.pop() {
        if game.won() {
            debug_assert!(game.wizard.hp > 0);
            return game;
        }
        if !game.finished() {
            candidates.extend(game.turn().into_iter());
        }
    }
    panic!("no solution found")
}

pub fn part1(game: &Game) -> usize {
    solve(game.wizard.clone(), game.boss.clone(), false).mana_spent
}
pub fn part2(game: &Game) -> usize {
    solve(game.wizard.clone(), game.boss.clone(), true).mana_spent
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let res = solve(Wizard::new(10, 250), Boss::new(13, 8), false);
        dbg!(&res);
        assert_eq!(res.mana_spent, 226);
    }
    #[test]
    fn test_example2() {
        let res = solve(Wizard::new(10, 250), Boss::new(14, 8), false);
        dbg!(&res);
        //Recharge 229
        //Shield
        //Drain
        //Poison
        //Magic Missile

        assert_eq!(res.mana_spent, 229 + 113 + 73 + 173 + 53);
    }
}
