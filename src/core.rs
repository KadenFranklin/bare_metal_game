#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;

const WIDTH: usize = 80;
const GAME_HEIGHT: usize = 23;
const HEADER_SPACE: usize = 2;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct SpaceInvadersGame {
    cells: [[Cell; WIDTH]; GAME_HEIGHT],
    space_invaders: SpaceInvaders,
    character: Character,
    score: u32,
    counter: u32,
    status: Status
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

    fn left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Position {
    col: i16, row: i16
}

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
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
    dx: ModNumC<usize, BUFFER_WIDTH>,
    dy: ModNumC<usize, BUFFER_HEIGHT>,
    active: bool
}

impl Character {
    pub fn new() -> Self {
        LetterMover {
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(22),
            dx: ModNumC::new(BUFFER_WIDTH / 2),
            dy: ModNumC::new(22),
            active: true
        }
    }

    pub fn tick(&mut self) {
        self.clear_current();
        self.update_location();
        self.draw_current();
    }

    fn shot(&mut self) { self.active = false; }

    fn clear_current(&self) { plot( ' ', self.col.a(), self.row.a(), ColorCode::new(Color::Black, Color::Black)); }

    fn update_location(&mut self) {
        if self.dx <  ModNumC::new(2) {
            self.col = ModNumC::new(2);
            self.dx = ModNumC::new(2);
        }
        if self.dx > ModNumC::new(78) {
            self.col = ModNumC::new(78);
            self.dx = ModNumC::new(78);
        }
        else {
            self.col = self.dx;
            self.row = self.dy;
        }
    }

    fn draw_current(&self) { plot( 'A', self.col.a(), self.row.a(), ColorCode::new(Color::Cyan, Color::Black)); }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.dx = self.dx - 1 ;
            }
            KeyCode::ArrowRight => {
                self.dx = self.dx + 1 ;
            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            match key {
                ' ' => {
                    //shoot or something
                }
                _ => {}
            }
        }
    }
}

//on tick self.Laser.movee
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Laser {
    char: char,
    pos: Position,
    active: bool,
}

impl Laser {
    fn new(pos: Position)-> Self {
        Laser{
            char : '|',
            pos,
            active: true
        }
    }

    fn movee(&mut self) { self.pos.row = self.pos.row + 1; }

    fn done(&mut self) {self.active = false;}
}

//on tick self.
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct SpaceInvaders {
    array: [[Invaders; 17]; 7],
}

impl SpaceInvaders {
    fn new() -> Self {
        SpaceInvaders{
            array: [[Invaders::new() ]]
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Invaders {
    char: char,
    pos: Position,
    active: bool
}

impl Invaders {
    fn new(pos: Position) -> Self {
        Invaders{
            char: 'M',
            pos,
            active: true
        }
    }

}

const START: &'static str =
    "################################################################################
#                                                                             #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#      M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M   M      #
#                                                                             #
#                                                                             #
#                                                                             #
#                                                                             #
#                                                                             #
#                                                                             #
#    **********          **********          **********          **********   #
#    **********          **********          **********          **********   #
#    **********          **********          **********          **********   #
#    **********          **********          **********          **********   #
#                                                                             #
#                                      A                                      #
#                                                                             #
################################################################################";

impl SpaceInvadersGame {
    pub fn new() -> Self {
        let mut game = SpaceInvadersGame {
            space_invaders: spaceInvaders::new(),
            character: Character::new(),
            score: 0,
            status: Status::Running
        }
    }
    pub fn update() {

    }
}