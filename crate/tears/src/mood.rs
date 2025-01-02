use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// The mood the receiving person is in.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Mood {
    /// Unresponsiveness to any interaction. Outbursts, self-harm.
    _01_Anguished,
    /// Silence, eyes stare blankly. Little movement.
    _02_Closed,
    /// One word answers, eyes assessing every detail.
    _03_Cautious,
    /// Asks for justification / to see evidence.
    _04_Unsettled,
    /// No sad symptoms, smile is conscious.
    _05_Calm,
    /// Smiles subconciously.
    _06_Hopeful,
}

impl Mood {
    /// Returns an iterator over all variants of this `Mood` enum.
    pub fn iter() -> impl ExactSizeIterator<Item = Mood> {
        [
            Mood::_01_Anguished,
            Mood::_02_Closed,
            Mood::_03_Cautious,
            Mood::_04_Unsettled,
            Mood::_05_Calm,
            Mood::_06_Hopeful,
        ]
        .into_iter()
    }

    /// Returns the number that this mood would fit on a scale of 1 to 10.
    ///
    /// Notably the variants don't actually reach 10, because this application's
    /// purpose is to bring someone out of sadness, and has not (yet) extended
    /// to joy.
    pub fn rank(self) -> u8 {
        match self {
            Mood::_01_Anguished => 1,
            Mood::_02_Closed => 2,
            Mood::_03_Cautious => 3,
            Mood::_04_Unsettled => 4,
            Mood::_05_Calm => 5,
            Mood::_06_Hopeful => 6,
        }
    }

    pub fn symptoms(self) -> &'static str {
        match self {
            Mood::_01_Anguished => "Unresponsiveness to any interaction. Outbursts, self-harm.",
            Mood::_02_Closed => "Silence, eyes stare blankly. Little movement.",
            Mood::_03_Cautious => "One word answers, eyes assessing every detail.",
            Mood::_04_Unsettled => "Asks for justification / to see evidence.",
            Mood::_05_Calm => "No sad symptoms, smile takes conscious effort.",
            Mood::_06_Hopeful => "Smiles subconciously.",
        }
    }

    pub fn summary(self) -> &'static str {
        match self {
            Mood::_01_Anguished => "The person believes that to live is to suffer.",
            Mood::_02_Closed => "The person believes that trust no longer exists.",
            Mood::_03_Cautious => "The person only trusts people who know how to empathize.",
            Mood::_04_Unsettled => "The person is suspicious of people.",
            Mood::_05_Calm => "The person believes life is okay.",
            Mood::_06_Hopeful => "The person believes there is good in life.",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Mood::_01_Anguished => {
                "Being awake is already experienced as emotional pain, so \
                every stimulus is overwhelming."
            }
            Mood::_02_Closed => {
                "No promise of \"better\" gets through, usually because past \
                attempts at improvement have ended in negative experiences. \
                i.e. Don't make things worse"
            }
            Mood::_03_Cautious => {
                "The emotions are in a state that the person will hate doing \
                anything an untrusted person says."
            }
            Mood::_04_Unsettled => {
                "Trust has been broken, but the person is willing to try and \
                see if it can be mended."
            }
            Mood::_05_Calm => {
                "There is little / no bias towards things being positive or \
                negative."
            }
            Mood::_06_Hopeful => {
                "The person believes goodness will happen when one works \
                towards it."
            }
        }
    }
}

impl TryFrom<u8> for Mood {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Mood::_01_Anguished),
            2 => Ok(Mood::_02_Closed),
            3 => Ok(Mood::_03_Cautious),
            4 => Ok(Mood::_04_Unsettled),
            5 => Ok(Mood::_05_Calm),
            6 => Ok(Mood::_06_Hopeful),
            _ => Err(()),
        }
    }
}

impl Display for Mood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mood::_01_Anguished => "Anguished".fmt(f),
            Mood::_02_Closed => "Closed".fmt(f),
            Mood::_03_Cautious => "Cautious".fmt(f),
            Mood::_04_Unsettled => "Unsettled".fmt(f),
            Mood::_05_Calm => "Calm".fmt(f),
            Mood::_06_Hopeful => "Hopeful".fmt(f),
        }
    }
}

impl FromStr for Mood {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Anguished" => Ok(Mood::_01_Anguished),
            "Closed" => Ok(Mood::_02_Closed),
            "Cautious" => Ok(Mood::_03_Cautious),
            "Unsettled" => Ok(Mood::_04_Unsettled),
            "Calm" => Ok(Mood::_05_Calm),
            "Hopeful" => Ok(Mood::_06_Hopeful),
            _ => Err(()),
        }
    }
}
