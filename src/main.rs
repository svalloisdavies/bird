
use bracket_lib::{geometry, prelude::*};
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const DRAGON_FRAMES : [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    loc: geometry::PointF,
    velocity: f32,
    rotation: Radians,
    scale: geometry::PointF,
    frame: usize,
}

impl Player {
    fn new() -> Self {
        let start_loc = geometry::PointF { x: 5.0, y: 5.0 };
        Player {
            loc: start_loc,
            velocity: 0.0,
            rotation: Radians(0.0),
            scale: geometry::PointF { x: 2.0, y: 2.0 },
            frame: 0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            geometry::PointF {
                x: 0.0,
                y: self.loc.y,
            },
            1,
            self.rotation,
            self.scale,
            WHITE, NAVY,
            DRAGON_FRAMES[self.frame],
        );
        ctx.set_active_console(0);        
    }

    fn update_pos(&mut self) {
        if self.velocity < 1.2 {
            self.velocity += 0.1;
        }
        self.loc.y += self.velocity;
        self.loc.x += 0.1;
        if self.loc.y < 0.0 {
            self.loc.y = 0.0;
        }
        self.frame = (self.frame + 1) % 6
    }
    fn flap(&mut self) {
        self.velocity = -1.2;
    }
}

// struct Obstacle {
//     x: f32,
//     gap_y: i32,
//     size: i32,
// }

// impl Obstacle {
//     fn new(x:f32, score: i32) -> Self {
//         let mut random = RandomNumberGenerator::new();
//         Obstacle { x, gap_y: random.range(10,40), size: i32::max(2,20-score) }

//     }

//     fn render(&mut self, ctx: &mut BTerm, player_loc: geometry::PointF) {
//         let screen_x = self.x - player_loc.x;
//         let half_size = self.size /2 ;
//         for obs_range in [0..self.gap_y - half_size, self.gap_y + half_size..SCREEN_HEIGHT ] {
//             for y in obs_range {
//                 ctx.set(screen_x, y, RED, BLACK,to_cp437('|'));
//             }
//         }
//     }

//     fn hit(&self, player: &Player) -> bool {
//         false
//     }
// }
struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    // obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(),
            frame_time: 0.0,
            // obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.update_pos();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        // render:
        // let mut draw_batch = DrawBatch::new();
        // // Show frame rate
        // draw_batch.target(1);
        // draw_batch.cls();
        // // draw the bird:
        // ctx.set_active_console(1);
        // ctx.cls();
        // draw_batch.set_fancy(
        //     geometry::PointF {
        //         x: 0.0,
        //         y: self.player.loc.y,
        //     },
        //     0,
        //     self.player.rotation,
        //     self.player.scale,
        //     ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
        //     DRAGON_FRAMES[self.player.frame],
        // );
        // ctx.set_active_console(0);
        // draw_batch.print_color(
        //     Point::new(0, 0),
        //     &format!("Frame Time: {} ms", ctx.frame_time_ms),
        //     ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
        // );
        // draw_batch.submit(0).expect("Batch error");
        // render_draw_buffer(ctx).expect("render error");

        self.player.render(ctx);
        let msg = format!("press space to flap [x:{}]", self.player.loc.x);
        ctx.print(0, 0, msg);
        ctx.print(0, 1, &format!("Score: {}", self.score));
        // self.obstacle.render(ctx, self.player.loc.x);
        // if self.player.loc.x > self.obstacle.loc.x {
        //     // player made it past the obstacle:
        //     self.score += 1;
        //     self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        //     self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        // }
        if self.player.loc.y > SCREEN_HEIGHT as f32 {
            self.mode = GameMode::End;
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to flappy dragon");
        ctx.print_centered(8, "(P)lay Game");
        ctx.print_centered(9, "(Q)uit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You died!");
        ctx.print_centered(6, &format!("Your score was  {}", self.score));
        ctx.print_centered(8, "(P)lay Again");
        ctx.print_centered(9, "(Q)uit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new();
        self.frame_time = 0.0;
        // self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::End => self.dead(ctx),
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let resource_path = "../resources/flappy32.png";
    let context = BTermBuilder::simple80x50()
        .with_font(resource_path, 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, resource_path)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, resource_path)
        .with_title("flappy dragon")
        .with_tile_dimensions(16, 16)
        .build()?;
    main_loop(context, State::new())
}
