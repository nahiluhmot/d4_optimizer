use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
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
    data: BoardData<String>,
}

struct BoardData<A> {
    rows: Vec<Vec<Option<A>>>,
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

static DEFAULT_BOARD_HEIGHT: usize = 20;
static DEFAULT_BOARD_WIDTH: usize = 20;

impl<'de> Deserialize<'de> for BoardData<String> {
    fn deserialize<D>(deserializer: D) -> Result<BoardData<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BoardDataVisitor)
    }
}

struct BoardDataVisitor;

impl<'de> Visitor<'de> for BoardDataVisitor {
    type Value = BoardData<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let suggested_height = seq.size_hint().unwrap_or(0);
        let height = if suggested_height > DEFAULT_BOARD_HEIGHT {
            suggested_height
        } else {
            DEFAULT_BOARD_HEIGHT
        };
        let mut rows: Vec<Vec<Option<String>>> = Vec::with_capacity(height);

        while let Some(serialized_row) = seq.next_element::<&str>()? {
            let split_by_comma = serialized_row.split(',');

            let (lower_bound, upper_bound) = split_by_comma.size_hint();
            let suggested_width = upper_bound.unwrap_or(lower_bound);
            let width = if suggested_width > DEFAULT_BOARD_WIDTH {
                suggested_width
            } else {
                DEFAULT_BOARD_WIDTH
            };

            let mut row: Vec<Option<String>> = Vec::with_capacity(width);

            for split in split_by_comma {
                row.push(if split.is_empty() {
                    None
                } else {
                    Some(String::from(split))
                })
            }

            rows.push(row)
        }

        Ok(BoardData { rows })
    }
}

impl Serialize for BoardData<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(self.rows.len())?;
        let mut buffer = String::new();

        for row in self.rows.iter() {
            let last_idx = row.len() - 1;

            buffer.clear();

            for (idx, cell) in row.iter().enumerate() {
                cell.as_ref().map(|cell| buffer.push_str(cell));

                if idx != last_idx {
                    buffer.push(',');
                }
            }

            tup.serialize_element(&buffer)?;
        }

        tup.end()
    }
}
