use bracket_lib::{prelude::*, random};
const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION: f32 = 75.0;

const SCREEN_POS:i32 = 3;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y:i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y:i32) -> Self {
        Player { x, y, velocity:0.0, }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(SCREEN_POS, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn update_pos(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x +=1 ;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = - 2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x:i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle { x, gap_y: random.range(10,40), size: i32::max(2,20-score) }

    }

    fn render(&mut self, ctx: &mut BTerm, player_x:i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size /2 ; 
        for obs_range in [0..self.gap_y - half_size, self.gap_y + half_size..SCREEN_HEIGHT ] {
            for y in obs_range {
                ctx.set(screen_x, y, RED, BLACK,to_cp437('|'));
            }
        }
    }

    fn hit(&self, player: &Player) -> bool {
        false
    }
}
struct State{
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score:i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5,25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
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
        self.player.render(ctx);
        let msg = format!("press space to flap [x:{}]", self.player.x);
        ctx.print(0,0, msg);
        ctx.print(0,1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            // player made it past the obstacle:
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to flappy dragon");
        ctx.print_centered(8, "(P)lay Game");
        ctx.print_centered(9, "(Q)uit Game");
        if let Some(key) = ctx.key{
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
        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

    }
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5,25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
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


fn main() -> BError{
    let context = BTermBuilder::simple80x50()
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_title("flappy dragon")
        .with_vsync(false)
        .build()?;
    main_loop(context , State::new())

}

