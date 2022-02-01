use druid::{Data, Lens, Color};


#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub top_left: Color,
    pub top_right: Color,
    pub bottom: Color,
}