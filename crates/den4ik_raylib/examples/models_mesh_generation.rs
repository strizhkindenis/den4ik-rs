use den4ik_raylib::prelude::*;

fn gen_mesh_custom(handle: &mut RaylibHandle) -> MeshId {
    let config = den4ik_raylib::mesh::MeshConfigBuilder::new(3, 1)
        .with_normals()
        .build()
        .unwrap();
    let mesh_id = handle.load_mesh(config).unwrap();
    handle.get_mesh_vertices_mut(mesh_id).unwrap()[0] = [0.0, 0.0, 0.0];
    handle.get_mesh_normals_mut(mesh_id).unwrap()[0] = [0.0, 1.0, 0.0];
    handle.get_mesh_texcoords_mut(mesh_id).unwrap()[0] = [0.0, 0.0];
    handle.get_mesh_vertices_mut(mesh_id).unwrap()[1] = [1.0, 0.0, 2.0];
    handle.get_mesh_normals_mut(mesh_id).unwrap()[1] = [0.0, 1.0, 0.0];
    handle.get_mesh_texcoords_mut(mesh_id).unwrap()[1] = [0.5, 1.0];
    handle.get_mesh_vertices_mut(mesh_id).unwrap()[2] = [2.0, 0.0, 0.0];
    handle.get_mesh_normals_mut(mesh_id).unwrap()[2] = [0.0, 1.0, 0.0];
    handle.get_mesh_texcoords_mut(mesh_id).unwrap()[2] = [1.0, 0.0];
    handle.upload_mesh(mesh_id, false).unwrap();
    mesh_id
}

fn get_texture(handle: &mut RaylibHandle) -> Texture2DId {
    let img_id = handle.gen_image_checked(2, 2, 1, 1, Color::RED, Color::GREEN);
    let texture_id = handle.load_texture_from_image(img_id).unwrap();
    handle.remove_image(img_id).unwrap();
    texture_id
}

fn get_models(handle: &mut RaylibHandle) -> Vec<ModelId> {
    let texture_id = get_texture(handle);
    let mut models = Vec::new();

    let mesh_id = handle.gen_mesh_plane(2.0, 2.0, 4, 3);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_cube(2.0, 1.0, 2.0);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_sphere(2.0, 32, 32);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_hemisphere(2.0, 16, 16);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_cylinder(1.0, 2.0, 16);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_torus(0.25, 4.0, 16, 32);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_knot(1.0, 2.0, 16, 128);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = handle.gen_mesh_poly(5, 2.0);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());
    let mesh_id = gen_mesh_custom(handle);
    models.push(handle.load_model_from_mesh(mesh_id).unwrap());

    for &model_id in &models {
        handle
            .set_model_material_texture(model_id, 0, MaterialMap::Diffuse, texture_id)
            .unwrap();
    }
    models
}

fn main() {
    let mut handle = RaylibHandle::new(800, 450, "raylib [models] example - mesh generation");
    let models = get_models(&mut handle);
    let mut camera = Camera::new(
        Vector3::new(5.0, 5.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
        0,
    );
    let position = Vector3::new(0.0, 0.0, 0.0);
    let mut current_model: usize = 0;
    handle.set_target_fps(60);

    while !handle.window_should_close() {
        handle.update_camera(&mut camera, CameraMode::Orbital);

        if handle.is_mouse_button_pressed(MouseButton::Left) {
            current_model = (current_model + 1) % models.len();
        }

        if handle.is_key_pressed(KeyboardKey::Right) {
            current_model = (current_model + 1) % models.len();
        } else if handle.is_key_pressed(KeyboardKey::Left) {
            current_model = current_model.checked_sub(1).unwrap_or(models.len() - 1);
        }

        handle.begin_drawing(|mut d| {
            d.clear_background(Color::RAYWHITE);
            d.begin_mode_3d(
                |mut d3| {
                    d3.draw_model(models[current_model], position, 1.0, Color::WHITE)
                        .unwrap();
                    d3.draw_grid(10, 1.0);
                },
                camera,
            );
            d.draw_rectangle(
                30,
                400,
                310,
                30,
                den4ik_raylib::color::fade(Color::SKYBLUE, 0.5),
            );
            d.draw_rectangle_lines(
                30,
                400,
                310,
                30,
                den4ik_raylib::color::fade(Color::DARKBLUE, 0.5),
            );
            d.draw_text(
                "MOUSE LEFT BUTTON to CYCLE PROCEDURAL MODELS",
                40,
                410,
                10,
                Color::BLUE,
            );
            let label = match current_model {
                0 => ("PLANE", 680),
                1 => ("CUBE", 680),
                2 => ("SPHERE", 680),
                3 => ("HEMISPHERE", 640),
                4 => ("CYLINDER", 680),
                5 => ("TORUS", 680),
                6 => ("KNOT", 680),
                7 => ("POLY", 680),
                8 => ("Custom (triangle)", 580),
                _ => unreachable!(),
            };
            d.draw_text(label.0, label.1, 10, 20, Color::DARKBLUE);
        });
    }
}
