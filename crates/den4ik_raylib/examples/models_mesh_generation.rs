use den4ik_raylib::{
    RaylibHandle,
    color::{Color, fade},
    container::{Container, ContainerId},
    core::{
        CAMERA_ORBITAL, Camera, KEY_LEFT, KEY_RIGHT, MOUSE_BUTTON_LEFT, Vector3, begin_drawing,
        begin_mode_3d, clear_background, draw_grid, draw_rectangle, draw_rectangle_lines,
        draw_text, end_drawing, end_mode_3d, is_key_pressed, is_mouse_button_pressed,
        update_camera,
    },
    image::{Image, ImageId},
    material::MATERIAL_MAP_DIFFUSE,
    mesh::{Mesh, MeshConfigBuilder, MeshId},
    model::{Model, ModelId},
    texture::{Texture2D, Texture2DId},
};

const NUM_MODELS: usize = 9;

fn gen_mesh_custom(handle: &mut RaylibHandle) -> MeshId {
    let config = MeshConfigBuilder::new(3, 1).with_normals().build().unwrap();
    let mesh_id = Mesh::new(handle, config).unwrap();
    {
        let mesh = handle.meshes.get_mut(mesh_id).unwrap();
        mesh.get_vertices_mut()[0] = [0.0, 0.0, 0.0];
        mesh.get_normals_mut()[0] = [0.0, 1.0, 0.0];
        mesh.get_texcoords_mut()[0] = [0.0, 0.0];
        mesh.get_vertices_mut()[1] = [1.0, 0.0, 2.0];
        mesh.get_normals_mut()[1] = [0.0, 1.0, 0.0];
        mesh.get_texcoords_mut()[1] = [0.5, 1.0];
        mesh.get_vertices_mut()[2] = [2.0, 0.0, 0.0];
        mesh.get_normals_mut()[2] = [0.0, 1.0, 0.0];
        mesh.get_texcoords_mut()[2] = [1.0, 0.0];
        mesh.upload(handle, false);
    }
    mesh_id
}

fn get_texture(handle: &mut RaylibHandle) -> Texture2DId {
    let img_id: ImageId = Image::gen_checked(handle, 2, 2, 1, 1, Color::RED, Color::GREEN);
    let texture_id: Texture2DId = Texture2D::load_from_image(handle, img_id).unwrap();
    <RaylibHandle as Container<Image, ImageId>>::remove(handle, img_id);
    texture_id
}

fn get_models(handle: &mut RaylibHandle) -> Vec<ModelId> {
    let texture_id = get_texture(handle);
    let mut models = Vec::new();
    let mesh_id = Mesh::gen_plane(handle, 2.0, 2.0, 4, 3).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_cube(handle, 2.0, 1.0, 2.0).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_sphere(handle, 2.0, 32, 32).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_hemisphere(handle, 2.0, 16, 16).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_cylinder(handle, 1.0, 2.0, 16).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_torus(handle, 0.25, 4.0, 16, 32).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_knot(handle, 1.0, 2.0, 16, 128).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = Mesh::gen_poly(handle, 5, 2.0).unwrap();
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    let mesh_id = gen_mesh_custom(handle);
    models.push(Model::load_from_mesh(handle, mesh_id).unwrap());
    for model_id in &models {
        handle.set_model_material_texture(*model_id, 0, MATERIAL_MAP_DIFFUSE as i32, texture_id);
    }
    models
}

fn main() {
    let mut handle = RaylibHandle::new(800, 450, "raylib [models] example - mesh generation");
    let models = get_models(&mut handle);
    let mut camera = Camera {
        position: Vector3 {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        },
        target: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        up: Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        fovy: 45.0,
        projection: 0,
    };
    let position = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut current_model: usize = 0;
    handle.set_target_fps(60);

    while !handle.window_should_close() {
        update_camera(&mut camera, CAMERA_ORBITAL as i32);

        if is_mouse_button_pressed(MOUSE_BUTTON_LEFT as i32) {
            current_model = (current_model + 1) % models.len();
        }

        if is_key_pressed(KEY_RIGHT as i32) {
            current_model += 1;
            if current_model >= models.len() {
                current_model = 0;
            }
        } else if is_key_pressed(KEY_LEFT as i32) {
            if current_model == 0 {
                current_model = models.len() - 1;
            } else {
                current_model -= 1;
            }
        }

        begin_drawing();
        clear_background(Color::RAYWHITE);

        begin_mode_3d(camera);
        handle
            .models
            .get(models[current_model as usize])
            .unwrap()
            .draw(position, 1.0, Color::WHITE);
        draw_grid(10, 1.0);
        end_mode_3d();

        draw_rectangle(30, 400, 310, 30, fade(Color::SKYBLUE, 0.5));
        draw_rectangle_lines(30, 400, 310, 30, fade(Color::DARKBLUE, 0.5));
        draw_text(
            "MOUSE LEFT BUTTON to CYCLE PROCEDURAL MODELS",
            40,
            410,
            10,
            Color::BLUE,
        );

        match current_model {
            0 => draw_text("PLANE", 680, 10, 20, Color::DARKBLUE),
            1 => draw_text("CUBE", 680, 10, 20, Color::DARKBLUE),
            2 => draw_text("SPHERE", 680, 10, 20, Color::DARKBLUE),
            3 => draw_text("HEMISPHERE", 640, 10, 20, Color::DARKBLUE),
            4 => draw_text("CYLINDER", 680, 10, 20, Color::DARKBLUE),
            5 => draw_text("TORUS", 680, 10, 20, Color::DARKBLUE),
            6 => draw_text("KNOT", 680, 10, 20, Color::DARKBLUE),
            7 => draw_text("POLY", 680, 10, 20, Color::DARKBLUE),
            8 => draw_text("Custom (triangle)", 580, 10, 20, Color::DARKBLUE),
            _ => {}
        }

        end_drawing();
    }
}
