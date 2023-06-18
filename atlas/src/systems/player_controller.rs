use glam::{Quat, Vec3, Vec4};

use crate::{
    components::{camera::Camera, physical_body::PhysicalBody, transform::Transform},
    entity_manager::{ComponentMutIteratorGenerator, EntityManager},
    event_bus::{EventReader, EventSender},
    game_entities::{bullet::BulletEntity, player_ship::PlayerShip},
    graphics::graphics_context::IoEvent,
};

pub struct PlayerController {
    buttons: Vec<char>,
    thruster_force: f32,
    mouse_speed: f32,
}

impl<'a> ComponentMutIteratorGenerator<'a, (&'a mut Transform, &'a mut PhysicalBody, &'a Camera)>
    for EntityManager
{
    fn get_mut_view(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = (&'a mut Transform, &'a mut PhysicalBody, &'a Camera)> + 'a> {
        let player = self.iter_mut::<PlayerShip>().map(|player| {
            (
                &mut player.transform,
                &mut player.entity.physical_body,
                &player.entity.camera,
            )
        });
        Box::new(player)
    }
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            buttons: vec![],
            thruster_force: 0.1,
            mouse_speed: 0.001,
        }
    }

    pub fn control(
        &mut self,
        entity_manager: &mut EntityManager,
        event_reader: &mut EventReader,
        event_sender: &mut EventSender,
    ) {
        entity_manager.get_mut_view().for_each(
            |(transform, physical_body, _): (&mut Transform, &mut PhysicalBody, &Camera)| {
                self.process_inputs(transform, event_reader);
                self.move_around(transform, physical_body, event_sender);
            },
        );
    }

    fn process_inputs(&mut self, transform: &mut Transform, event_reader: &mut EventReader) {
        event_reader.read::<IoEvent>().map(|events| {
            events.for_each(|event| match event {
                IoEvent::KeyPressed(key) => self.buttons.push(key),
                IoEvent::KeyReleased(key) => {
                    if let Some(key) = self.buttons.iter().position(|&k| k == key) {
                        self.buttons.swap_remove(key);
                    }
                }
                IoEvent::MouseMotion((x, y)) => {
                    let x_rotation = Quat::from_rotation_x(-y * self.mouse_speed);
                    let y_rotation = Quat::from_rotation_y(-x * self.mouse_speed);
                    transform.rotation = transform.rotation * y_rotation * x_rotation;
                }
                _ => {}
            })
        });
    }

    fn move_around(
        &self,
        transform: &Transform,
        physical_body: &mut PhysicalBody,
        event_sender: &mut EventSender,
    ) {
        let force: Vec4 = self
            .buttons
            .iter()
            .map(|button| match button {
                'S' => Vec4::new(0.0, 0.0, self.thruster_force, 0.0),
                'W' => Vec4::new(0.0, 0.0, -self.thruster_force, 0.0),
                _ => Vec4::new(0.0, 0.0, 0.0, 0.0),
            })
            .sum();

        let force = transform.model() * force * 100.;
        physical_body.add_force(Vec3::new(force.x, force.y, force.z));

        self.buttons.iter().for_each(|button| match button {
            '1' => event_sender.write(BulletEntity {}),
            _ => {}
        });
    }
}
