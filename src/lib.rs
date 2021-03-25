#![feature(const_generics)]
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, plot_str, plot_num, clear_row, ColorCode, Color};
use bare_metal_game::core;
use crate::core::{SpaceInvadersGame, Status, Cell, Position, Invaders, Character, Laser};

const GAME_HEIGHT: usize = 23;
const HEADER_SPACE: usize = 2;

pub type MainGame = SpaceInvadersGame;

pub fn tick(game: &mut MainGame) {
    if game.countdown_complete() {
        game.update();
        draw(game);
    }
}

fn draw(game: &MainGame) {
    draw_header(game);
    draw_board(game);
}

fn draw_header(game: &MainGame) {
    match game.status() {
        Status::Running => draw_normal_header(game),
        Status::Over => draw_game_over_header(game)
    }
}

fn draw_normal_header(game: &MainGame) {
    let header_color = ColorCode::new(Color::White, Color::Black);
    let score_text = "Score:";
    clear_row(0, Color::Black);
    clear_row(1, Color::Black);
    plot_str(score_text, 0, 0, header_color);
    plot_num(game.score() as isize, score_text.len() + 1, 0, header_color);
}

fn draw_subheader(subheader: &str) {
    plot_str(subheader, 0, 1, ColorCode::new(Color::LightRed, Color::Black));
}

fn draw_game_over_header(game: &MainGame) {
    draw_normal_header(game);
    draw_subheader("Game over. Press Spacebar to restart.");
}

fn draw_board(game: &MainGame) {
    for p in game.cell_pos_iter() {
        let (row, col) = p.row_col();
        let (c, color) = get_icon_color(game, p, &game.cell(p));
        plot(c, col, row + HEADER_SPACE, color);
    }
}
