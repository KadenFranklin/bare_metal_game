#![no_std]
#![no_main]

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use bare_metal_game::LetterMover;
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
    static ref GAME: Mutex<SpaceInvadersGame> = Mutex::new(LetterMover::new());
}

fn tick() {
    let mut game = GAME.lock();
    game.tick();
}

fn key(key: DecodedKey) {
    let mut game = GAME.lock();
    game.key(key);
}

// need to think about how i am going to structure the invaders
// if an entire column gets destroyed, need to update it so that
// bounce off of the boundary from that - DATA STRUCTURE ???

// need to think about the cover aswell, needs to be
// destroyable by both char and invader lasers

// on tick- move space invaders,
// move char following keypress,
// launch laser following keypress,
// move laser,
// update score,
// if hit by laser game over

// keypress interupts - move left & right, rockets

// need interrupts for moving left and right, rockets, rocket collision
