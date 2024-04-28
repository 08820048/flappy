# Rust开发一个小游戏
![image](https://github.com/08820048/flappy/assets/71597859/30f82922-1505-4383-b293-cf3ebe3ce634)

![image](https://github.com/08820048/flappy/assets/71597859/e216ee91-0e9b-47f9-8948-995d9ba845e8)



## 1. Game loop

使用`game loop`可以使得游戏运行更加流畅和顺滑，它可以：

- 初始化窗口、图形和其他资源；
- 每当屏幕刷新他都会运行(通常是每秒30,60 )；
- 每次通过循环，他都会调用游戏的`tick()`函数。

大致的原理流程如下：

![image-20240428105116174](https://images.waer.ltd/notes/image-20240428105116174.png)

---

## 2. 游戏引擎/库

这里选择使用一款名为`bracket-Lib`的游戏编程库，这是基于`rust`：

- 抽象了游戏开发中很多复杂的东西，但是保留了相关的概念，可以作为简单的教学工具。
- 包括了随机数生成、几何、寻路、颜色处理、常用算法等。

---

### 2.1 Bracket-terminal

这个终端主要负责`Bracket-Lib`中的显示部分。

- 提供了模拟控制台；
- 可以与多种渲染平台配合
  -  从文本控制台到`Web Assembly`
  - 例如:`OpenGL`,`Vulkan`,`Metal`;
- 支持`sprites`和原生的`OpenGL`开发。

---

### 2.2  Codepage437

- 这是IBM扩展的ACSLL字符集。来自`Dos PC`上得到字符，用于终端输出，除了字母和数字，还提供一些符号。
- `Bracket-lib`会把字符翻译为图形`sprites`并提供一个有限的字符集，字符所展示的是相应的图片；

---

## 3. 开始编码

### 3.1 游戏窗口初始化

使用`cargo new`创建游戏项目并导入`Gracket-lib`依赖。下面是第一部分代码实现，创建了游戏终窗口并打印一条简单的输出：

```rust
use bracket_lib::prelude::*;

// 保留帧状态

struct State {

}

// 状态怎么和哟游戏帧关联上呢？，这就用到了一个名为GaemState的trait
impl GameState for State {
    // 实现tick函数
    fn tick(&mut self, ctx: &mut BTerm) {
        // 清屏
        ctx.cls();
        // 在屏幕上打印输出,坐标系x,y从屏幕左上角开始计算(0,0)
        ctx.print(1, 1, "Hello,Bracket Terminall!");


    }
}
fn main() -> BError {
    // 创建一个80x50的简单窗口，标题为游戏名称，?表示这个build可能会出错，出错就捕获返回，否则成功
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    
    main_loop (context,State{})
}
```

运行结果:

![image-20240428112645545](https://images.waer.ltd/notes/image-20240428112645545.png)

---

### 3.2 游戏模式

一般情况下，游戏都是有一些明确的游戏模式，每种模式会明确游戏在当前的`tick()`中应该作的任务。

这个游戏也不例外，主要涉及三种模式:

- 菜单
- 游戏中
- 结束

> 下面先将大致的框架构建好。

```rust
use bracket_lib::prelude::*;

// 保留帧状态
struct State {
    mode:GameMode,

}
// 为游戏状态实现一个叫new的关联函数
impl State {
    fn new() ->Self {
        State {
            mode:GameMode::Menu, // 设置游戏初始状态为菜单模式
        }
    }

    // 实现play方法
    fn play(&mut self,ctx:&mut BTerm) {
        //TODO
		self.mode = GameMode::End;
    }
    
  	// restart
    fn resatrt(&mut self) {
        self.mode = GameMode::Playing;
    }
    
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO

    }
    
    // 实现end方法
    fn dead(&mut self, ctx: &mut BTerm) {
        
    }
    
    // 实现menu方法
    
}
// 游戏模式枚举并存储到游戏状态中
enum GameMode{
    Menu,
    Playing,
    End,
}


// 状态怎么和哟游戏帧关联上呢？，这就用到了一个名为GaemState的trait
impl GameState for State {
    // 实现tick函数
    fn tick(&mut self, ctx: &mut BTerm) {
        // 根据游戏状态选择方向
        match self.mode {
            GameMode::Menu =>self.main_menu(ctx),
            GameMode::Playing => self.dead(ctx),
            GameMode::End => self.play(ctx),
        }
        
    }
}
fn main() -> BError {
    // 创建一个80x50的简单窗口，标题为游戏名称，?表示这个build可能会出错，出错就捕获返回，否则成功
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    
    main_loop (context,State::new())
}
```

---

#### 3.2.1 游戏菜单实现

游戏菜单的实现逻辑比较简单，主要是提供一个游戏操作的入口以供玩家进行选择操作：

- 清理屏幕
- 打印欢迎语
- 开始游戏(P)
- 离开游戏(Q)

```rust
fn main_menu(&self, ctx: &mut BTerm) {
    // TODO
    ctx.cls();
    // print_centered会在屏幕水平中间位置进行打印
    ctx.print_centered( 5,"欢迎来到Flappy Dragon!");
    ctx.print_centered( 8, "(P) 开始游戏");
    ctx.print_centered(9, " (Q) 离开游戏");

    if let Some(key) =ctx.key {
        match key {
            VirtualKeyCode::P => self.resatrt(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
}
```

---

#### 3.2.2 游戏结束的实现

这块代码和游戏菜单差不多，把提示词换一下

```rust
// 实现end方法
fn dead(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    // print_centered会在屏幕水平中间位置进行打印
    ctx.print_centered( 5,"小菜鸡,你已经嘎了!");
    ctx.print_centered( 8, "(P) 不服,再战");
    ctx.print_centered(9, " (Q) 离开游戏" );

    if let Some(key) =ctx.key {
        match key {
            VirtualKeyCode::P => self.resatrt(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
}
```

---

#### 3.3.3 第一阶段效果

> 下面是该阶段全部代码，实现了游戏基本窗口以及三个基本模式的逻辑。

```rust
use bracket_lib::prelude::*;

// 保留帧状态
struct State {
    mode:GameMode,

}
// 为游戏状态实现一个叫new的关联函数
impl State {
    fn new() ->Self {
        State {
            mode:GameMode::Menu, // 设置游戏初始状态为菜单模式
        }
    }

    // 实现play方法
    fn play(&mut self,ctx:&mut BTerm) {
        //TODO
        self.mode = GameMode::End;
    }

    // menu方法
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls();
        // print_centered会在屏幕水平中间位置进行打印
        ctx.print_centered( 5,"Welcome to Flappy Dragon!");
        ctx.print_centered( 8, "(P) Start play");
        ctx.print_centered(9, " (Q) Quit game");
        
        if let Some(key) =ctx.key {
            match key {
                VirtualKeyCode::P => self.resatrt(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    // restart
    fn resatrt(&mut self) {
        self.mode = GameMode::Playing;
    }
    
    // 实现end方法
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // print_centered会在屏幕水平中间位置进行打印
        ctx.print_centered( 5,"You are dead!");
        ctx.print_centered( 8, "(P) replay");
        ctx.print_centered(9,  "(Q) quit game");
        
        if let Some(key) =ctx.key {
            match key {
                VirtualKeyCode::P => self.resatrt(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    
    
}
// 游戏模式枚举并存储到游戏状态中
enum GameMode{
    Menu,
    Playing,
    End,
}


// 状态怎么和哟游戏帧关联上呢？，这就用到了一个名为GaemState的trait
impl GameState for State {
    // 实现tick函数
    fn tick(&mut self, ctx: &mut BTerm) {
        // 根据游戏状态选择方向
        match self.mode {
            GameMode::Menu =>self.main_menu(ctx),
            GameMode::Playing => self.dead(ctx),
            GameMode::End => self.play(ctx),
        }
        
    }
}
fn main() -> BError {
    // 创建一个80x50的简单窗口，标题为游戏名称，?表示这个build可能会出错，出错就捕获返回，否则成功
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    
    main_loop (context,State::new())
}

```

- 运行效果:

![image-20240428121025278](https://images.waer.ltd/notes/image-20240428121025278.png)

---

### 3.3 添加play

这部分主要在游戏窗口添加一个玩家角色，这里以字符`@`作为龙，实现玩家通过空格键控制该角色的上下移动：

- 一定时间不按空格，角色会下落，当下落碰到屏幕时游戏失败并结束游戏；
- 按下空格时，龙会网上移动。

```rust
use bracket_lib::prelude::*;

// 保留帧状态
struct State {
    player:Player,
    frame_time:f32,// 结果多少帧后累计的时间
    mode:GameMode,

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
        
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    // menu方法
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // print_centered会在屏幕水平中间位置进行打印
        ctx.print_centered( 5,"Welcome to Flappy Dragon!");
        ctx.print_centered( 8, "(P) Play Game");
        ctx.print_centered(9,  "(Q) Quit Game");
        
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
        self.mode = GameMode::Menu;
    }
    
    // 实现end方法
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // print_centered会在屏幕水平中间位置进行打印
        ctx.print_centered( 5,"You are dead!");
        ctx.print_centered( 8, "(P) replay");
        ctx.print_centered(9,  "(Q) quit game");
        
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
fn main() -> BError {
    // 创建一个80x50的简单窗口，标题为游戏名称，?表示这个build可能会出错，出错就捕获返回，否则成功
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    
    main_loop (context,State::new())
}
```

---

### 3.4 添加障碍物

```rust
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
        // print_centered会在屏幕水平中间位置进行打印
        ctx.print_centered( 5,"Welcome to Flappy Dragon!");
        ctx.print_color_right(60, 7, WEB_GREEN, BLACK,"by:Gemini48");
        ctx.print_centered( 8, "(P) Play Game");
        ctx.print_centered(9,  "(Q) Quit Game");
        
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


```

> 结束。

### 参考&引用

> - Rust依赖库:[crates.io](https://crates.io/crates/bracket-lib)
> - [bracket-lib](https://github.com/amethyst/bracket-lib)
> - 

