use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NavGridItem {
    item_id: String,
    path_to_thumbnail: String,
    title: String,
    state_id: usize,
}
impl NavGridItem {
    pub fn new(section_id: &str, item_id: &str, title: &str, state_id: usize) -> Self {
        let path_to_thumbnail = format!(
            "/static/img/simulations/{}/{}/{}.png",
            section_id, item_id, item_id
        );
        let item_id = String::from(item_id);
        let title = String::from(title);

        NavGridItem {
            item_id,
            path_to_thumbnail,
            title,
            state_id,
        }
    }
}
