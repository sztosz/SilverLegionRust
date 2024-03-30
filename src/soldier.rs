use serde_yaml::Value;
use serde::Serialize;

use crate::mission;

#[derive(Debug, Serialize)]
pub struct Soldier {
    pub(crate) id: i64,
    name: String,
    armor: String,
    missions: i64,
    kills: i64,
    stuns: i64,
    days_wounded: i64,
    bravery: i64,
    initial_bravery: i64,
    firing: i64,
    initial_firing: i64,
    health: i64,
    initial_health: i64,
    mana: i64,
    initial_mana: i64,
    melee: i64,
    initial_melee: i64,
    psi_skill: i64,
    psi_strength: i64,
    initial_psi_skill: i64,
    initial_psi_strength: i64,
    reactions: i64,
    initial_reactions: i64,
    strength: i64,
    initial_strength: i64,
    throwing: i64,
    initial_throwing: i64,
    tu: i64,
    initial_tu: i64,
    stamina: i64,
    initial_stamina: i64,
    score: i64,
}

impl Soldier {
    pub fn from_data(soldier_data: &Value, mission_data: &Vec<super::mission::Mission>) -> Soldier {
        let name = soldier_data.get("name").unwrap().as_str().unwrap();
        let armor = soldier_data.get("armor").unwrap().as_str().unwrap();
        let id = soldier_data.get("id").unwrap().as_i64().unwrap();
        println!("Found alive soldier {} {}", id, name);
        let missions = soldier_data.get("missions").unwrap().as_i64().unwrap();
        let kills = soldier_data.get("kills").unwrap().as_i64().unwrap();
        let stuns = soldier_data.get("stuns").unwrap().as_i64().unwrap();

        let days_wounded = match soldier_data.get("diary").unwrap().get("daysWoundedTotal") {
            Some(val) => val.as_i64().unwrap(),
            None => 0,
        };

        let current_stats = soldier_data.get("currentStats").unwrap();
        let initial_stats = soldier_data.get("initialStats").unwrap();

        let stamina = current_stats.get("stamina").unwrap().as_i64().unwrap();
        let initial_stamina = initial_stats.get("stamina").unwrap().as_i64().unwrap();
        let bravery = current_stats.get("bravery").unwrap().as_i64().unwrap();
        let initial_bravery = initial_stats.get("bravery").unwrap().as_i64().unwrap();
        let firing = current_stats.get("firing").unwrap().as_i64().unwrap();
        let initial_firing = initial_stats.get("firing").unwrap().as_i64().unwrap();
        let health = current_stats.get("health").unwrap().as_i64().unwrap();
        let initial_health = initial_stats.get("health").unwrap().as_i64().unwrap();
        let mana = current_stats.get("mana").unwrap().as_i64().unwrap();
        let initial_mana = initial_stats.get("mana").unwrap().as_i64().unwrap();
        let melee = current_stats.get("melee").unwrap().as_i64().unwrap();
        let initial_melee = initial_stats.get("melee").unwrap().as_i64().unwrap();
        let psi_skill = match current_stats.get("psiSkill") {
            Some(val) => val.as_i64().unwrap(),
            None => 0,
        };
        let initial_psi_skill = match initial_stats.get("psiSkill") {
            Some(val) => val.as_i64().unwrap(),
            None => 0,
        };
        let psi_strength = current_stats.get("psiStrength").unwrap().as_i64().unwrap();
        let initial_psi_strength = initial_stats.get("psiStrength").unwrap().as_i64().unwrap();
        let reactions = current_stats.get("reactions").unwrap().as_i64().unwrap();
        let initial_reactions = initial_stats.get("reactions").unwrap().as_i64().unwrap();
        let strength = current_stats.get("strength").unwrap().as_i64().unwrap();
        let initial_strength = initial_stats.get("strength").unwrap().as_i64().unwrap();
        let throwing = current_stats.get("throwing").unwrap().as_i64().unwrap();
        let initial_throwing = initial_stats.get("throwing").unwrap().as_i64().unwrap();
        let initial_tu = initial_stats.get("tu").unwrap().as_i64().unwrap();
        let tu = current_stats.get("tu").unwrap().as_i64().unwrap();
        let score = match soldier_data.get("diary").unwrap().get("missionIdList") {
            Some(val) => {
                let mut score = 0;
                let mission_ids = val.as_sequence().unwrap();
                for mission in mission_ids {
                    score += mission.as_i64().unwrap();
                }
                score
            }
            None => 0,
        };

        Soldier {
            name: name.to_owned(),
            armor: armor.to_owned(),
            id,
            missions,
            kills,
            stuns,
            days_wounded,
            initial_stamina,
            stamina,
            initial_bravery,
            bravery,
            initial_firing,
            firing,
            initial_health,
            health,
            mana,
            initial_mana,
            melee,
            initial_melee,
            initial_psi_skill,
            psi_skill,
            psi_strength,
            initial_psi_strength,
            reactions,
            initial_reactions,
            initial_strength,
            strength,
            initial_throwing,
            throwing,
            initial_tu,
            tu,
            score,
        }
    }
}
