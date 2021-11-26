use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug)]
struct Army {
    name: String,
    groups: Vec<Group>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Group {
    faction: String,
    id: usize,
    units: usize,
    hp: usize,
    adamage: usize,
    atype: String,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.adamage
    }

    fn calculate_damage(&self, other: &Group) -> usize {
        if self.immunities.contains(&other.atype) {
            return 0;
        }
        if self.weaknesses.contains(&other.atype) {
            2 * other.effective_power()
        } else {
            other.effective_power()
        }
    }

    fn receive_damage(&mut self, damage: usize) -> usize {
        let units = damage / self.hp;
        self.units = self.units.saturating_sub(units);
        units
    }
}

fn parse(input: &str) -> HashMap<String, Vec<Group>> {
    let it = &mut input.lines();

    let mut armies = HashMap::new();

    while let Some(army) = it.next() {
        let name = army
            .trim()
            .trim_end_matches(&[':', ' ', '\n'][..])
            .to_string();
        let mut groups: Vec<Group> = vec![];
        for l in it.take_while(|s| !s.is_empty()) {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(?P<units>\d+) units each with (?P<hp>\d+) hit points(?P<modifiers>.*)with an attack that does (?P<adamage>\d+) (?P<atype>\w+) damage at initiative (?P<initiative>\d+)$").unwrap();
                static ref IMMUNE: Regex = Regex::new(r"immune to ([^;]+)[;\)]").unwrap();
                static ref WEAK: Regex = Regex::new(r"weak to ([^;]+)[;\)]").unwrap();
            }

            let cap = RE.captures(l).unwrap();

            let units: usize = cap.name("units").unwrap().as_str().parse().unwrap();
            let hp: usize = cap.name("hp").unwrap().as_str().parse().unwrap();
            let adamage: usize = cap.name("adamage").unwrap().as_str().parse().unwrap();
            let initiative: usize = cap.name("initiative").unwrap().as_str().parse().unwrap();
            let atype: String = cap.name("atype").unwrap().as_str().to_string();
            let mut immunities = vec![];
            let mut weaknesses = vec![];
            if let Some(mods) = cap.name("modifiers") {
                if let Some(imm) = IMMUNE.captures(mods.as_str()) {
                    immunities = imm
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
                if let Some(weak) = WEAK.captures(mods.as_str()) {
                    weaknesses = weak
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
            }

            debug_assert_eq!(l.contains("weak to"), !weaknesses.is_empty());
            debug_assert_eq!(l.contains("immune to"), !immunities.is_empty());

            groups.push(Group {
                faction: name.clone(),
                id: groups.len() + 1,
                units,
                hp,
                adamage,
                atype,
                initiative,
                weaknesses,
                immunities,
            });
        }
        armies.insert(name, groups);
    }
    armies
}

fn solve1(input: &str) -> usize {
    solve(input, 0).unwrap().0
}

fn solve2(input: &str) -> usize {
    for i in 1.. {
        match solve(input, i) {
            Some((units, winner)) if  winner == "Immune System" => return units,
            _  => {}
        }
    }
    unreachable!();
}

fn solve(input: &str, boost: usize) -> Option<(usize, String)> {
    let armies = &mut parse(input);

    let mut alive: Vec<Group> = armies
        .values()
        .into_iter()
        .flat_map(|a| a.clone())
        .collect();
    alive
        .iter_mut()
        .filter(|g| g.faction == "Immune System")
        .for_each(|g| g.adamage += boost);
    while armies
        .keys()
        .all(|a| alive.iter().any(|g| g.faction == *a))
    {
        alive.retain(|g| g.units > 0);
        alive.sort_by(|a, b| {
            a.effective_power()
                .partial_cmp(&b.effective_power())
                .unwrap()
                .then_with(|| a.initiative.partial_cmp(&b.initiative).unwrap())
        });

        let mut attacks: Vec<(usize, usize, usize)> = Vec::with_capacity(alive.len());
        for (ax, group) in alive.iter().enumerate().rev() {
            let mut target: Option<usize> = None;
            let mut metrics = (0, 0, 0);
            let mut count = 0;
            for (dx, d) in alive.iter().enumerate() {
                if d.faction == group.faction
                    || attacks
                        .iter()
                        .any(|(_, other, _)| other == &dx)
                {
                    continue;
                }
                let new_metrics = (
                    alive[dx].calculate_damage(group),
                    alive[dx].effective_power(),
                    alive[dx].initiative,
                );
                // println!(
                //     "{} group {} would deal defending group {} {} damage",
                //     group.faction, group.id, d.id, new_metrics.0
                // );
                if count == 0 {
                    target = Some(dx);
                    metrics = new_metrics;
                    count = 1;
                } else if new_metrics == metrics {
                    count += 1;
                } else if new_metrics > metrics {
                    target = Some(dx);
                    metrics = new_metrics;
                    count = 1;
                }
            }
            if count == 1 {
                if metrics.0 == 0 {
                    continue;
                }
                attacks.push((ax, target.unwrap(), group.initiative));
            }
        }
        attacks.sort_by_key(|&( _, _, initiative)| Reverse(initiative));
        // println!();
        let mut total_killed = 0;
        for (from, to, _priority) in attacks {
            if alive[to].units == 0 || alive[from].units == 0 {
                continue;
            }
            let attacking = alive.get(from).unwrap();
            let damage = alive.get(to).unwrap().calculate_damage(attacking);
            let killed = alive.get_mut(to).unwrap().receive_damage(damage);
            total_killed += killed;
            // let attacking = alive.get(from).unwrap();
            // let defending = alive.get(to).unwrap();
            // println!(
            //     "{} group {} attacks defending group {} killing {} units",
            //     attacking.faction, attacking.id, defending.id, killed
            // );
        }
        if total_killed == 0 {
            return None
        }
        alive.retain(|g| g.units > 0);
    }
    return Some((
        alive.iter().map(|a| a.units).sum::<usize>(),
        alive
            .first()
            .map(|g| g.faction.clone())
            .unwrap_or_else(|| "".to_string()),
    ));
}

fn main() {
    let input = &std::fs::read_to_string("input").unwrap();
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}
