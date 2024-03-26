use std::{collections::HashMap, ops::AddAssign, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut total_count = HashMap::<char, usize>::new();

    if input.is_empty() {
        return total_count;
    }

    let receiver = {
        let (sender, receiver) = mpsc::channel();

        thread::scope(|s| {
            for chunk in input.chunks((input.len() - 1) / worker_count + 1) {
                let thread_sender = sender.clone();

                s.spawn(move || {
                    let mut counter = HashMap::<char, usize>::new();

                    for row in chunk {
                        for c in row.chars().filter(|c| c.is_alphabetic()) {
                            if let Some(c) = c.to_lowercase().next() {
                                counter.entry(c).or_insert(0).add_assign(1);
                            }
                        }
                    }

                    thread_sender.send(counter).unwrap();
                });
            }
        });

        receiver
    };

    for counter in receiver {
        for (c, count) in counter {
            total_count.entry(c).or_insert(0).add_assign(count);
        }
    }

    total_count
}
