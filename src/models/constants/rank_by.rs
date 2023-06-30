use serde::{Deserialize};

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum RankBy {
    Prominence,
    Distance,
}

impl RankBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            RankBy::Prominence => "prominence",
            RankBy::Distance => "distance",
        }
    }
}
