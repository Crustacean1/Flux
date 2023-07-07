use atlas::{
    components::{
        camera::{Camera, Frustrum},
        collider::Collider,
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
        instanced_mesh::InstancedMesh,
        lights::{Light, LightColor},
        material::{
            phong_material::PhongMaterial, skybox_material::SkyboxMaterial,
            sprite_material::SpriteMaterial,
        },
        model::Model,
        mesh::Mesh,
        vertices::{crosshair::crosshair, generator, sphere},
    },
    resource_manager::{scene_resource_manager::SceneResourceManager, ResourceManager},
    systems::{hud_refresher::HudRefresher, particle_system::thruster_spawner},
};
use glam::{Quat, Vec3};
use rand::Rng;

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    graphics_context: &mut GraphicsContext,
) -> Result<HudRefresher, GameError> {
    create_asteroids(entity_manager, resource_manager);

    let skybox_material: SkyboxMaterial = resource_manager.get("space1").res;
    let skybox = SkyboxRenderer::new(1.0, skybox_material);

    let meshes = ["spaceship3"];

    let meshes: Vec<Model> = meshes
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

    let player_id = entity_manager.add_at(
        PlayerShip {
            camera,
            physical_body: PhysicalBody::new(10., 10.),
            collider: Collider { radius: 2.0 },
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
                collider: Collider { radius: 2.0 },
                physical_body: PhysicalBody::new(10., 10.),
                thruster,
                mesh: mesh.clone(),
            },
            Transform::pos(Vec3::new(0.0, 0.0, -10.0)),
        );
    });

    entity_manager.add(SpaceBox { renderer: skybox });
    create_lights(entity_manager, resource_manager);
    let hud_id = create_hud(entity_manager, resource_manager, graphics_context);

    Ok(HudRefresher { hud_id, player_id })
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
                let mut rnd = rand::thread_rng();

                /*entity_manager.add_at(
                    AsteroidEntity::prefab(material.clone(), 10.0),
                    Transform::pos(Vec3::new(
                        rnd.gen_range(-100.0..100.0),
                        rnd.gen_range(-100.0..100.0),
                        rnd.gen_range(-100.0..100.0),
                    )),
                );*/
            })
        })
    });
}

fn create_hud(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    graphics_context: &mut GraphicsContext,
) -> usize {
    let (width, height) = graphics_context.dimensions();
    let font = resource_manager.get("main").res;
    let mat = resource_manager.get("white").res;

    let crosshair = SpriteRenderer::crosshair(mat);

    entity_manager.add_at(
        HudEntity {
            crosshair,
            velocity: TextRenderer::new(
                Transform::pos(Vec3::new(100., -50.0, 0.)),
                "Velocity",
                font,
            ),
        },
        Transform::pos(Vec3::new(width as f32 * 0.5, height as f32 * 0.5, 0.)),
    )
}

fn create_thruster(resource_manager: &mut SceneResourceManager) -> ParticleEmitter {
    let thruster_material = resource_manager.get("thruster").res;
    let (vertices, indices) = generator::quad(1.0, 1.0);
    let instanced_mesh = InstancedMesh::new(&vertices, &indices, &vec![]);

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
                ambient: Vec3::new(0.3, 0.3, 0.3),
                diffuse: Vec3::new(0.5, 0.5, 0.5),
                specular: Vec3::new(0.0, 0.0, 0.2),
            },
        ),
    });
}
