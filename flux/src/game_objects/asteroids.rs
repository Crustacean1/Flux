use atlas::{
    components::{
        physical_body::PhysicalBody, skybox_renderer::SkyboxRenderer, text_renderer::TextRenderer,
        transform::Transform,
    },
    entity_manager::{EntityManager, EntityManagerTrait},
    game_entities::{
        enemy_ship::EnemyShip, space_box::SpaceBox, starlight::Starlight, ui_label::UiLabel,
    },
    game_root::GameError,
    graphics::{
        lights::{Light, LightColor},
        material::{phong_material::PhongMaterial, skybox_material::SkyboxMaterial},
        mesh::Mesh,
        shaders::mesh_shader::MeshShader,
    },
    resource_manager::{font::Font, scene_resource_manager::SceneResourceManager, ResourceManager},
};
use glam::Vec3;

pub fn asteroids(
    entity_manager: &mut EntityManager,
    resource_manager: &mut SceneResourceManager,
) -> Result<(), GameError> {
    let font: Font = resource_manager.get("main").res;

    let skybox_material: SkyboxMaterial = resource_manager.get("space1").res;
    let skybox = SkyboxRenderer::new(1.0, skybox_material);

    let meshes = ["spaceship3"];

    let meshes: Vec<Mesh<MeshShader, PhongMaterial>> = meshes
        .iter()
        .map(|mesh| {
            return resource_manager.get(mesh).res;
        })
        .collect();

    meshes.iter().enumerate().for_each(|(i, mesh)| {
        let position = Vec3::new(0.0, 10.0 * i as f32, 0.0);
        let scale = Vec3::new(1., 1., 1.);
        let rotation = Vec3::new(0.0, 0.0, 0.0);

        entity_manager.add_at(
            EnemyShip {
                physical_body: PhysicalBody::new(10., 10.),
                mesh: mesh.clone(),
            },
            Transform::pos(Vec3::new(0.0, 0.0, -10.0)),
        );

        entity_manager.add_at(
            EnemyShip {
                physical_body: PhysicalBody::new(10., 10.),
                mesh: mesh.clone(),
            },
            Transform::pos(Vec3::new(0.0, 0.0, 10.0)),
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

    entity_manager.add(UiLabel {
        renderer: TextRenderer::new("Velocity: 18.31 [m/s]", font),
    });

    entity_manager.add(SpaceBox { renderer: skybox });

    Ok(())
}
