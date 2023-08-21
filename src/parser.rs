use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "Barbarian")]
    barbarian: Class,
    #[serde(rename = "Druid")]
    druid: Class,
    #[serde(rename = "Necromancer")]
    necromancer: Class,
    #[serde(rename = "Rogue")]
    rogue: Class,
    #[serde(rename = "Sorcerer")]
    sorcerer: Class,
    #[serde(rename = "Generic")]
    generic: GenericClass,
}

#[derive(Serialize, Deserialize)]
struct Class {
    #[serde(rename = "Paragon (Board)")]
    paragon_board: HashMap<String, ParagonBoard>,
    #[serde(rename = "Paragon (Glyph)")]
    paragon_glyphs: HashMap<String, ParagonGlyph>,
    #[serde(rename = "Paragon (Node)")]
    paragon_nodes: HashMap<String, ParagonNode>,
}

#[derive(Serialize, Deserialize)]
struct GenericClass {
    #[serde(rename = "Paragon (Node)")]
    paragon_nodes: HashMap<String, ParagonNode>,
}

#[derive(Serialize, Deserialize)]
struct ParagonBoard {
    name: String,
    data: BoardData,
}

struct BoardData {
    rows: Vec<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
struct ParagonGlyph {
    name: String,
    bonus: String,
    tags: Vec<String>,
    threshold_requirements: ThresholdRequirements,
}

#[derive(Serialize, Deserialize)]
struct ParagonNode {
    name: Option<String>,
    desc: Option<String>,
    tags: Option<Vec<String>>,
    threshold_bonus: Option<String>,
    threshold_requirements: Option<ThresholdRequirements>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ThresholdRequirements {
    Global(String),
    ByClass(PerClassRequirements),
}

#[derive(Serialize, Deserialize)]
struct PerClassRequirements {
    #[serde(rename = "Barbarian")]
    barbarian: Option<Vec<String>>,
    #[serde(rename = "Druid")]
    druid: Option<Vec<String>>,
    #[serde(rename = "Necromancer")]
    necromancer: Option<Vec<String>>,
    #[serde(rename = "Rogue")]
    rogue: Option<Vec<String>>,
    #[serde(rename = "Sorcerer")]
    sorcerer: Option<Vec<String>>,
}

impl<'de> Deserialize<'de> for BoardData {
    fn deserialize<D>(_deserializer: D) -> Result<BoardData, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BoardData { rows: Vec::new() })
    }
}

impl Serialize for BoardData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup: S::SerializeTuple = serializer.serialize_tuple(self.rows.len())?;
        let mut buffer = String::from("");

        for row in self.rows.iter() {
            buffer.clear();

            for col in row.iter() {
                col.as_ref().map(|cell| buffer.push_str(cell));

                buffer.push_str(",");
            }

            tup.serialize_element(&buffer)?;
        }

        tup.end()
    }
}
