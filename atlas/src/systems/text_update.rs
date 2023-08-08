use crate::{
    entity_manager::{EntityManager},
    event_bus::EventReader,
    game_entities::ui_label::UiLabel,
};

pub enum TextChangeEvent {
    TextChange(usize, String),
}

pub fn update_text(entity_manager: &mut EntityManager, event_reader: &mut EventReader) {
    event_reader.read().map(|events| {
        events.for_each(|event| match event {
            TextChangeEvent::TextChange(id, content) => {
                entity_manager
                    .iter_mut::<UiLabel>()
                    .find(|label| label.id == id)
                    .map(|label| label.entity.renderer.set_text(content));
            }
        });
    });
}
