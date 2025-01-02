use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// Whether the receiving person trusts you.
///
/// A good indicator whether the receiving person trusts you is if they initiate
/// a conversation with you.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Trust {
    /// The receiving person does not trust you.
    Absent,
    /// The receiving person trusts you.
    Present,
}

impl Trust {
    /// Returns an iterator over all variants of this `Trust` enum.
    pub fn iter() -> impl Iterator<Item = Trust> {
        [Trust::Absent, Trust::Present].into_iter()
    }

    pub fn description(self) -> &'static str {
        match self {
            Trust::Absent => "The person has not initiated a conversation with you recently.",
            Trust::Present => {
                "The person has initiated a conversation with you recently, with no obligation."
            }
        }
    }
}

impl Display for Trust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trust::Absent => "Absent".fmt(f),
            Trust::Present => "Present".fmt(f),
        }
    }
}

impl FromStr for Trust {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Absent" => Ok(Trust::Absent),
            "Present" => Ok(Trust::Present),
            _ => Err(()),
        }
    }
}
