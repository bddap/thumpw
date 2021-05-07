extern crate alloc;

mod cam;

use crate::cam::Cam;
use alloc::collections::BTreeMap;
use futures::future::join_all;
use macroquad::prelude as mp;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::is_key_down;
use macroquad::prelude::mouse_position;
use macroquad::prelude::set_cursor_grab;
use macroquad::prelude::show_mouse;
use macroquad::prelude::Vec2;
use mp::{
    clear_background, draw_cube, draw_grid, draw_plane, draw_sphere, draw_text, is_key_pressed,
    load_texture, next_frame, set_camera, set_default_camera, vec2, vec3, KeyCode, Texture2D,
    BLACK, BLUE, RED, WHITE,
};
use tap::Pipe;

struct Chunk([u16; 16 * 16 * 16]);

struct Mouse {
    last_position: Vec2,
}

impl Mouse {
    fn create() -> Self {
        Self {
            last_position: mouse_position().into(),
        }
    }

    fn delta(&mut self) -> Vec2 {
        let new_p: Vec2 = mouse_position().into();
        let ret = new_p - self.last_position;
        self.last_position = new_p;
        ret
    }

    const LOOK_SPEED: f32 = 0.01;
    const MOVE_SPEED: f32 = 2.;
}

impl Default for Chunk {
    fn default() -> Self {
        Self([0; 16 * 16 * 16])
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    use alloc::collections::BTreeMap;

    let mut chunks: BTreeMap<[i32; 3], Chunk> = Default::default();
    let textures: Vec<Texture2D> = assets().await;

    let rust_logo = load_texture("asset/rust.png").await.unwrap();
    let ferris = load_texture("asset/ferris.png").await.unwrap();
    let mut cam = Cam {
        pos: vec3(-10., 0., 0.),
        uprot: 0.,
        rightrot: core::f32::consts::PI,
    };

    show_mouse(false);
    set_cursor_grab(true);
    let mut ms = Mouse::create();

    tmp_populate_chunks(&mut chunks);

    loop {
        controls(&mut cam, &mut ms);

        clear_background(RED);

        // Going 3d!

        set_camera(&cam);

        draw_grid(20, 1.);

        draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), ferris, WHITE);

        draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), rust_logo, WHITE);
        draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), ferris, WHITE);
        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
        for (i, t) in textures.iter().enumerate() {
            let h = i % 20;
            let v = i / 20;
            draw_cube(
                vec3(9., 1. + v as f32, -9. + h as f32),
                vec3(1., 1., 1.),
                *t,
                WHITE,
            );
        }

        for (loc, chunk) in chunks.iter() {
            drawchunk(*loc, chunk, &textures);
        }

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

async fn assets() -> Vec<Texture2D> {
    [
        "asset/block/brick_grey.png",
        "asset/block/brick_red.png",
        "asset/block/cactus_inside.png",
        "asset/block/cactus_side.png",
        "asset/block/cactus_top.png",
        "asset/block/cotton_blue.png",
        "asset/block/cotton_green.png",
        "asset/block/cotton_red.png",
        "asset/block/cotton_tan.png",
        "asset/block/dirt_grass.png",
        "asset/block/dirt.png",
        "asset/block/dirt_sand.png",
        "asset/block/dirt_snow.png",
        "asset/block/fence_stone.png",
        "asset/block/fence_wood.png",
        "asset/block/ferris.png",
        "asset/block/glass_frame.png",
        "asset/block/glass.png",
        "asset/block/grass1.png",
        "asset/block/grass2.png",
        "asset/block/grass3.png",
        "asset/block/grass4.png",
        "asset/block/grass_brown.png",
        "asset/block/grass_tan.png",
        "asset/block/grass_top.png",
        "asset/block/gravel_dirt.png",
        "asset/block/gravel_stone.png",
        "asset/block/greysand.png",
        "asset/block/greystone.png",
        "asset/block/greystone_ruby_alt.png",
        "asset/block/greystone_ruby.png",
        "asset/block/greystone_sand.png",
        "asset/block/ice.png",
        "asset/block/lava.png",
        "asset/block/leaves_orange.png",
        "asset/block/leaves_orange_transparent.png",
        "asset/block/leaves.png",
        "asset/block/leaves_transparent.png",
        "asset/block/mushroom_brown.png",
        "asset/block/mushroom_red.png",
        "asset/block/mushroom_tan.png",
        "asset/block/oven.png",
        "asset/block/redsand.png",
        "asset/block/redstone_emerald_alt.png",
        "asset/block/redstone_emerald.png",
        "asset/block/redstone.png",
        "asset/block/redstone_sand.png",
        "asset/block/rock_moss.png",
        "asset/block/rock.png",
        "asset/block/rust.png",
        "asset/block/sand.png",
        "asset/block/snow.png",
        "asset/block/stone_browniron_alt.png",
        "asset/block/stone_browniron.png",
        "asset/block/stone_coal_alt.png",
        "asset/block/stone_coal.png",
        "asset/block/stone_diamond_alt.png",
        "asset/block/stone_diamond.png",
        "asset/block/stone_dirt.png",
        "asset/block/stone_gold_alt.png",
        "asset/block/stone_gold.png",
        "asset/block/stone_grass.png",
        "asset/block/stone_iron_alt.png",
        "asset/block/stone_iron.png",
        "asset/block/stone.png",
        "asset/block/stone_sand.png",
        "asset/block/stone_silver_alt.png",
        "asset/block/stone_silver.png",
        "asset/block/stone_snow.png",
        "asset/block/table.png",
        "asset/block/track_corner_alt.png",
        "asset/block/track_corner.png",
        "asset/block/track_straight_alt.png",
        "asset/block/track_straight.png",
        "asset/block/trunk_bottom.png",
        "asset/block/trunk_mid.png",
        "asset/block/trunk_side.png",
        "asset/block/trunk_top.png",
        "asset/block/trunk_white_side.png",
        "asset/block/trunk_white_top.png",
        "asset/block/water.png",
        "asset/block/wheat_stage1.png",
        "asset/block/wheat_stage2.png",
        "asset/block/wheat_stage3.png",
        "asset/block/wheat_stage4.png",
        "asset/block/wood.png",
        "asset/block/wood_red.png",
    ]
    .iter()
    .map(|fln| async move { load_texture(fln).await.unwrap() })
    .pipe(join_all)
    .await
}

fn drawchunk(chunkloc: [i32; 3], chunk: &Chunk, textures: &[Texture2D]) {
    debug_assert!(!textures.is_empty());
    let [cx, cy, cz] = chunkloc;
    let [cx, cy, cz] = [cx * 16, cy * 16, cz * 16];
    for (i, b) in chunk.0.iter().enumerate() {
        let texture = textures[*b as usize % textures.len()];
        let [x, y, z] = [i / (16 * 16), i / 16 % 16, i % 16];
        mp::draw_cube(
            vec3(
                (cx + x as i32) as f32,
                (cy + y as i32) as f32,
                (cz + z as i32) as f32,
            ),
            vec3(1., 1., 1.),
            texture,
            WHITE,
        );
    }
}

fn tmp_populate_chunks(worl: &mut BTreeMap<[i32; 3], Chunk>) {
    fn set_bloc(worl: &mut BTreeMap<[i32; 3], Chunk>, pos: [i32; 3], bloc: u16) {
        let (global, local) = global_and_local(pos);
        worl.entry(global).or_insert_with(|| Default::default()).0[local] = bloc;
    }

    let rad = 10.;
    for i in 0..1000 {
        let fi = i as f32;
        let f2 = fi * 100.;
        let s = f2.cos();
        let x = fi.sin() * s * rad;
        let y = fi.cos() * s * rad;
        let z = f2.sin() * rad;
        set_bloc(
            worl,
            [x.round() as i32, y.round() as i32, z.round() as i32],
            i as u16,
        );
    }
}

fn global_and_local(pos: [i32; 3]) -> ([i32; 3], usize) {
    let in_chunk = |i: i32| ((i % 16 + 16) % 16) as usize;

    let [x, y, z] = pos;
    let global = [x / 16, y / 16, z / 16];
    let local = in_chunk(x) * 16 * 16 + in_chunk(y) * 16 + in_chunk(z);
    (global, local)
}

fn controls(cam: &mut Cam, ms: &mut Mouse) {
    let b = |b: bool| if b { 1. } else { 0. };

    if is_key_pressed(KeyCode::Q) {
        panic!("game exit");
    }
    if is_key_pressed(KeyCode::B) {
        set_cursor_grab(false);
    }
    if is_key_pressed(KeyCode::V) {
        set_cursor_grab(true);
    }

    let mut movec = vec3(
        b(is_key_down(KeyCode::D)) - b(is_key_down(KeyCode::A)),
        b(is_key_down(KeyCode::X)) - b(is_key_down(KeyCode::Z)),
        b(is_key_down(KeyCode::S)) - b(is_key_down(KeyCode::W)),
    );
    movec += vec3(0., 0., -1.);
    cam.translate_local(movec * get_frame_time() * Mouse::MOVE_SPEED);

    let [mx, my]: [f32; 2] = (ms.delta() * Mouse::LOOK_SPEED).as_ref().clone();
    cam.uprot += my;
    cam.uprot = cam
        .uprot
        .min(core::f32::consts::PI / 2.)
        .max(-core::f32::consts::PI / 2.);
    cam.rightrot += mx;
    cam.rightrot += 0.01;
}
