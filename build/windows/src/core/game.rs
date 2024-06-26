use std::{
    any::TypeId,
    fmt::format,
    ops::Deref,
    ptr::null,
    sync::{Arc, RwLock},
};

use env_logger::fmt::Color;
use log::debug;
use nalgebra::{Point3, Vector3, Vector4};
use thunderdome::Index;
use QcCore::{
    ecs::{
        component::{Component, Named, V8},
        components::{
            camera::Camera, light::Light, material_render::MaterialRender, mesh_render::MeshRender,
            skybox::SkyBox, transform::Transform,
        },
        game_object::GameObject,
    },
    resources::{
        material::Material,
        project::{ProjectConfig, ProjectConfigFile},
    },
    scene_system::scene::Scene,
};
use QcMacros::Comp;
use QcRender::resources::{Mesh, UniformInfo};
use QcRender::{
    gl,
    resources::{Model, Texture},
};
use QcScript::{core::JsComponent, utils::GoExt, v8};
use QcTools::time::clock::Clock;
use QcUI::{
    component::{
        Button, ButtonMessage, Canvas, FlexDirection, Grid, Image, ImageLoader, Label, Panel,
        TextBox, ToUi, UiNode, Widget,
    },
    core::uiBind::{JsUiBind, UiBind},
    egui,
    message::UiMessageType,
    panel, Align2, Color32, Margin, Pos2, RetainedImage, Vec2,
};
use QcWindowing::{event::WindowEvent, event_loop::ControlFlow, Window};

use super::{context::Context, game_render::GameRender};

pub struct Game {
    gameRender: Arc<GameRender>,
    context: Arc<Context>,
    elapsed: f32,
    debugDraw: Canvas,
    fps: Index,
}

impl Game {
    ///初始化场景(暂时)
    //initialize scene (temporary)
    pub fn createScene(context: Arc<Context>) {
        let mut sceneManagerRef = context.sceneManager.try_write().unwrap();

        let resource_manager = context.resourceManager.clone();

        let scene = resource_manager.get_string("scene/main.scene");
        if let Some(scene) = scene {
            sceneManagerRef.load_scene_from_str(&scene, context.resourceManager.clone());
        }

        // {
        //     let currentScene = sceneManagerRef.get_current_scene_mut();
        //     if let Some(currentScene) = currentScene {
        //         let camera = Component::Camera(Camera::new());
        //         let mut skybox = SkyBox::new();

        //         let transform = Component::Transform(Transform::new(Point3::new(0., 0., 0.)));
        //         let mut obj = GameObject::new("Camera");
        //         obj.insert(camera);
        //         obj.insert(Component::SkyBox(skybox));
        //         obj.insert(transform);
        //         // obj.insert(Component::new(Example));
        //         let objId = currentScene.add_child(obj);

        //         let mut parent = GameObject::default();
        //         let transform = Component::Transform(Transform::new(Point3::new(0., 0., -5.)));
        //         let tf = parent.addComponent(transform);

        //         // parent.addComponent(component)

        //         let parent = currentScene.add_child(parent);

        //         //添加平面
        //         //add plane
        //         {
        //             let obj = GameObject::new("Plane");
        //             let objId = currentScene.add_child_with_parent(obj, Some(parent));
        //             let obj = &mut currentScene[objId];

        //             let mut transform = Transform::new(Point3::new(0., -1., 0.));

        //             transform.set_scale(Vector3::new(5., 1., 5.));
        //             let mut meshRender = MeshRender::new();
        //             let mut model = Mesh::plane();
        //             model.setMaterialIndex(0);

        //             meshRender.addModel(model.into());

        //             let mut materialRender = MaterialRender::new();
        //             let mut material = Material::default();

        //             let image = context.resourceManager.get("shitou.dds").unwrap();
        //             let texture = Texture::new(image);
        //             material.addTexture(texture);
        //             materialRender.addMaterial(material);
        //             obj.addComponent(Component::Transform(transform));
        //             obj.addComponent(Component::MeshRender(meshRender));
        //             obj.addComponent(Component::MaterialRender(materialRender));
        //         }

        //         //添加猴头
        //         //add monkey head
        //         let obj = {
        //             let obj = GameObject::new("Monkey");
        //             let objId = currentScene.add_child_with_parent(obj, Some(parent));
        //             let obj = &mut currentScene[objId];

        //             let mut transform = Transform::new(Point3::new(0., 0., 0.));

        //             let mut meshRender = MeshRender::new();
        //             let mut model = Mesh::new("monkey.mesh");
        //             model.setMaterialIndex(0);

        //             meshRender.addModel(model.into());

        //             let mut materialRender = MaterialRender::new();
        //             let mut material = Material::default();
        //             let image = context.resourceManager.get("texture.dds").unwrap();
        //             let texture = Texture::new(image);
        //             material.addTexture(texture);
        //             materialRender.addMaterial(material);
        //             obj.addComponent(Component::Transform(transform));
        //             obj.addComponent(Component::MeshRender(meshRender));
        //             obj.addComponent(Component::MaterialRender(materialRender));
        //             obj
        //         };

        //         {
        //             let mut canvas = Canvas::new();

        //             let mut panel = Panel::new(
        //                 Widget::default()
        //                     .with_background(Color32::YELLOW)
        //                     .with_margin(Margin::symmetric(100., 100.))
        //                     .with_width(400.)
        //                     .with_height(400.),
        //             )
        //             .with_orientation(FlexDirection::Row)
        //             .with_spacing(100.);

        //             {
        //                 let mut panel0: Panel = Panel::new(
        //                     Widget::default()
        //                         // .with_background(Color32::BLACK)
        //                         .with_padding(Margin::same(10.)),
        //                 )
        //                 .with_orientation(FlexDirection::Column)
        //                 .with_spacing(20.);

        //                 let mut panel1: Panel = Panel::new(
        //                     Widget::default()
        //                         .with_background(Color32::BLACK)
        //                         .with_padding(Margin::same(10.)),
        //                 )
        //                 // .with_orientation(FlexDirection::Column)
        //                 .with_spacing(20.);

        //                 let button = Button::default().with_text("OnClick");
        //                 let index = button.uuid;
        //                 panel1.addChild(UiNode::new(button));

        //                 // let cube = JsComponent::new("Cube", None);
        //                 // let compId = obj.insert(Component::Other(Box::new(cube)));
        //                 // canvas.addUiBind(
        //                 //     index,
        //                 //     UiBind::JsBind(JsUiBind::new(
        //                 //         objId,
        //                 //         compId,
        //                 //         "onClick".to_string(),
        //                 //         UiMessageType::ButtonMessage(ButtonMessage::Clicked),
        //                 //     )),
        //                 // );

        //                 let button = Button::default().with_text("确定");
        //                 panel1.addChild(UiNode::new(button));
        //                 let button = Button::default().with_text("确定");
        //                 panel1.addChild(UiNode::new(button));
        //                 let button = Button::default().with_text("确定");
        //                 panel1.addChild(UiNode::new(button));
        //                 panel0.addChild(panel1.toUi());

        //                 let image = context.resourceManager.get("user.jpg").unwrap();

        //                 let grid = Grid::default()
        //                     .with_columns(2)
        //                     .with_spacing(Vec2::new(20., 20.))
        //                     .with_children(vec![
        //                         Image::default()
        //                             .with_texture(RetainedImage::load_texture(&image))
        //                             .toUi(),
        //                         Image::default()
        //                             .with_texture(RetainedImage::load_texture(&image))
        //                             .toUi(),
        //                         Image::default()
        //                             .with_texture(RetainedImage::load_texture(&image))
        //                             .toUi(),
        //                         Image::default()
        //                             .with_texture(RetainedImage::load_texture(&image))
        //                             .toUi(),
        //                         Image::default()
        //                             .with_texture(RetainedImage::load_texture(&image))
        //                             .toUi(),
        //                         Image::default()
        //                             .with_texture(RetainedImage::load_texture(&image))
        //                             .toUi(),
        //                     ]);
        //                 panel0.addChild(grid.toUi());
        //                 //     let imgFile = RetainedImage::from_image_bytes(
        //                 //         "user.jpg",
        //                 //         include_bytes!("../../assets/user.jpg"),
        //                 //     )
        //                 //     .unwrap();
        //                 //     let mut image = Image::default().with_texture("user.jpg", Some(imgFile));

        //                 //     panel1.addChild(UiNode::new(image));

        //                 panel.addChild(UiNode::new(panel0));
        //             }
        //             // {
        //             //     let mut panel1 =
        //             //         Panel::new(Widget::default().with_background(Color32::LIGHT_BLUE))
        //             //             .with_orientation(FlexDirection::Column)
        //             //             .with_spacing(20.);
        //             //     let button = Button::default().with_text("确定");
        //             //     panel1.addChild(UiNode::new(button));
        //             //     let button = Button::default().with_text("确定");
        //             //     panel1.addChild(UiNode::new(button));
        //             //     let button = Button::default().with_text("确定");
        //             //     panel1.addChild(UiNode::new(button));
        //             //     let button = Button::default().with_text("确定");
        //             //     panel1.addChild(UiNode::new(button));
        //             //     panel.addChild(UiNode::new(panel1));
        //             // }

        //             {
        //                 let mut panel1 =
        //                     Panel::new(Widget::default().with_background(Color32::LIGHT_RED))
        //                         .with_orientation(FlexDirection::Column)
        //                         .with_spacing(20.);

        //                 let createTextbox = |align: Align2| {
        //                     TextBox::new(
        //                         Widget::default()
        //                             .with_height(100.)
        //                             .with_width(100.)
        //                             .with_align(align),
        //                     )
        //                     .with_text("确定")
        //                 };

        //                 let textbox = createTextbox(Align2::LEFT_TOP);
        //                 panel1.addChild(UiNode::new(textbox));
        //                 let textbox = createTextbox(Align2::LEFT_CENTER);
        //                 panel1.addChild(UiNode::new(textbox));
        //                 let textbox = createTextbox(Align2::LEFT_BOTTOM);
        //                 panel1.addChild(UiNode::new(textbox));
        //                 let textbox = createTextbox(Align2::CENTER_TOP);
        //                 panel1.addChild(UiNode::new(textbox));
        //                 panel.addChild(UiNode::new(panel1));
        //             }

        //             canvas.addChild(UiNode::new(panel));

        //             // let imgFile = RetainedImage::from_image_bytes(
        //             //     "user.jpg",
        //             //     include_bytes!("../../assets/user.jpg"),
        //             // )
        //             // .unwrap();

        //             let image = context.resourceManager.get("user.jpg").unwrap();
        //             let img = RetainedImage::load_texture(&image);
        //             let image = Image::default().with_texture(img);

        //             canvas.addChild(UiNode::new(image));
        //             obj.insert(Component::Other(Box::new(canvas)));
        //         }

        //         // obj.addComponent(Component::new(cube));
        //         // println!("{:#?}", currentScene);

        // let mut scene = Scene::new();
        // let camera = Component::Camera(Camera::new());
        // let mut skybox = SkyBox::new();

        // let transform = Component::Transform(Transform::new(Point3::new(0., 0., 0.)));
        // let mut obj = GameObject::new("Camera");
        // obj.insert(camera);
        // obj.insert(Component::SkyBox(skybox));
        // obj.insert(transform);
        // scene.add_child(obj);
        // println!("{}", scene.save());
        //     }
        // }

        let mut jsManager = context.jsRuntimeManager.try_write().unwrap();

        let scope = &mut jsManager.handle_scope();

        let context = scope.get_current_context();

        let global = context.global(scope);

        let currentScene = sceneManagerRef.get_current_scene_mut().unwrap();

        for (_, go) in currentScene.iter_mut() {
            for (_, comp) in go.iter_mut() {
                if comp.type_id() == TypeId::of::<JsComponent>() {
                    let jsComp = comp.cast_mut::<JsComponent>().unwrap();

                    let jsValue = {
                        let objName =
                            v8::String::new(scope, &format!("##{}##", jsComp.getName())).unwrap();
                        let obj = global.get(scope, objName.into()).unwrap();

                        let obj = v8::Local::<v8::Function>::try_from(obj).unwrap();

                        let undefined = v8::undefined(scope);
                        let obj = obj.call(scope, undefined.into(), &[]).unwrap();

                        obj
                    };

                    jsComp.setValue(Some(v8::Global::new(scope, jsValue).into()))
                }
            }
        }

        for (_, go) in currentScene.iter_mut() {
            let name = go.getName().to_string();
            for (_, comp) in go.iter_mut() {
                if comp.type_id() == TypeId::of::<JsComponent>() {
                    let jsComp = comp.cast_mut::<JsComponent>().unwrap();

                    let comp = v8::Local::<v8::Value>::new(scope, jsComp.getV8Value());
                    GoExt::setParentName(comp, scope, &name);
                    GoExt::onStart(comp, scope);
                }
            }
        }
    }

    pub fn load_game(context: Arc<Context>, scene: String) {
        let mut scene_manager = context.sceneManager.try_write().unwrap();
        let resource_manager = context.resourceManager.clone();
        let scene = resource_manager.get_string(&scene);
        if let Some(scene) = scene {
            scene_manager.load_scene_from_str(&scene, context.resourceManager.clone());
        }
        let currentScene = scene_manager.get_current_scene_mut().unwrap();

        let mut jsRuntimeManager = context.jsRuntimeManager.try_write().unwrap();
        jsRuntimeManager
            .op_state()
            .borrow_mut()
            .put(context.sceneManager.clone());

        jsRuntimeManager
            .op_state()
            .borrow_mut()
            .put(context.resourceManager.clone());

        jsRuntimeManager
            .op_state()
            .borrow_mut()
            .put(context.audio_engine.clone());

        jsRuntimeManager
            .op_state()
            .borrow_mut()
            .put(context.uiManager.clone());

        jsRuntimeManager
            .op_state()
            .borrow_mut()
            .put(currentScene as *mut Scene);

        jsRuntimeManager
            .op_state()
            .borrow_mut()
            .put(jsRuntimeManager.v8_isolate() as *mut _);

        let code = resource_manager.get_string("dist/quincy.js");
        if let Some(code) = code {
            jsRuntimeManager
                .execute_script_static("qc", code.leak())
                .unwrap();
        }

        let scope = &mut jsRuntimeManager.handle_scope();

        let context = scope.get_current_context();

        let global = context.global(scope);

        for (_, go) in currentScene.iter_mut() {
            for (_, comp) in go.iter_mut() {
                if comp.type_id() == TypeId::of::<JsComponent>() {
                    let jsComp = comp.cast_mut::<JsComponent>().unwrap();

                    let jsValue = {
                        let objName =
                            v8::String::new(scope, &format!("##{}##", jsComp.getName())).unwrap();
                        let obj = global.get(scope, objName.into()).unwrap();

                        let obj = v8::Local::<v8::Function>::try_from(obj).unwrap();

                        let undefined = v8::undefined(scope);
                        let obj = obj.call(scope, undefined.into(), &[]).unwrap();

                        obj
                    };
                   
                    jsComp.setValue(Some(v8::Global::new(scope, jsValue).into()))
                }
            }
        }

        for (_, go) in currentScene.iter_mut() {
            let name = go.getName().to_string();
            for (_, comp) in go.iter_mut() {
                if comp.type_id() == TypeId::of::<JsComponent>() {
                    let jsComp = comp.cast_mut::<JsComponent>().unwrap();

                    let comp = v8::Local::<v8::Value>::new(scope, jsComp.getV8Value());
                    GoExt::setParentName(comp, scope, &name);
                    GoExt::onStart(comp, scope);
                }
            }
        }
    }

    pub fn test_ui(context: Arc<Context>, scene: &mut Scene) {
        let camera = Component::Camera(Camera::new());
        let mut skybox = SkyBox::new();

        let transform = Component::Transform(Transform::new(Point3::new(0., 2., 0.)));
        let mut obj = GameObject::new("Camera");
        obj.insert(camera);
        obj.insert(Component::SkyBox(skybox));
        obj.insert(transform);
        let compId = obj.insert(Component::Other(Box::new(JsComponent::new(
            "UiTest3", None,
        ))));
        let objId = scene.add_child(obj);

        let mut obj = GameObject::new("Canvas");
        obj.addComponent(Component::Other(Box::new(Canvas::new())));
        scene.add_child(obj);
        println!("{}", scene.save());

        let currentScene = scene;

        let index = currentScene.get_main_canvas().unwrap();
        let canvas = currentScene[index].getComponentMut::<Canvas>().unwrap();

        let widget = Button::default().widget.with_position(Pos2::new(50., 50.));
        let mut button = Button::new(widget);
        button.set_name("Hello");

        canvas.addUiBind(
            button.uuid,
            UiBind::JsBind(JsUiBind::new(
                objId,
                compId,
                "btnClicked",
                UiMessageType::ButtonMessage(ButtonMessage::Clicked),
            )),
        );

        let parent = canvas.add_child(button.toUi());

        let widget = Button::default().widget.with_position(Pos2::new(200., 50.));
        let button = Button::new(widget);

        canvas.add_child_with_parent(button.toUi(), Some(parent));

        let widget = Panel::default().widget.with_position(Pos2::new(400., 50.));
        let panel = Panel::new(widget);
        let parent = canvas.add_child(panel.toUi());

        let button = Button::default();
        canvas.add_child_with_parent(button.toUi(), Some(parent));

        let mut label = Label::default();
        label.set_name("Label");
        canvas.add_child_with_parent(label.toUi(), Some(parent));

        let button = TextBox::default();
        canvas.add_child_with_parent(button.toUi(), Some(parent));
    }

    pub fn test_light(context: Arc<Context>, scene: &mut Scene) {
        let mut light = GameObject::new("Light");

        light.addComponent(Component::Light(Light::default()));
        let transform = Transform::new(Point3::new(0., 3., -3.));
        light.addComponent(Component::Transform(transform));

        scene.add_child(light);

        let mut cube = GameObject::new("Cube");
        let mut mesh_render = MeshRender::new().with_mesh(Mesh::cube());
        cube.addComponent(Component::MeshRender(mesh_render));

        let mut material_render = MaterialRender::new().with_material(Material::default());
        cube.addComponent(Component::MaterialRender(material_render));

        let mut transform = Transform::new(Point3::new(0., 0., -3.));
        cube.addComponent(Component::Transform(transform));

        scene.add_child(cube);
    }

    pub fn new(context: Arc<Context>, scene: Option<String>) -> Self {
        // {
        //     let mut scene = Scene::new();

        //     Self::test_ui(context.clone(), &mut scene);

        //     Self::test_light(context.clone(), &mut scene);

        //     let mut scene_manager = context.sceneManager.try_write().unwrap();
        //     scene_manager.load_scene(scene);
        // }

        let scene_path = if let Some(scene) = scene {
            scene
        } else {
            let config = context
                .resourceManager
                .get_json::<ProjectConfigFile>("project.json")
                .unwrap();

            config.scene
        };
        Self::load_game(context.clone(), scene_path);

        //物理引擎初始化
        {
            let mut physics_engine = context.physics_engine.try_write().unwrap();
            let scene_manager = context.sceneManager.try_write().unwrap();
            if let Some(scene) = scene_manager.get_current_scene() {
                physics_engine.init(scene);
            }
        }

        let fps = Label::new(Widget::default().with_foreground(Color32::RED))
            .with_text("fps")
            .with_selectable(false);

        let mut canvas = Canvas::new();
        let index = canvas.add_child(UiNode::Label(fps));

        let gameRender = GameRender::new(context.clone());
        Self {
            gameRender,
            context,
            elapsed: 0.,
            fps: index,
            debugDraw: canvas,
        }
    }
    pub fn preUpdate(&self, event: &WindowEvent) {
        let window = self.context.window.try_read().unwrap();

        let result = self
            .context
            .uiManager
            .try_write()
            .unwrap()
            .handleEvent(&window, event);

        window.request_redraw();

        match event {
            WindowEvent::MouseInput { state, .. } => {}
            _ => {
                if result.consumed {
                    return;
                }
            }
        }

        let jsManager = self.context.jsRuntimeManager.clone();
        self.context
            .inputManager
            .try_write()
            .unwrap()
            .handleEvent(event, &mut jsManager.try_write().unwrap().handle_scope());
    }

    pub fn update(&mut self, clock: &Clock) {
        self.elapsed += clock.getDeltaTime();
        if self.elapsed > 1. {
            if let UiNode::Label(label) = &mut self.debugDraw[self.fps] {
                label.set_text(&format!("fps: {}", clock.getFrameRate()));
            }

            self.elapsed = 0.;
        }

        let window = self.context.window.try_read().unwrap();
        {
            let mut jsRuntimeManager = self.context.jsRuntimeManager.try_write().unwrap();
            let jsRuntime = jsRuntimeManager.main_realm();

            let mut sceneManager = self.context.sceneManager.try_write().unwrap();
            let currentScene = sceneManager.get_current_scene_mut().unwrap();

            jsRuntimeManager
                .op_state()
                .borrow_mut()
                .put::<*mut Scene>(currentScene);

            currentScene.update(
                clock.getDeltaTime(),
                jsRuntime,
                jsRuntimeManager.v8_isolate(),
            );
        }

        {
            let mut physics_engine = self.context.physics_engine.try_write().unwrap();
            physics_engine.update();

            let mut sceneManager = self.context.sceneManager.try_write().unwrap();
            let currentScene = sceneManager.get_current_scene_mut().unwrap();

            physics_engine.post_update(currentScene);
        }

        //渲染游戏场景
        //render game scene
        self.gameRender.renderScene();

        //渲染游戏ui
        //render game ui
        {
            let mut jsRuntimeManager = self.context.jsRuntimeManager.try_write().unwrap();

            let mut sceneManager = self.context.sceneManager.try_write().unwrap();
            let currentScene = sceneManager.get_current_scene_mut().unwrap();

            let mut canvasList = vec![&mut self.debugDraw];
            if let Some(index) = currentScene.get_main_canvas() {
                let canvas = currentScene[index].getComponentMut::<Canvas>().unwrap();

                canvasList.push(canvas);
            }

            let mut uiManager = self.context.uiManager.try_write().unwrap();

            uiManager.render(&window, &mut canvasList);

            uiManager.update(canvasList, &mut jsRuntimeManager.handle_scope());
        }

        // 执行脚本系统更新
        {
            let mut js_runtime_manager = self.context.jsRuntimeManager.try_write().unwrap();
            js_runtime_manager.update();
        }
    }

    pub fn postUpdate(&self) {
        self.context.device.swapBuffers();
    }

    pub fn destory(&mut self) {
        let mut ui = self.context.uiManager.try_write().unwrap();
        ui.destory();
    }
}
