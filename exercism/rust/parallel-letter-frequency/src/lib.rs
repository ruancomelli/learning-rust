use std::{collections::HashMap, ops::AddAssign, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut total_count = HashMap::<char, usize>::new();

    let receiver = {
        let (sender, receiver) = mpsc::channel();

        let total_len = input.len();
        let chunk_size =
            total_len / worker_count + if total_len % worker_count == 0 { 0 } else { 1 };

        for index in 0..worker_count.min(total_len) {
            let start = index * chunk_size;
            let end = ((index + 1) * chunk_size).min(total_len);
            let chunk = &input[start..end];

            thread::scope(|s| {
                for row in chunk {
                    let thread_sender = sender.clone();

                    s.spawn(move || {
                        let mut counter = HashMap::<char, usize>::new();

                        for c in row.chars().filter(|c| c.is_alphabetic()) {
                            if let Some(c) = c.to_lowercase().next() {
                                counter.entry(c).or_insert(0).add_assign(1);
                            }
                        }

                        thread_sender.send(counter).unwrap();
                    });
                }
            });
        }
        receiver
    };

    for counter in receiver {
        for (c, count) in counter {
            total_count.entry(c).or_insert(0).add_assign(count);
        }
    }

    total_count
}
