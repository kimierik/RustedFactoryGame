use strum_macros::EnumIter;


#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Material {
    Money,
    MoneyMultiplier,
}
impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
