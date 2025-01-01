/// A suggestion to show for a given trust + mood level.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Suggestion {
    /// Action to take, e.g. `"Stay Away"`.
    pub action: &'static str,
    /// Description or rationale.
    ///
    /// e.g.
    ///
    /// > Your presence pressurizes the person to be aware of you, and does not
    /// > allow them to settle down.
    pub description: &'static str,
}

impl Suggestion {
    pub fn action(&self) -> &str {
        self.action
    }

    pub fn description(&self) -> &str {
        self.description
    }
}
