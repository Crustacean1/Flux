use glam::{Quat, Vec3, Vec4, Vec4Swizzles};

use crate::{
    components::{
        camera::Camera, collider::Collider, physical_body::PhysicalBody, transform::Transform,
    },
    entity_manager::{ComponentMutIteratorGenerator, EntityManager},
    event_bus::{EventReader, EventSender},
    game_entities::{bullet::BulletEntity, player_ship::PlayerShip},
    graphics::graphics_context::IoEvent,
};

use super::bullet_detonator::BulletEvent;

pub enum GameEvent {
    ShootPlasmaBullet(Transform, BulletEntity),
    RemoveBullet(usize),
}

pub struct PlayerController {
    buttons: Vec<char>,
    thruster_force: f32,
    mouse_speed: f32,
    bullet_cooldown: u128,
}

impl<'a>
    ComponentMutIteratorGenerator<'a, (&'a mut Transform, &'a mut PhysicalBody, &'a mut Camera)>
    for EntityManager
{
    fn get_mut_view(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = (&'a mut Transform, &'a mut PhysicalBody, &'a mut Camera)> + 'a>
    {
        let player = self.iter_mut::<PlayerShip>().map(|player| {
            (
                &mut player.transform,
                &mut player.entity.physical_body,
                &mut player.entity.camera,
            )
        });
        Box::new(player)
    }
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            buttons: vec![],
            thruster_force: 2.5,
            mouse_speed: 0.001,
            bullet_cooldown: 0,
        }
    }

    pub fn control(
        &mut self,
        time: u128,
        entity_manager: &mut EntityManager,
        event_reader: &mut EventReader,
        event_sender: &mut EventSender,
    ) {
        entity_manager.get_mut_view().for_each(
            |(transform, physical_body, camera): (
                &mut Transform,
                &mut PhysicalBody,
                &mut Camera,
            )| {
                self.process_inputs(transform, camera, event_reader);
                self.move_around(time, transform, physical_body, event_sender);
            },
        );
    }

    fn process_inputs(
        &mut self,
        transform: &mut Transform,
        camera: &mut Camera,
        event_reader: &mut EventReader,
    ) {
        event_reader.read(|event| match event {
            IoEvent::KeyPressed(key) => self.buttons.push(key),
            IoEvent::KeyReleased(key) => {
                if let Some(key) = self.buttons.iter().position(|&k| k == key) {
                    self.buttons.swap_remove(key);
                }
            }
            IoEvent::MouseMotion((x, y)) => {
                //let x_rotation = Quat::from_rotation_x(-y * self.mouse_speed);
                //let y_rotation = Quat::from_rotation_y(-x * self.mouse_speed);
                //transform.rotation = transform.rotation * y_rotation * x_rotation;
                let yaw = Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), y * self.mouse_speed);
                let pitch = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), x * self.mouse_speed);
                camera.apply_rotation(pitch);
                camera.apply_rotation(yaw);
            }
            _ => {}
        });
    }

    fn move_around(
        &mut self,
        delta: u128,
        transform: &mut Transform,
        physical_body: &mut PhysicalBody,
        event_sender: &mut EventSender,
    ) {
        let mut force = Vec4::ZERO;

        self.bullet_cooldown = if self.bullet_cooldown > delta {
            self.bullet_cooldown - delta
        } else {
            0
        };

        let create_bullet = |pos: Vec4| {
            let mut transform1 = *transform;
            transform1.position += transform.to_global(pos).xyz();

            let velocity = physical_body.velocity()
                + transform.to_global(Vec4::new(0.0, 0.0, -256.0, 0.0)).xyz();

            let mut body1 = PhysicalBody::new(1.0, 1.0, 1.0);

            body1.momentum = velocity;

            let sender = event_sender.clone();

            event_sender.write(GameEvent::ShootPlasmaBullet(
                transform1,
                BulletEntity {
                    collider: Collider {
                        toi: 0.0,
                        last_impact: Vec3::ZERO,
                        radius: 0.5,
                        callback: Some(Box::new(move |id, entity, pos| {
                            sender.write(BulletEvent::Exploded(id, pos));
                            sender.write(BulletEvent::Damaged(entity, 10.0))
                        })),
                    },
                    body: body1,
                    lifetime: 5.0,
                },
            ));
        };

        self.buttons.iter().for_each(|button| match button {
            'S' => force += Vec4::new(0.0, 0.0, self.thruster_force, 0.0),
            'W' => force += Vec4::new(0.0, 0.0, -self.thruster_force, 0.0),
            '1' => {
                if self.bullet_cooldown == 0 {
                    self.bullet_cooldown = 2_00_000_000;
                    create_bullet(Vec4::new(-1.5, -0.4, -2.5, 0.0));
                    create_bullet(Vec4::new(1.5, -0.4, -2.5, 0.0));
                }
            }
            _ => {}
        });

        let force = transform.model() * force;
        physical_body.impulse(Vec3::new(force.x, force.y, force.z));
    }
}
