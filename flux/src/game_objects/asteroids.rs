use atlas::{
    components::{
        camera::{Camera, Frustrum},
        particle_emitter::{ParticleEmitter, ParticleEmitterDefinition},
        physical_body::PhysicalBody,
        skybox_renderer::SkyboxRenderer,
        transform::Transform,
    },
    entity_manager::EntityManager,
    game_entities::{
        enemy_ship::EnemyShip, player_ship::PlayerShip, space_box::SpaceBox, starlight::Starlight,
    },
    game_root::GameError,
    graphics::{
        instanced_primitive::InstancedPrimitive,
        lights::{Light, LightColor},
        material::{phong_material::PhongMaterial, skybox_material::SkyboxMaterial},
        mesh::Mesh,
        vertices::generator,
    },
    resource_manager::{font::Font, scene_resource_manager::SceneResourceManager, ResourceManager},
    systems::particle_system::thruster_spawner,
};
use glam::{Quat, Vec3};

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) -> Result<(), GameError> {
    let skybox_material: SkyboxMaterial = resource_manager.get("space1").res;
    let skybox = SkyboxRenderer::new(1.0, skybox_material);

    let meshes = ["spaceship3"];

    let meshes: Vec<Mesh> = meshes
        .iter()
        .map(|mesh| {
            return resource_manager.get(mesh).res;
        })
        .collect();

    let camera = Camera::new(
        Frustrum::perspective(1920 as f32, 1080 as f32, 0.1, 100.0),
        Vec3::new(0.0, 1.5, 5.0),
    );
    let thruster = create_thruster(entity_manager, resource_manager);

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
        let thruster = create_thruster(entity_manager, resource_manager);

        entity_manager.add_at(
            EnemyShip {
                physical_body: PhysicalBody::new(10., 10.),
                thruster,
                mesh: mesh.clone(),
            },
            Transform::pos(Vec3::new(0.0, 0.0, -10.0)),
        );
    });

    entity_manager.add(Starlight {
        light: Light::DirectionalLight(
            Vec3::new(0.0, -1.0, 0.0),
            LightColor {
                ambient: Vec3::new(0.01, 0.01, 0.01),
                diffuse: Vec3::new(0.1, 0.1, 0.1),
                specular: Vec3::new(0.3, 0.3, 0.3),
            },
        ),
    });

    entity_manager.add(SpaceBox { renderer: skybox });

    Ok(())
}

fn create_asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) {
}

fn create_thruster(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) -> ParticleEmitter {
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
fn create_lights(entity_manager: &mut EntityManager, resource_manager: &mut SceneResourceManager) {}
