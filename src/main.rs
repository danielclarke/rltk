use rltk::{GameState, Rltk, Tile, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

mod map;
use map::*;

mod player;
use player::*;

mod rect;
use rect::*;

mod component;
use component::*;

struct LeftWalker {}

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        // self.run_systems();

        draw_map(&self.ecs, ctx);

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
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
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
