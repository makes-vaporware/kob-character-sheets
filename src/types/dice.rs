#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl DiceType {
    pub fn sides(&self) -> u32 {
        match self {
            DiceType::D4 => 4,
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D12 => 12,
            DiceType::D20 => 20,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            DiceType::D4 => "d4",
            DiceType::D6 => "d6",
            DiceType::D8 => "d8",
            DiceType::D10 => "d10",
            DiceType::D12 => "d12",
            DiceType::D20 => "d20",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "d4" => Some(DiceType::D4),
            "d6" => Some(DiceType::D6),
            "d8" => Some(DiceType::D8),
            "d10" => Some(DiceType::D10),
            "d12" => Some(DiceType::D12),
            "d20" => Some(DiceType::D20),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            DiceType::D4,
            DiceType::D6,
            DiceType::D8,
            DiceType::D10,
            DiceType::D12,
            DiceType::D20,
        ]
    }
}
