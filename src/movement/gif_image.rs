use iced::widget::image;
use uuid::Uuid;

struct GifImage {
    pub id: Uuid,
    pub image_ids: Vec<Uuid>,
}

// impl GifImage {
//     fn view(&self) {} -> Element<'_, Message> {

//     }
// }
