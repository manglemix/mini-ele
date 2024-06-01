use std::{fs::File, io::{BufWriter, Write}, ops::Deref, sync::Arc};

use crossbeam::atomic::AtomicCell;
use mini_ele_lib::{Action, RobotState, DELTA};

const DRIVE_SPEED: f32 = 2.0;
const LIFT_SPEED: f32 = 0.5;
const TILT_SPEED: f32 = 0.5;

const LIFT_DELTA: f32 = LIFT_SPEED * DELTA;
const TILT_DELTA: f32 = TILT_SPEED * DELTA;


fn main() -> std::io::Result<()> {
    let child = std::process::Command::new("cargo")
        .args(&["run", "--bin", "mini-ele-client"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.unwrap();
    let mut stdout = std::io::BufReader::new(child.stdout.unwrap());

    let drive = Arc::new(AtomicCell::new((0.0, 0.0)));
    let bucket_height = Arc::new(AtomicCell::new(0.1));
    let bucket_angle = Arc::new(AtomicCell::new(0.0));

    let drive2 = drive.clone();
    let bucket_height2 = bucket_height.clone();
    let bucket_angle2 = bucket_angle.clone();

    let recording = File::create("recording.bin")?;
    let mut recording = BufWriter::new(recording);

    let thr = std::thread::spawn(move || {
        let mut state = RobotState::default();
        let spin_sleeper = spin_sleep::SpinSleeper::default();
        
        loop {
            let target_height = bucket_height.load();
            let height_diff = target_height - state.bucket_height;

            if height_diff.abs() < LIFT_DELTA {
                state.bucket_height = target_height;
            } else {
                state.bucket_height += height_diff.signum() * LIFT_DELTA;
            }

            let target_angle = bucket_angle.load();
            let angle_diff = target_angle - state.bucket_angle;

            if angle_diff.abs() < TILT_DELTA {
                state.bucket_angle = target_angle;
            } else {
                state.bucket_angle += angle_diff.signum() * TILT_DELTA;
            }

            if let Err(e) = stdin.write_all(bytemuck::bytes_of(&state)) {
                if e.kind() == std::io::ErrorKind::BrokenPipe {
                    break;
                }
                eprintln!("Unexpected IO Error: {}", e);
                break;
            }
            recording.write_all(bytemuck::bytes_of(&state)).expect("Failed to write to recording file");
            spin_sleeper.sleep_s(DELTA as f64);
        }

        recording.flush().expect("Failed to flush recording file");
    });

    loop {
        let action: Action = match bincode::deserialize_from(&mut stdout) {
            Ok(action) => action,
            Err(e) => match e.deref() {
                bincode::ErrorKind::Io(err) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        println!("Your program ended itself.");
                        break;
                    }
                    eprintln!("Unexpected IO Error: {}", err);
                    break;
                }
                _ => {
                    eprintln!("Deserialization error. Ensure that you are sending the right messages.\n{}", e);
                    break;
                }
            }
        };
        match action {
            Action::SetDrive(x, y) => {
                drive2.store((x, y));
            }
            Action::SetBucketHeight(h) => {
                bucket_height2.store(h);
            }
            Action::SetBucketAngle(a) => {
                bucket_angle2.store(a);
            }
        }
    }

    thr.join().expect("Failed to join thread");
    
    Ok(())
}
