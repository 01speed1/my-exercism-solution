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
        let mana = if self.level >= 10 { Some(100) } else { None };

        if self.health < 1 {
            Some(Player {
                health: 100,
                mana: mana,
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mana = match self.mana {
            Some(value) => self.mana.unwrap().saturating_sub(mana_cost),
            None => 0,
        };

        if self.level < 10 {
            self.health = self.health.saturating_sub(mana_cost);
            return 0;
        }

        if mana > 0 {
            self.mana = Some(mana);
            return mana_cost * 2;
        } else {
            return 0;
        }
    }
}
