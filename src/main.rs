use serde::Deserialize;
use serde_yaml::Value;
use std::env;
use std::fs;
use mission::Mission;
use soldier::Soldier;

mod soldier;
mod mission;

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


    let mut mission_data = vec![];
    match docs[1].get("missionStatistics") {
        Some(missions) => match missions.as_sequence()  {
            Some(missions) => for mission in missions {
                mission_data.push(Mission::from_data(mission))
            }
            None => println!("No missions found in save file"),
        }
        None => println!("No missions found in save file"),
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
                        Some(soldiers) => for soldier_data in soldiers {
                            soldiers_data.push(Soldier::from_data(soldier_data, &mission_data))
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
            Some(soldiers) => for soldier_data in soldiers {
                dead_soldiers_data.push(Soldier::from_data(soldier_data, &mission_data))
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
