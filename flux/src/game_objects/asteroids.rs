use atlas::{
    components::{
        camera::{Camera, Frustrum},
        particle_emitter::{ParticleEmitter, ParticleEmitterDefinition},
        physical_body::PhysicalBody,
        skybox_renderer::SkyboxRenderer,
        sprite_renderer::SpriteRenderer,
        text_renderer::TextRenderer,
        transform::Transform,
    },
    entity_manager::EntityManager,
    game_entities::{
        asteroid::{add_perlin_noise, generate_asteroid, AsteroidEntity},
        bullet::BulletEntity,
        enemy_ship::EnemyShip,
        hud::HudEntity,
        player_ship::PlayerShip,
        space_box::SpaceBox,
        starlight::Starlight,
        ui_label::UiLabel,
    },
    game_root::GameError,
    graphics::{
        graphics_context::{self, GraphicsContext},
        instanced_primitive::InstancedPrimitive,
        lights::{Light, LightColor},
        material::{
            phong_material::PhongMaterial, skybox_material::SkyboxMaterial,
            sprite_material::SpriteMaterial,
        },
        mesh::Mesh,
        primitive::Primitive,
        vertices::{generator, sphere},
    },
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
    systems::particle_system::thruster_spawner,
};
use glam::{Quat, Vec3};

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    graphics_context: &mut GraphicsContext,
) -> Result<(), GameError> {
    create_asteroids(entity_manager, resource_manager);

    let skybox_material: SkyboxMaterial = resource_manager.get("space1").res;
    let skybox = SkyboxRenderer::new(1.0, skybox_material);

    let meshes = ["spaceship3"];

    let meshes: Vec<Mesh> = meshes
        .iter()
        .map(|mesh| {
            return resource_manager.get(mesh).res;
        })
        .collect();

    let (width, height) = graphics_context.dimensions();

    let camera = Camera::new(
        Frustrum::perspective(width as f32, height as f32, 0.1, 1000.0),
        Vec3::new(0.0, 1.5, 5.0),
    );

    let thruster = create_thruster(resource_manager);

    entity_manager.add_at(
        PlayerShip {
            camera,
            physical_body: PhysicalBody::new(10., 10.),
            thruster,
            mesh: resource_manager.get("spaceship3").res,
        },
        Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Quat::IDENTITY,
        },
    );

    meshes.iter().enumerate().for_each(|(i, mesh)| {
        let thruster = create_thruster(resource_manager);

        entity_manager.add_at(
            EnemyShip {
                physical_body: PhysicalBody::new(10., 10.),
                thruster,
                mesh: mesh.clone(),
            },
            Transform::pos(Vec3::new(0.0, 0.0, -10.0)),
        );
    });

    entity_manager.add_at(BulletEntity {}, Transform::pos(Vec3::new(10.0, 0.0, 0.0)));

    entity_manager.add(SpaceBox { renderer: skybox });
    create_lights(entity_manager, resource_manager);
    create_hud(entity_manager, resource_manager, graphics_context);

    Ok(())
}

fn create_asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) {
    let _ = generate_asteroid((200, 200))
        .map(|texture| resource_manager.register("perlin", PhongMaterial { diffuse: texture }));

    let material: &PhongMaterial = &resource_manager.get("perlin").res;

    (0..5).for_each(|x| {
        (0..5).for_each(|y| {
            (0..5).for_each(|z| {
                entity_manager.add_at(
                    AsteroidEntity::new(material.clone(), 1.0),
                    Transform::pos(Vec3::new(x as f32 * 20.0, y as f32 * 20.0, z as f32 * 20.0)),
                );
            })
        })
    });
}

fn create_hud(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    graphics_context: &mut GraphicsContext,
) {
    let (width, height) = graphics_context.dimensions();
    let sprite = resource_manager.get("crosshair").res;
    let font = resource_manager.get("main").res;

    entity_manager.add_at(
        HudEntity {
            crosshair: SpriteRenderer::quad((200.0, 200.0), sprite),
            enemy_name: TextRenderer::new("Enemy", font),
        },
        Transform::pos(Vec3::new(width as f32 * 0.5, height as f32 * 0.5, 0.)),
    );
}

fn create_thruster(resource_manager: &mut SceneResourceManager) -> ParticleEmitter {
    let thruster_material = resource_manager.get("thruster").res;
    let (vertices, indices) = generator::quad(1.0, 1.0);
    let instanced_mesh = InstancedPrimitive::new(&vertices, &indices, &vec![]);

    let emitter_definition = ParticleEmitterDefinition {
        count: 1000,
        rate: 0.005,
    };

    ParticleEmitter::new(
        emitter_definition,
        thruster_material,
        instanced_mesh,
        &thruster_spawner,
    )
}

fn create_lights(entity_manager: &mut EntityManager, resource_manager: &mut SceneResourceManager) {
    entity_manager.add(Starlight {
        light: Light::DirectionalLight(
            Vec3::new(0.0, -1.0, 0.0),
            LightColor {
                ambient: Vec3::new(0.01, 0.01, 0.01),
                diffuse: Vec3::new(0.5, 0.5, 0.5),
                specular: Vec3::new(0.3, 0.3, 0.3),
            },
        ),
    });
}
