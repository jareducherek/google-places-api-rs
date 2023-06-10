pub enum ReviewSort {
    MostRelevant,
    Newest,
}
impl ToString for ReviewSort {
    fn to_string(&self) -> String {
        match *self {
            ReviewSort::MostRelevant => "most_relevant".to_string(),
            ReviewSort::Newest => "newest".to_string(),
        }
    }
}
