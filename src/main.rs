extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;
use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}
#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    
    fn rnd_postion() -> Position {
        Position {
            x: rand::thread_rng().gen_range(0,10),
            y: rand::thread_rng().gen_range(0,10),
        }
    }
}

struct Snake {
    body: LinkedList<Position>,
    dir: Direction,
}

struct Food {
    pos: Position
}

impl Food {

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

        let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square : graphics::types::Rectangle = graphics::rectangle::square(
            (self.pos.x * 20) as f64,
            (self.pos.y * 20) as f64,
            20_f64
        );

        gl.draw(args.viewport(), |c,gl| {
            let transform = c.transform;
            graphics::rectangle(blue, square, transform, gl);
        })
    }

    fn update(&mut self, snake: &Snake) {

        let head = snake.body.front().expect("Snake has no body");
        if head.x == self.pos.x && head.y == self.pos.y {
            self.pos = Position::rnd_postion();
        }

    }
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,

}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|p| {
                graphics::rectangle::square(
                    (p.x * 20) as f64,
                    (p.y * 20) as f64,
                    20_f64)
            })
            .collect();

        
        gl.draw(args.viewport(), |c,gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
            
        })
    }

    fn update(&mut self, food: &Food) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        println!("Snake location: {:?}", new_head);
        match self.dir {
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x  += 1,
            Direction::Up => new_head.y  -= 1,
            Direction::Down => new_head.y += 1,
        }
        if new_head.x == food.pos.x && new_head.y == food.pos.y {
            self.eat();
        }
        self.body.push_front(new_head);
        self.body.pop_back().unwrap();


    }

    fn eat(&mut self) {
        let mut back = (*self.body.back().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => back.x += 1,
            Direction::Right => back.x -= 1,
            Direction::Up => back.y += 1,
            Direction::Down => back.y -= 1 
        }
        self.body.push_back(back);
    }
}

impl Game {
    // MUTABLE REFERENCE: BECAUSE DRAWRING TO THE SCREEN IS A MUTATION
    fn render(&mut self, arg: &RenderArgs) {

        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });

        self.snake.render(&mut self.gl, arg);

        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update(&self.food);
        self.food.update(&self.snake);
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        };
    }
}
fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [800,600]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![Position{x:0, y:0}, Position{x:0,y:1}]).into_iter()),
            dir: Direction::Right
        },
        food: Food{pos: Position::rnd_postion()},
    };
    
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        
        // CHECK IF RENDER EVENT OCCURS
        if let Some(_r) = e.render_args() {
            game.render(&_r);
        }

        // CHECK IF UPDATE EVENT OCCURS
        if let Some(_u) = e.update_args() {
            game.update();
        }

        // CHECK IF USER INPUT EVENT OCCURS
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
