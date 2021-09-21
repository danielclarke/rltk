use rltk::{GameState, Rltk, Tile, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

mod map;
use map::*;

mod rect;
use rect::*;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Debug)]
struct Player2 {}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn try_move_player<T: Component>(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<T>();

    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[dest_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(79, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player::<Player>(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player::<Player>(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player::<Player>(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player::<Player>(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

fn player_2_input(gs: &mut State, ctx: &Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player::<Player2>(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player::<Player2>(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player::<Player2>(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player::<Player2>(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        player_2_input(self, ctx);
        // self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    // gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    // gs.ecs.register::<Player2>();
    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    // gs.ecs
    //     .create_entity()
    //     .with(Position { x: 20, y: 25 })
    //     .with(Renderable {
    //         glyph: rltk::to_cp437('@'),
    //         fg: RGB::named(rltk::BLUE),
    //         bg: RGB::named(rltk::BLACK),
    //     })
    //     .with(Player2 {})
    //     .build();

    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 })
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('☺'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(LeftMover {})
    //         .build();
    // }

    rltk::main_loop(context, gs)
}
