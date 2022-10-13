
pub(crate) enum EffectOptionType {
    analog,
    boolean,
    color,
    select,
    unknown,
}

pub(crate) struct ApiOption {
    pub id: usize,
    pub name: String,
    pub option_type: EffectOptionType,
    pub min: u64,
    pub max: u64,
    pub options: Vec<String>,
    pub analog_value: i64,
    pub boolean_value: bool,
    pub color_value: String,
    pub select_value: String,

}