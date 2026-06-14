use iced::widget::scrollable;
use iced::Task;

pub struct ScrollList {
    pub selected_index: usize,
    pub scrollable_id: scrollable::Id,
    pub item_height: f32,
    pub viewport_height: f32,
}

impl ScrollList {
    pub fn new(item_height: f32, viewport_height: f32) -> Self {
        Self {
            selected_index: 0,
            scrollable_id: scrollable::Id::unique(),
            item_height,
            viewport_height,
        }
    }

    /// Calculate centering scroll offset and generate scroll task
    pub fn scroll_to_selected<Message: 'static>(&self, total_items: usize) -> Task<Message> {
        let target_y = (self.selected_index as f32) * self.item_height;
        let total_height = (total_items as f32) * self.item_height;
        let max_scroll = (total_height - self.viewport_height).max(0.0);
        
        let center_offset = target_y - (self.viewport_height / 2.0) + (self.item_height / 2.0);
        let final_y = center_offset.clamp(0.0, max_scroll);

        iced::widget::scrollable::scroll_to(
            self.scrollable_id.clone(),
            iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: final_y }
        )
    }
}
