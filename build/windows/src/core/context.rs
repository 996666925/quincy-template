use nalgebra::{Matrix1, Matrix4, Vector3};
use std::mem::size_of;
use std::sync::{Arc, RwLock};
use QcAudio::core::audio_engine::AudioEngine;
use QcCore::ecs::components::camera::Camera;
use QcCore::ecs::game_object::GameObject;
use QcCore::ecs::renderer::Renderer;
use QcCore::ecs::MvpUbo;
use QcCore::ecs::{component::Component, components::transform::Transform};
use QcCore::resources::ResourceManager;
use QcPhysics::core::physics_engine::PhysicsEngine;

use QcCore::scene_system::scene_manager::SceneManager;
use QcRender::buffers::{AccessSpecifier, ShaderStorageBuffer, UniformBuffer};
use QcRender::settings::driver_settings::DriverSettings;
use QcScript::core::JsRuntimeManager;
use QcTools::utils::r#ref::Ref;
use QcUI::core::ui_manager::UiManager;
use QcWindowing::{
    context::device::Device, event_loop::EventLoop, input::input_manager::InputManager,
    settings::DeviceSettings, window::QcWindow,
};

pub struct Context {
    pub device: Device,
    pub inputManager: Ref<InputManager>,
    pub uiManager: Ref<UiManager>,
    pub window: Ref<QcWindow>,
    pub renderer: Ref<Renderer>,
    pub sceneManager: Ref<SceneManager>,
    pub engineUBO: Arc<MvpUbo>,
    pub jsRuntimeManager: Ref<JsRuntimeManager>,
    pub resourceManager: Arc<ResourceManager>,
    pub audio_engine: Ref<AudioEngine>,
    pub lightSSBO: Arc<ShaderStorageBuffer>,
    pub physics_engine: Ref<PhysicsEngine>,
}

impl Context {
    pub fn new(window: Ref<QcWindow>, el: &EventLoop<()>) -> Arc<Context> {
        let inputManager = InputManager::new();
        let window_ref = window.clone();
        let window_read = window_ref.try_read().unwrap();
        let device = Device::new(&window_read, DeviceSettings::default());
        let uiManager = UiManager::new(&window_read, el);
        let renderer = Renderer::new(DriverSettings::default());
        let sceneManager = SceneManager::new();
        let engineUBO = Arc::new(UniformBuffer::new(6));
        let jsRuntimeManager = Ref::new(JsRuntimeManager::new());
        let resourceManager = Arc::new(ResourceManager::new());
        let audio_engine = Ref::new(AudioEngine::new());

        let lightSSBO = Arc::new(ShaderStorageBuffer::new(
            AccessSpecifier::DYNAMIC_STORAGE_BIT,
        ));
        lightSSBO.bind(0);

        let physics_engine = Ref::new(PhysicsEngine::new());

        Arc::new(Self {
            device,
            inputManager,
            uiManager,
            window,
            renderer,
            sceneManager,
            engineUBO,
            jsRuntimeManager,
            resourceManager,
            audio_engine,
            lightSSBO,
            physics_engine,
        })
    }
}
