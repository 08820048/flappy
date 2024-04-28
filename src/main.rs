use std::fmt::format;

use bracket_lib::{prelude::*, random};

// 保留帧状态
struct State {
    player:Player,
    frame_time:f32,// 结果多少帧后累计的时间
    mode:GameMode,
    obstacle:Obstacle,
    score:i32,

}

const SCREEN_WIDTH:i32 = 80; // 屏幕宽度
const SCREEN_HEIGHT:i32 = 50; // 屏幕高度
const FRAME_DURATION:f32 = 75.0; //

struct Player {
    x:i32,
    y:i32,
    velocity:f32,// 纵向速度 > 0 玩家就会往下掉
}
// 游戏模式枚举并存储到游戏状态中
enum GameMode{
    Menu,
    Playing,
    End,
}
impl Player {
    fn new(x:i32,y:i32)-> Self {
        Player {
            x:0,
            y:0,
            velocity:0.0, // 下落更加丝滑
        }
    }

    // 使用'@’在屏幕上表示玩家
    fn render (&mut self,ctx:&mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }


    fn gravity_and_move (&mut self) {
        // 当下降速度小于2.0时让它的重力加速度每次增加0.2
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }
    // 按下空格实现玩家角色的向上移动
    fn flap (&mut self) {
        self.velocity = -2.0;
    }

}

// 为游戏状态实现一个叫new的关联函数
impl State {
    fn new() ->Self {
        State {
            player:Player::new(5,25),
            frame_time:0.0,
            mode:GameMode::Menu, // 设置游戏初始状态为菜单模式
            obstacle:Obstacle::new(SCREEN_WIDTH,0),
            score:0,
        }
    }

    // 实现play方法
    fn play(&mut self,ctx:&mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time >FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0,0,  "Press Space to Flap");
        ctx.print(0, 1, &format!("Score:{}",self.score));

        self.obstacle.render(ctx, self.player.x);
        
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH,self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    // menu方法
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        // 欢迎语句
        ctx.print_color_centered(5, RGB::named(WHITE), RGB::named(BLACK), "Welcome to Flappy Dragon!");

        // Play Game按钮
        ctx.print_color_centered(8, RGB::named(GREEN), RGB::named(BLACK), "(P) Play Game");

        // Quit Game按钮
        ctx.print_color_centered(9, RGB::named(RED), RGB::named(BLACK), "(Q) Quit Game");
        
        ctx.print(SCREEN_WIDTH - 20, SCREEN_HEIGHT - 3, "Developer: Code0408");
        ctx.print(SCREEN_WIDTH - 20, SCREEN_HEIGHT - 2, "Version: 1.0");
        ctx.print(SCREEN_WIDTH - 20, SCREEN_HEIGHT-1, "Language: Rust");
        if let Some(key) =ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = GameMode::Playing,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    // restart
    fn resatrt(&mut self) {
        self.player = Player::new(5,25);
        self.frame_time = 0.0;
        //self.mode = GameMode::Menu;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH,0);
        self.score = 0;
    }
    
    // 实现end方法
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // print_centered会在屏幕水平中间位置进行打印
        ctx.print_centered( 5,"You are dead!");
        ctx.print_centered(6,&format!("You earned {} points",self.score));
        ctx.print_centered( 8, "(P) Play Again");
        ctx.print_centered(9,  "(Q) Quit Game");

        ctx.print(SCREEN_WIDTH - 20, SCREEN_HEIGHT - 3, "Developer: Code0408");
        ctx.print(SCREEN_WIDTH - 20, SCREEN_HEIGHT - 2, "Version: 1.0");
        ctx.print(SCREEN_WIDTH - 20, SCREEN_HEIGHT-1, "Language: Rust");
        
        if let Some(key) =ctx.key {
            match key {
                VirtualKeyCode::P => self.resatrt(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    
    
}



// 状态怎么和哟游戏帧关联上呢？，这就用到了一个名为GaemState的trait
impl GameState for State {
    // 实现tick函数
    fn tick(&mut self, ctx: &mut BTerm) {
        // 根据游戏状态选择方向
        match self.mode {
            GameMode::Menu =>self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

struct Obstacle {
    x:i32,
    gap_y:i32, // 表示上下两个障碍物之间的空隙
    size:i32,
}

impl Obstacle {
    fn new(x:i32,score:i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y:random.range(10, 40), // 障碍纵向高度缝隙随机
            size:i32::max(2,20-score),
        }
    }

    fn render(&mut self,ctx:&mut BTerm,player_x:i32) {
        let screen_x = self.x -  player_x; // 屏幕空间
        let half_size:i32  = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x,y, RED,BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x,y,RED,BLACK,to_cp437('|'));
        }
    }

    // 玩家碰撞到障碍物的处理
    fn hit_obstacle(&self,player:&Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x; // 玩家x和障碍物x坐标
        let player_above_gap  =player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}


fn main() -> BError {
    // 创建一个80x50的简单窗口，标题为游戏名称，?表示这个build可能会出错，出错就捕获返回，否则成功
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    
    main_loop (context,State::new())
}

