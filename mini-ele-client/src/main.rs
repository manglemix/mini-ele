mod wrapper;

use wrapper::*;


fn main() {
    wrapper(move || {
        // Write your code here
        // You can delete the following code
        send_action(&Action::SetBucketAngle(1.0));
        loop {
            let state = get_robot_state();
            println!("{}", state);
            wait();
        }
    });
}
