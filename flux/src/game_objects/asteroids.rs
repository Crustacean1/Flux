use atlas::{
    components::{
        camera::{Camera, Frustrum},
        collider::Collider,
        health_renderer::HealthRenderer,
        particle_emitter::{ParticleEmitter, ParticleEmitterDefinition},
        physical_body::PhysicalBody,
        skybox_renderer::SkyboxRenderer,
        sprite_renderer::SpriteRenderer,
        text_renderer::TextRenderer,
        transform::Transform,
        unit::Unit,
    },
    entity_manager::EntityManager,
    game_entities::{
        asteroid::{generate_asteroid, AsteroidEntity},
        enemy_ship::EnemyShip,
        hud::HudEntity,
        player_ship::PlayerShip,
        space_box::SpaceBox,
        starlight::Starlight,
    },
    game_root::GameError,
    graphics::{
        graphics_context::GraphicsContext,
        instanced_mesh::InstancedMesh,
        lights::{Light, LightColor},
        material::{
            phong_material::PhongMaterial, skybox_material::SkyboxMaterial,
            sprite_material::SpriteMaterial,
        },
        model::Model,
        vertices::generator,
    },
    resource_manager::{font::Font, scene_resource_manager::SceneResourceManager, ResourceManager},
    systems::particle_system::thruster_spawner,
};
use glam::{Quat, Vec3};
use rand::Rng;

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    graphics_context: &mut GraphicsContext,
) -> Result<(), GameError> {
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
        Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 0.0),
    );

    let thruster = create_thruster(resource_manager);

    let player_id = entity_manager.add_at(
        PlayerShip {
            camera,
            physical_body: PhysicalBody::new(10., 10., 0.995),
            collider: Collider {
                toi: 0.0,
                last_impact: Vec3::ZERO,
                radius: 2.0,
                callback: None,
            },
            thruster,
            mesh: resource_manager.get("spaceship3").res,
        },
        Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Quat::IDENTITY,
        },
    );

    meshes.iter().enumerate().for_each(|(_i, mesh)| {
        let thruster = create_thruster(resource_manager);

        entity_manager.add_at(
            EnemyShip {
                collider: Collider {
                    toi: 0.0,
                    last_impact: Vec3::ZERO,
                    radius: 2.0,
                    callback: None,
                },
                physical_body: PhysicalBody::new(100., 100., 0.995),
                thruster,
                mesh: mesh.clone(),
                info: Unit::new("Ravager A", "Enemy", 256.0),
            },
            Transform::pos(Vec3::new(0.0, 0.0, -10.0)),
        );
    });

    entity_manager.add(SpaceBox { renderer: skybox });
    create_lights(entity_manager, resource_manager);
    create_hud(
        player_id,
        entity_manager,
        resource_manager,
        graphics_context,
    );

    Ok(())
}

fn create_asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) {
    let _ = generate_asteroid((200, 200))
        .map(|texture| resource_manager.register("perlin", PhongMaterial { diffuse: texture }));

    let material: &PhongMaterial = &resource_manager.get("perlin").res;

    (-2..3).for_each(|x| {
        (-2..3).for_each(|y| {
            (-2..3).for_each(|z| {
                let mut rnd = rand::thread_rng();

                entity_manager.add_at(
                    AsteroidEntity::prefab(material.clone(), rnd.gen_range(5.0..20.0)),
                    Transform::pos(Vec3::new(
                        x as f32 * 100.0 + rnd.gen_range(-10.0..10.0),
                        y as f32 * 100.0 + rnd.gen_range(-10.0..10.0),
                        z as f32 * 100.0 + rnd.gen_range(-10.0..10.0),
                    )),
                );
            })
        })
    });
}

fn create_hud(
    player_id: usize,
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
    graphics_context: &mut GraphicsContext,
) -> usize {
    let (width, height) = graphics_context.dimensions();
    let font: Font = resource_manager.get("main").res;
    let mat: SpriteMaterial = resource_manager.get("white").res;

    let crosshair = SpriteRenderer::crosshair(mat.clone());

    let health = HealthRenderer::health_bar(100.0);

    entity_manager.add_at(
        HudEntity {
            health,
            player_id,
            crosshair,
            velocity: new_text("Velocitty", font.clone(), 100.0, -30.0),
            mass: new_text("Unit: []", font.clone(), 110.0, 0.0),
            unit: new_text("Unit allometry", font.clone(), 100.0, 30.0),
        },
        Transform::pos(Vec3::new(width as f32 * 0.5, height as f32 * 0.5, 0.)),
    )
}

fn new_text(text: &str, font: Font, x: f32, y: f32) -> TextRenderer {
    TextRenderer::new(Transform::pos(Vec3::new(x, y, 0.)), text, font)
}

fn create_thruster(resource_manager: &mut SceneResourceManager) -> ParticleEmitter {
    let thruster_material = resource_manager.get("explosion").res;
    let (vertices, indices) = generator::quad(1.0, 1.0);
    let instanced_mesh = InstancedMesh::new(&vertices, &indices, &vec![]);

    let emitter_definition = ParticleEmitterDefinition {
        count: 50,
        rate: 0.005,
    };

    ParticleEmitter::new(
        emitter_definition,
        thruster_material,
        instanced_mesh,
        Box::new(thruster_spawner),
    )
}

fn create_lights(entity_manager: &mut EntityManager, _resource_manager: &mut SceneResourceManager) {
    entity_manager.add(Starlight {
        light: Light::DirectionalLight(
            Vec3::new(0.0, -1.0, 0.0),
            LightColor {
                ambient: Vec3::new(0.6, 0.6, 0.6),
                diffuse: Vec3::new(0.5, 0.5, 0.5),
                specular: Vec3::new(0.0, 0.0, 0.2),
            },
        ),
    });
}
