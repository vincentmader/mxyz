use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NavGridItem {
    item_id: String,
    path_to_thumbnail: String,
    title: String,
    state: String,
}
impl NavGridItem {
    pub fn new(item_id: &str, title: &str, state: &str) -> Self {
        let path_to_thumbnail = format!("/static/img/simulations/{}/thumbnail.png", item_id);
        let item_id = String::from(item_id);
        let title = String::from(title);
        let state = String::from(state);

        NavGridItem {
            item_id,
            path_to_thumbnail,
            title,
            state,
        }
    }
}
