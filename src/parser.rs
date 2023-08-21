use serde::{Deserialize, Serialize};
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
    data: Vec<String>,
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
