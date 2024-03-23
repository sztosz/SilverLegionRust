use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;
use std::env;
use std::fs;

#[derive(Debug, Serialize)]
struct Soldier {
    name: String,
    armor: String,
    id: i64,
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} INPUT.TXT OUTPUT ", args[0]);
        eprintln!("It will create OUTPUT.CSV and OUTPUT_dead.csv files with alive and dead soldiers");
        std::process::exit(1);
    }
    let in_filename = &args[1];
    let out_filename = &args[2];


    let contents = fs::read_to_string(in_filename).expect("Error opening file");

    let mut docs = Vec::new();

    for document in serde_yaml::Deserializer::from_str(&contents) {
        match Value::deserialize(document) {
            Ok(data) => docs.push(data),
            Err(err) => {
                eprintln!("There was an error parsing the SAVE file {}", err);
                std::process::exit(1);
            }
        }
    }

    let mut soldiers_data = vec![];

    match docs[1].get("bases") {
        Some(bases) => match bases.as_sequence() {
            Some(bases) => for base in bases {
                let base_name = base
                    .get("name")
                    .expect("could not find base name!!!")
                    .as_str()
                    .expect("could not find base name!!!");
                match base.get("soldiers") {
                    Some(soldiers) => match soldiers.as_sequence() {
                        Some(soldiers) => for soldier in soldiers {
                            let name = soldier.get("name").unwrap().as_str().unwrap();
                            let armor = soldier.get("armor").unwrap().as_str().unwrap();
                            let id = soldier.get("id").unwrap().as_i64().unwrap();
                            println!("Found alive soldier {} {}", id, name);
                            let missions = soldier.get("missions").unwrap().as_i64().unwrap();
                            let kills = soldier.get("kills").unwrap().as_i64().unwrap();
                            let stuns = soldier.get("stuns").unwrap().as_i64().unwrap();

                            let days_wounded = match soldier.get("diary").unwrap().get("daysWoundedTotal") {
                                Some(val) => val.as_i64().unwrap(),
                                None => 0,
                            };

                            let current_stats = soldier.get("currentStats").unwrap();
                            let initial_stats = soldier.get("initialStats").unwrap();

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

                            let soldier_stats = Soldier {
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
                            };

                            soldiers_data.push(soldier_stats)
                        },
                        None => println!("No bases found in save file"),
                    },
                    None => println!("No soldiers found in base {}", base_name),
                }
            },
            None => println!("No bases found in save file"),
        },
        None => println!("No bases found in save file"),
    };

    let mut dead_soldiers_data = vec![];

    match docs[1].get("deadSoldiers") {
        Some(soldiers) => match soldiers.as_sequence() {
            Some(soldiers) => for soldier in soldiers {
                let name = soldier.get("name").unwrap().as_str().unwrap();
                let armor = soldier.get("armor").unwrap().as_str().unwrap();
                let id = soldier.get("id").unwrap().as_i64().unwrap();
                println!("Found dead soldier {} {}", id, name);
                let missions = soldier.get("missions").unwrap().as_i64().unwrap();
                let kills = soldier.get("kills").unwrap().as_i64().unwrap();
                let stuns = soldier.get("stuns").unwrap().as_i64().unwrap();

                let days_wounded = match soldier.get("diary").unwrap().get("daysWoundedTotal") {
                    Some(val) => val.as_i64().unwrap(),
                    None => 0,
                };

                let current_stats = soldier.get("currentStats").unwrap();
                let initial_stats = soldier.get("initialStats").unwrap();

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




                let soldier_stats = Soldier {
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
                };

                dead_soldiers_data.push(soldier_stats)
            },
            None => println!("No bases found in save file"),
        },
        None => println!("No dead soldiers found"),
    };

    soldiers_data.sort_by(|a, b| a.id.cmp(&b.id));
    dead_soldiers_data.sort_by(|a, b| a.id.cmp(&b.id));

    let mut soldiers_data_wtr = csv::Writer::from_path(format!("{}.csv", out_filename)).unwrap();
    let mut dead_soldiers_data_wtr = csv::Writer::from_path(format!("{}_dead.csv", out_filename)).unwrap();

    for record in soldiers_data {
        soldiers_data_wtr.serialize(record).unwrap();
    }
    for record in dead_soldiers_data {
        dead_soldiers_data_wtr.serialize(record).unwrap();
    }

    // wtr.flush().unwrap();
}
