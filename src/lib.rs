#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable, plot_num};
use pc_keyboard::{DecodedKey, KeyCode};
use num::Integer;

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

pub struct SpaceInvadersGame {
    character: Character,
    cells: [[Cell; WIDTH]; HEIGHT],
    lasers: [Laser; 5],
    laser_count: usize,
    space_invaders: [Invaders; 70],
    invader_count: usize,
    status: Status,
    score: usize
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Status {
    Running,
    Over
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
#[repr(u8)]
pub enum Cell {
    Empty,
    Wall,
    Barricade
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Dir {
    N, S, E, W
}

impl Dir {
    fn reverse(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Position { col: i16, row: i16 }

impl Position {
    pub fn is_legal(&self) -> bool {
        0 <= self.col && self.col < WIDTH as i16 && 0 <= self.row && self.row < HEIGHT as i16
    }

    pub fn row_col(&self) -> (usize, usize) {
        (self.row as usize, self.col as usize)
    }

    pub fn neighbor(&self, d: Dir) -> Position {
        match d {
            Dir::N => Position {row: self.row - 1, col: self.col},
            Dir::S => Position {row: self.row + 1, col: self.col},
            Dir::E => Position {row: self.row,     col: self.col + 1},
            Dir::W => Position {row: self.row,     col: self.col - 1}
        }
    }
}

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct Character {
    col: usize,
    row: usize,
    dx: usize,
    active: bool

}

impl Character {
    pub fn new() -> Self {
        Character {
            col: BUFFER_WIDTH / 2,
            row: BUFFER_HEIGHT - 3,
            dx: BUFFER_WIDTH / 2,
            active: true
        }
    }
    fn update_character(&mut self) {
        self.clear_current();
        self.update_location();
        self.draw_current();
    }

    fn clear_current(&self) { plot(' ', self.col, self.row, ColorCode::new(Color::Black, Color::Black)); }

    fn update_location(&mut self) {
        if self.dx <= 2 { self.dx = 2 }
        if self.dx >= 78 { self.dx = 78 }
        else{ self.col = self.dx; }
    }

    fn draw_current(&self) { plot('A', self.col, self.row, ColorCode::new(Color::Cyan, Color::Black)); }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Laser {
    pos: Position,
    active: bool
}

impl Laser {
    fn new(pos: Position) -> Self {
        Laser{
            pos,
            active: true
        }
    }

    fn update_lasers(&mut self) {
        self.clear_laser();
        self.increment_laser();
        self.draw_laser();
    }

    fn clear_laser(&self) {
        if self.active { plot(' ', self.pos.col as usize, self.pos.row as usize, ColorCode::new(Color::Black, Color::Black)); } }

    fn draw_laser(&self) {
        if self.active { plot('|', self.pos.col as usize, self.pos.row as usize, ColorCode::new(Color::Red, Color::Black)); } }

    fn increment_laser(&mut self) {
        if self.pos.row > 1 {
            self.pos.row -= 1;
        }
        else{
            self.active = false;
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Invaders {
    pos: Position,
    dir: Dir,
    active: bool
}

impl Invaders {
    fn new(pos: Position) -> Self {
        Invaders{
            pos,
            dir: Dir::E,
            active: true
        }
    }

    fn update_invader(&mut self) {
        self.clear_invader();
        self.increment_invader();
        self.draw_invader();
    }

    fn clear_invader(&self) {
        if self.active { plot(' ', self.pos.col as usize, self.pos.row as usize, ColorCode::new(Color::Black, Color::Black)); }
    }

    fn draw_invader(&self) {
        if self.active { plot('M', self.pos.col as usize, self.pos.row as usize, ColorCode::new(Color::Green, Color::Black)); }
        }

    fn increment_invader(&mut self) {
        if self.pos.col <= 2 {
            self.pos.row += 1;
            self.dir = self.dir.reverse();
        }
        if self.pos.col >= 78 {
            self.pos.row += 1;
            self.dir = self.dir.reverse();
        }
        else {
            if self.pos.row.is_even(){
                self.pos.col += 1;
            }
            if self.pos.row.is_odd() {
                self.pos.col -= 1;
            }
        }
    }
}

const HEADER: &'static str =
    "##                                 SpaceInvaders                               #
#Score: 0                                                                      #";
const START: &'static str =
    "################################################################################
#                                                                              #
#     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #
#                                                                              #
#     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #
#                                                                              #
#     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#       ****                ****                ****                ****       #
#     ********            ********            ********            ********     #
#    **********          **********          **********          **********    #
#    **********          **********          **********          **********    #
#                                                                              #
#                                                                              #
#                                                                              #
################################################################################";

impl SpaceInvadersGame {
    pub fn new() -> Self {
        let mut game = SpaceInvadersGame {
            character: Character::new(),
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
            lasers: [Laser::new(Position{col: 0, row: 0}); 5],
            laser_count: 0,
            space_invaders: [Invaders::new(Position{col: 0, row: 0}); 70],
            invader_count: 0,
            status: Status::Running,
            score: 0
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        self.put_header();
        for (row, row_chars) in START.split('\n').enumerate() {
            for (col, icon) in row_chars.chars().enumerate() {
                if icon == ' ' { plot(icon, col, row + 2, ColorCode::new(Color::Black, Color::Black)) }
                else {
                    self.translate_icon(row, col, icon);
                    plot(icon, col, row + 2, ColorCode::new(Color::Blue, Color::Black));
                }
            }
        }
        self.status = Status::Running;
        self.laser_count = 0;
        self.invader_count = 0;
        self.score = 0;
    }

    fn translate_icon(&mut self, row: usize, col: usize, icon: char) {
        match icon {
            '#' => self.cells[row][col] = Cell::Wall,
            '*' => self.cells[row][col] = Cell::Barricade,
            ' ' => self.cells[row][col] = Cell::Empty,
            'A' => self.character = Character::new(),
            'M' => {
                    self.invader_count += 1;
                    self.space_invaders[self.invader_count] = Invaders::new( Position{col: col as i16 ,row: row as i16 } );
            }
            _ =>  panic!("Unrecognized character: '{}'", icon)
        }
    }

    fn put_header(&mut self) {
        for (row, row_chars) in HEADER.split('\n').enumerate() {
            for (col, icon) in row_chars.chars().enumerate() {
                plot(icon, col, row, ColorCode::new(Color::White, Color::Black));
            }
        }
    }

    fn update_score(&mut self) {
        self.score += 10;
        plot_num(self.score as isize, 8, 1, ColorCode::new(Color::White, Color::Black));
    }

    pub fn tick(&mut self) {
        self.character.update_character();
        self.increment_laser();
        //rn doesnt work with increment_invaders()
        //self.increment_invaders();
        self.check_collision();
        self.update_score();
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.character.dx -= 1;
            }
            KeyCode::ArrowRight => {
                self.character.dx += 1;
            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        match self.status {
            Status::Over => { if key == ' ' { self.reset()} },
            _ => { if key == ' ' { self.shoot() } }
        }
    }

    fn shoot(&mut self) {
        if self.laser_count < self.lasers.len() {
            self.lasers[self.laser_count] = Laser::new(Position{col: self.character.col as i16, row: self.character.row as i16 - 1});
            self.laser_count += 1;
        }
    }

    fn increment_laser(&mut self) {
        for laser in self.lasers.iter_mut() {
            laser.update_lasers();
            if laser.active == false && self.laser_count > 1 {
                self.laser_count -= 1;
            }
        }
    }

    fn increment_invaders(&mut self) {
        for invader in self.space_invaders.iter_mut() {
            invader.update_invader();
        }
    }

    fn check_collision(&mut self) {
        for laser in self.lasers.iter_mut() {
            //check for collision with barricades or walls

            for invader in self.space_invaders.iter_mut(){
                if (laser.pos.row == invader.pos.row && laser.pos.col == invader.pos.col) && (laser.active && invader.active) {
                    plot(' ', laser.pos.col as usize, laser.pos.row as usize, ColorCode::new(Color::Black, Color::Black));
                    laser.active = false;
                    invader.active = false;
                    self.score += 100;
                }
            }
        }
    }
}
