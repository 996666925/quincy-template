use log::info;
use nalgebra::{Matrix4, Point3, Rotation3, Vector3};
use std::sync::Arc;
use QcCore::{
    ecs::components::{camera::Camera, skybox::SkyBox},
    resources::material::Material,
    scene_system::scene::Scene,
};

use super::context::Context;
use QcCore::ecs::components::transform::Transform;
use QcRender::{
    gl,
    resources::{Mesh, Model},
};
use QcTools::utils::r#ref::Ref;

pub struct GameRender {
    context: Arc<Context>,
}

impl GameRender {
    pub fn new(context: Arc<Context>) -> Arc<GameRender> {
        Arc::new(Self { context })
    }

    pub fn renderScene(&self) {
        let mut sceneManager = self.context.sceneManager.try_write().unwrap();
        let mut window = self.context.window.try_read().unwrap();
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());
        let currnetScene = sceneManager
            .get_current_scene_mut()
            .expect("无法获取当前的场景对象");

        self.update_lights(currnetScene);
        

        if let Some(cameraObj) = currnetScene.get_main_camera() {
            let transform = currnetScene[cameraObj].getComponent::<Transform>().unwrap();

            let mut camera = currnetScene[cameraObj]
                .getComponent::<Camera>()
                .cloned()
                .unwrap();

            let position = transform.get_world_position(&currnetScene);
            let rotation = transform.get_rotation();
            camera.cacheMatrices(size.width, size.height, &position.into(), &rotation);
            camera.updateUBO(self.context.engineUBO.clone(), &position);

            let local_matrix = transform.get_world_position_matrix(&currnetScene)
                * Matrix4::new_scaling(camera.far / 2f32.sqrt());

            self.context
                .engineUBO
                .setSubData(0, local_matrix.as_slice());

            let renderer = self.context.renderer.try_read().unwrap();
            renderer.setClearColor(0.66, 0.66, 0.66, 1.);
            renderer.clear(true, true, false);

            {
                currnetScene
                    .get_main_skybox()
                    .map(|skybox: thunderdome::Index| {
                        let skybox = currnetScene[skybox].getComponent::<SkyBox>().unwrap();

                        renderer.renderSkybox(skybox, self.context.engineUBO.clone());
                    });
            }

            renderer.renderScene(currnetScene, self.context.engineUBO.clone());
        };
    }

    fn update_lights(&self, scene: &mut Scene) {
        let renderer = self.context.renderer.try_read().unwrap();
        let light_matrices = renderer.findLightMatrices(scene);

        let lightSSBO = self.context.lightSSBO.clone();


        if !light_matrices.is_empty() {
            lightSSBO.send_blocks(light_matrices.as_slice());
        }
    }
}
