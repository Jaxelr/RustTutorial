use std::time::Duration;
use std::thread;
use indicatif::ProgressBar;

fn main() {
    let pb = ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn do_hard_work() -> () {
    thread::sleep(Duration::from_millis(1000));
}