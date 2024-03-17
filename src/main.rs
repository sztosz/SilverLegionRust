use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;
use std::env;
use std::fs;

#[derive(Debug, Serialize)]
struct Soldier {
    name: String,
    id: i64,
    missions: i64,
    kills: i64,
    stuns: i64,
    days_wounded: i64,
    bravery: i64,
    firing: i64,
    health: i64,
    mana: i64,
    melee: i64,
    psi_skill: i64,
    psi_strength: i64,
    reactions: i64,
    strength: i64,
    throwing: i64,
    tu: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} INPUT.TXT OUTPUT.CSV", args[0]);
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
            Some(bases) => {
                for base in bases {
                    let base_name = base
                        .get("name")
                        .expect("could not find base name!!!")
                        .as_str()
                        .expect("could not find base name!!!");
                    match base.get("soldiers") {
                        Some(soldiers) => match soldiers.as_sequence() {
                            Some(soldiers) => {
                                for soldier in soldiers {
                                    let name = soldier.get("name").unwrap().as_str().unwrap();
                                    let id = soldier.get("id").unwrap().as_i64().unwrap();
                                    let missions =
                                        soldier.get("missions").unwrap().as_i64().unwrap();
                                    let kills = soldier.get("kills").unwrap().as_i64().unwrap();
                                    let stuns = soldier.get("stuns").unwrap().as_i64().unwrap();

                                    let days_wounded =
                                        match soldier.get("diary").unwrap().get("daysWoundedTotal")
                                        {
                                            Some(val) => val.as_i64().unwrap(),
                                            None => 0,
                                        };

                                    let current_stats = soldier.get("currentStats").unwrap();

                                    let bravery =
                                        current_stats.get("bravery").unwrap().as_i64().unwrap();
                                    let firing =
                                        current_stats.get("firing").unwrap().as_i64().unwrap();
                                    let health =
                                        current_stats.get("health").unwrap().as_i64().unwrap();
                                    let mana = current_stats.get("mana").unwrap().as_i64().unwrap();
                                    let melee =
                                        current_stats.get("melee").unwrap().as_i64().unwrap();
                                    let psi_skill = match current_stats.get("psiSkill") {
                                        Some(val) => val.as_i64().unwrap(),
                                        None => 0,
                                    };
                                    let psi_strength =
                                        current_stats.get("psiStrength").unwrap().as_i64().unwrap();
                                    let reactions =
                                        current_stats.get("reactions").unwrap().as_i64().unwrap();
                                    let strength =
                                        current_stats.get("strength").unwrap().as_i64().unwrap();
                                    let throwing =
                                        current_stats.get("throwing").unwrap().as_i64().unwrap();
                                    let tu = current_stats.get("tu").unwrap().as_i64().unwrap();

                                    let soldier_stats = Soldier {
                                        name: name.to_owned(),
                                        id: id,
                                        missions: missions,
                                        kills: kills,
                                        stuns: stuns,
                                        days_wounded: days_wounded,
                                        bravery: bravery,
                                        firing: firing,
                                        health: health,
                                        mana: mana,
                                        melee: melee,
                                        psi_skill: psi_skill,
                                        psi_strength: psi_strength,
                                        reactions: reactions,
                                        strength: strength,
                                        throwing: throwing,
                                        tu: tu,
                                    };

                                    soldiers_data.push(soldier_stats)
                                }
                            }
                            None => println!("No bases found in save file"),
                        },
                        None => println!("No soldiers found in base {}", base_name),
                    }
                }
            }
            None => println!("No bases found in save file"),
        },
        None => println!("No bases found in save file"),
    };

    match docs[1].get("deadSoldiers") {
        Some(soldiers) => match soldiers.as_sequence() {
            Some(soldiers) => {
                for soldier in soldiers {
                    let name = soldier.get("name").unwrap().as_str().unwrap();
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

                    let bravery = current_stats.get("bravery").unwrap().as_i64().unwrap();
                    let firing = current_stats.get("firing").unwrap().as_i64().unwrap();
                    let health = current_stats.get("health").unwrap().as_i64().unwrap();
                    let mana = current_stats.get("mana").unwrap().as_i64().unwrap();
                    let melee = current_stats.get("melee").unwrap().as_i64().unwrap();
                    let psi_skill = match current_stats.get("psiSkill") {
                        Some(val) => val.as_i64().unwrap(),
                        None => 0,
                    };
                    let psi_strength = current_stats.get("psiStrength").unwrap().as_i64().unwrap();
                    let reactions = current_stats.get("reactions").unwrap().as_i64().unwrap();
                    let strength = current_stats.get("strength").unwrap().as_i64().unwrap();
                    let throwing = current_stats.get("throwing").unwrap().as_i64().unwrap();
                    let tu = current_stats.get("tu").unwrap().as_i64().unwrap();

                    let soldier_stats = Soldier {
                        name: name.to_owned(),
                        id: id,
                        missions: missions,
                        kills: kills,
                        stuns: stuns,
                        days_wounded: days_wounded,
                        bravery: bravery,
                        firing: firing,
                        health: health,
                        mana: mana,
                        melee: melee,
                        psi_skill: psi_skill,
                        psi_strength: psi_strength,
                        reactions: reactions,
                        strength: strength,
                        throwing: throwing,
                        tu: tu,
                    };

                    soldiers_data.push(soldier_stats)
                }
            }
            None => println!("No bases found in save file"),
        },
        None => println!("No dead soldiers found"),
    };

    soldiers_data.sort_by(|a, b| a.id.cmp(&b.id));

    let mut wtr = csv::Writer::from_path(out_filename).unwrap();

    for record in soldiers_data {
        wtr.serialize(record).unwrap();
    }

    // wtr.flush().unwrap();
}
