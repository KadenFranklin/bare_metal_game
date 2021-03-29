#![no_std]
#![no_main]

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use bare_metal_game::SpaceInvadersGame;
use crossbeam::atomic::AtomicCell;
use pluggable_interrupt_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .startup(startup)
        .start()
}

lazy_static! {
    static ref GAME: Mutex<SpaceInvadersGame> = Mutex::new(SpaceInvadersGame::new());
}

fn tick() {
    let mut game = GAME.lock();
    game.tick();
}

fn key(key: DecodedKey) {
    let mut game = GAME.lock();
    game.key(key);
}

fn startup() {
    clear_screen();
}
