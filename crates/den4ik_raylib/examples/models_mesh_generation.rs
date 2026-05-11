use den4ik_raylib::{
    Container, RaylibHandle,
    allocator::Allocator,
    color::{Color, fade},
    core::{
        CAMERA_ORBITAL, Camera, KEY_LEFT, KEY_RIGHT, MOUSE_BUTTON_LEFT, Vector3, begin_drawing,
        begin_mode_3d, clear_background, draw_grid, draw_rectangle, draw_rectangle_lines,
        draw_text, end_drawing, end_mode_3d, is_key_pressed, is_mouse_button_pressed,
        update_camera,
    },
    image::Image,
    material::MATERIAL_MAP_DIFFUSE,
    mesh::{Mesh, MeshConfigBuilder},
    model::Model,
    texture::Texture2D,
};

const NUM_MODELS: usize = 9;

fn gen_mesh_custom(handle: &mut RaylibHandle, allocator: &mut Allocator) -> usize {
    let config = MeshConfigBuilder::new(3, 1)
        .with_texcoords2()
        .with_normals()
        .build();
    let mesh_id = Mesh::new(handle, config, allocator).unwrap();
    let mesh = handle.meshes.get_mut(mesh_id).unwrap();

    let vertices = unsafe { std::slice::from_raw_parts_mut(mesh.inner.vertices, 9) };
    let texcoords = unsafe { std::slice::from_raw_parts_mut(mesh.inner.texcoords, 6) };
    let normals = unsafe { std::slice::from_raw_parts_mut(mesh.inner.normals, 9) };

    // Vertex at (0, 0, 0)
    vertices[0] = 0.0;
    vertices[1] = 0.0;
    vertices[2] = 0.0;
    normals[0] = 0.0;
    normals[1] = 1.0;
    normals[2] = 0.0;
    texcoords[0] = 0.0;
    texcoords[1] = 0.0;

    // Vertex at (1, 0, 2)
    vertices[3] = 1.0;
    vertices[4] = 0.0;
    vertices[5] = 2.0;
    normals[3] = 0.0;
    normals[4] = 1.0;
    normals[5] = 0.0;
    texcoords[2] = 0.5;
    texcoords[3] = 1.0;

    // Vertex at (2, 0, 0)
    vertices[6] = 2.0;
    vertices[7] = 0.0;
    vertices[8] = 0.0;
    normals[6] = 0.0;
    normals[7] = 1.0;
    normals[8] = 0.0;
    texcoords[4] = 1.0;
    texcoords[5] = 0.0;

    mesh.upload(handle, false);
    mesh_id
}

fn main() {
    let mut handle = RaylibHandle::new(800, 450, "raylib [models] example - mesh generation");
    let img_id = Image::gen_checked(&mut handle, 2, 2, 1, 1, Color::RED, Color::GREEN);
    let texture_id = Texture2D::load_from_image(&mut handle, img_id).unwrap();
    <handle as Container<Image>>::remove(img_id);

    let mut models = [0usize; NUM_MODELS];

    let mesh_id = Mesh::gen_plane(&mut handle, 2.0, 2.0, 4, 3);
    models[0] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_cube(&mut handle, 2.0, 1.0, 2.0);
    models[1] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_sphere(&mut handle, 2.0, 32, 32);
    models[2] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_hemisphere(&mut handle, 2.0, 16, 16);
    models[3] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_cylinder(&mut handle, 1.0, 2.0, 16);
    models[4] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_torus(&mut handle, 0.25, 4.0, 16, 32);
    models[5] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_knot(&mut handle, 1.0, 2.0, 16, 128);
    models[6] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = Mesh::gen_poly(&mut handle, 5, 2.0);
    models[7] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    let mesh_id = gen_mesh_custom(&mut handle, &mut allocator);
    models[8] = Model::load_from_mesh(&mut handle, mesh_id).unwrap();

    for i in 0..NUM_MODELS {
        let model = handle.models.get_mut(models[i]).unwrap();
        model.get_materials_mut()[0].set_texture(&handle, MATERIAL_MAP_DIFFUSE as i32, texture_id);
    }

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
    let mut current_model: i32 = 0;

    handle.set_target_fps(60);

    while !handle.window_should_close() {
        update_camera(&mut camera, CAMERA_ORBITAL as i32);

        if is_mouse_button_pressed(MOUSE_BUTTON_LEFT as i32) {
            current_model = (current_model + 1) % (NUM_MODELS as i32);
        }

        if is_key_pressed(KEY_RIGHT as i32) {
            current_model += 1;
            if current_model >= NUM_MODELS as i32 {
                current_model = 0;
            }
        } else if is_key_pressed(KEY_LEFT as i32) {
            current_model -= 1;
            if current_model < 0 {
                current_model = (NUM_MODELS as i32) - 1;
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
    // CloseWindow is called automatically via Drop on RaylibHandle
}
