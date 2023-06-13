pub enum ReviewSort {
    MostRelevant,
    Newest,
}
impl ReviewSort {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ReviewSort::MostRelevant => "most_relevant",
            ReviewSort::Newest => "newest",
        }
    }
}