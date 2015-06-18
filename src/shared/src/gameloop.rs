use std::thread;

use clock_ticks::precise_time_ns;

pub enum LoopAction {
    Quit,
    Continue
}

pub fn game_loop<F>(tick: u64, mut f: F) where F: FnMut(u64) -> LoopAction {
    let mut acc = 0;
    let mut previous_clock = precise_time_ns();

    loop {
        let now = precise_time_ns();
        match f(now - previous_clock) {
            LoopAction::Quit => break,
            LoopAction::Continue => ()
        };

        acc += now - previous_clock;
        previous_clock = now;

        while acc >= tick {
            acc -= tick;
        }

        thread::sleep_ms(((tick - acc) / 1000000) as u32);
    }
}

