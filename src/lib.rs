#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::Color::{Cyan, Black, Blue};

const HEIGHT: usize = 25;
const WIDTH: usize = 80;
const GAME_HEIGHT: usize = 23;
const HEADER_SPACE: usize = 2;

pub struct SpaceInvadersGame {
    cells: [[Cell; WIDTH]; HEIGHT],
    character: Character,
    lasers: [Laser; 5],
    laser_count: usize,
    space_invaders: [Invaders; 119],
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

    fn check_bounds(&mut self) -> usize {
        if self.pos.col <= 2 { self.pos.col as usize }
        if self.pos.col >= 78 { self.pos.col as usize }
        else{ self.pos.col as usize }
    }

    fn increment_invader(&mut self) {
        if self.check_bounds() <= 2 {
            self.pos.row += 1;
            self.dir = self.dir.reverse();
        }
        if self.check_bounds() >= 78 {
            self.pos.row += 1;
            self.dir = self.dir.reverse();
        }
        else {
            match self.dir {
                Dir::E => self.pos.col += 1,
                Dir::W => self.pos.col -= 1,
                _ => {}
            }
        }
    }
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

const START: &'static str =
    "################################################################################\n
    #                                                                              #\n
    #     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #\n
    #                                                                              #\n
    #     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #\n
    #                                                                              #\n
    #     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #\n
    #                                                                              #\n
    #     M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M  M      #\n
    #                                                                              #\n
    #                                                                              #\n
    #                                                                              #\n
    #                                                                              #\n
    #                                                                              #\n
    #                                                                              #\n
    #      ******              ******              ******              ******      #\n
    #    **********          **********          **********          **********    #\n
    #    **********          **********          **********          **********    #\n
    #    **********          **********          **********          **********    #\n
    #                                                                              #\n
    #                                      A                                       #\n
    #                                                                              #\n
    ################################################################################";

impl SpaceInvadersGame {
    pub fn new() -> Self {
        let mut game =SpaceInvadersGame {
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
            character: Character::new(),
            lasers: [Laser::new(Position{col: 0, row: 0}); 5],
            laser_count: 0,
            space_invaders: [Invaders::new(Position{col: 0, row: 0}); 119],
            invader_count: 0,
            status: Status::Running,
            score: 0
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        for (row, row_chars) in START.split('\n').enumerate() {
            for (col, icon) in row_chars.chars().enumerate() {
                self.translate_icon(row, col, icon);
                plot(icon, col, row, ColorCode::new(Color::Blue, Color::Black))
            }
        }
        self.status = Status::Running;
        self.laser_count = 0;
        self.invader_count = 0;
        self.score = 0;
    }

    fn translate_icon(&mut self, rowe: usize, column: usize, icon: char) {
        match icon {
            '#' => self.cells[rowe][column] = Cell::Wall,
            '*' => self.cells[rowe][column] = Cell::Barricade,
            'A' => self.character = Character::new(),
            'M' => {
                if self.invader_count < 93 {
                    self.invader_count += 1;
                    self.space_invaders[self.invader_count] = Invaders::new( Position{col: column as i16 ,row: rowe as i16 } );
                }
            }
            _ =>  panic!("Unrecognized character: '{}'", icon)
        }
    }

    pub fn tick(&mut self) {
        self.character.update_character();
        self.increment_laser();
        self.increment_invaders();
        self.check_collision();
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
            Status::Over => { if key == ' ' { self.reset() } },
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
            for invader in self.space_invaders.iter_mut(){
                if (laser.pos.row == invader.pos.row && laser.pos.col == invader.pos.col) && (laser.active && invader.active) {
                    laser.active = false;
                    invader.active = false;
                }
            }
        }
    }
}
