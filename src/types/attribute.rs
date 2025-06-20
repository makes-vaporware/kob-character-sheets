use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum AttributeType {
    Brains,
    Fight,
    Charm,
    Brawn,
    Flight,
    Grit,
}

// Add this to get all attribute types
impl AttributeType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Brains,
            Self::Fight,
            Self::Charm,
            Self::Brawn,
            Self::Flight,
            Self::Grit,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Brains => "Brains",
            Self::Fight => "Fight",
            Self::Charm => "Charm",
            Self::Brawn => "Brawn",
            Self::Flight => "Flight",
            Self::Grit => "Grit",
        }
    }
}
