// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: if self.level > 9 { Some(100) } else { None },
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana_cost <= mana {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            } else {
                0
            }
        } else {
            self.health = if self.health >= mana_cost {
                self.health - mana_cost
            } else {
                0
            };
            0
        }
    }
}
