use std::{io::{Read, Write}, time::Duration, ops::DerefMut, cell::{RefCell, OnceCell}};

use crossbeam::atomic::AtomicCell;
use mini_ele_lib::DELTA;
// #[allow(unused_imports)]
pub use mini_ele_lib::{Action, RobotState};

static ROBOT_STATE: AtomicCell<RobotState> = AtomicCell::new(RobotState::new());


/// Gets the current state of the robot
#[inline(always)]
pub fn get_robot_state() -> RobotState {
    ROBOT_STATE.load()
}

thread_local! {
    static STDOUT_LOCK: OnceCell<RefCell<std::io::StdoutLock<'static>>> = OnceCell::new();
}


/// Sends an action to the runner
#[inline]
pub fn send_action(action: &Action) {
    STDOUT_LOCK.with(|stdout_lock| {
        let mut stdout = stdout_lock.get().unwrap().borrow_mut();
        bincode::serialize_into(stdout.deref_mut(), action).expect("Failed to send action");
        stdout.flush().expect("Failed to flush stdout");
    });
}

/// Waits for one frame
#[inline]
pub fn wait() {
    spin_sleep::sleep(Duration::from_secs_f32(DELTA));
}

/// Runs the given function with some background threads.
/// 
/// Your code should be placed inside the closure.
pub(super) fn wrapper(f: impl FnOnce()) {
    std::thread::spawn(|| {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut buf = [0u8; std::mem::size_of::<RobotState>()];
        loop {
            match stdin.read_exact(&mut buf) {
                Ok(()) => {
                    let state = bytemuck::from_bytes::<RobotState>(&buf);
                    ROBOT_STATE.store(*state);
                }
                Err(e) => {
                    eprintln!("Error reading from runner: {}", e);
                    std::process::exit(1);
                }
            }
        }
    });

    STDOUT_LOCK.with(|stdout_lock| {
        stdout_lock.set(RefCell::new(std::io::stdout().lock())).unwrap();
    });

    f();
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    }
}