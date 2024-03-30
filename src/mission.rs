use serde_yaml::Value;

#[derive(Debug)]
pub struct Mission {
    id: i64,
    score: i64,
}

impl Mission {
    pub(crate) fn from_data(mission_data: &Value) -> Mission {
        Mission {
            id: mission_data.get("id").unwrap().as_i64().unwrap(),
            score: mission_data.get("score").unwrap().as_i64().unwrap()
        }
    }
}
