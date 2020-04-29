extern crate crossbeam;
extern crate crossbeam_channel;

use std::{thread, time};
use crossbeam_channel::unbounded;

fn main() {
	let arr = &[1, 25, -4, 10];
	let max = find_max(arr);
	assert_eq!(max, Some(25));

	pass_data_between_threads()
}

fn find_max(arr: &[i32]) -> Option<i32> {
	const THRESHOLD: usize = 2;

	if arr.len() <= THRESHOLD {
		return arr.iter().cloned().max();
	}

	let mid = arr.len() / 2;
	let (left, right) = arr.split_at(mid);

	crossbeam::scope(|s| {
		let thread_l = s.spawn(|_| find_max(left));
		let thread_r = s.spawn(|_| find_max(right));

		let max_l = thread_l.join().unwrap()?;
		let max_r = thread_r.join().unwrap()?;

		Some(max_l.max(max_r))
	}).unwrap()
}

fn pass_data_between_threads() {
	let (snd, rcv)  = unbounded();
	let n_msgs = 5;
	crossbeam::scope(|s| {
		s.spawn(|_| {
			for i in 0..n_msgs {
				snd.send(i).unwrap();
				thread::sleep(time::Duration::from_millis(100));
			}
		});
	}).unwrap();
	for _ in 0..n_msgs {
		let msg = rcv.recv().unwrap();
		println!("Received {}", msg);
	}
}
