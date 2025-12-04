use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
};

pub fn search(input: &str, prefix: &str) -> eyre::Result<u32> {
    use std::num::NonZero;
    const BATCH_SIZE: u32 = 500;

    let max_threads = std::thread::available_parallelism().unwrap_or(NonZero::new(4).unwrap());

    let (input, prefix) = (input.to_string(), prefix.to_string());

    let start = Arc::new(AtomicU32::new(0));
    let found = Arc::new(Mutex::new(None));

    let mut handles = Vec::with_capacity(max_threads.get());

    for _ in 0..max_threads.get() {
        let start = start.clone();
        let found = found.clone();
        let input = input.clone();
        let prefix = prefix.clone();

        handles.push(std::thread::spawn(move || loop {
            if found.lock().unwrap().is_some() {
                break;
            }

            let start = start.fetch_add(BATCH_SIZE, Ordering::Relaxed);

            let attempt = (start..start + BATCH_SIZE).find(|n| {
                let input = format!("{}{n}", input.trim());
                let s = md5::compute(input);
                format!("{s:x}").starts_with(&prefix)
            });

            match attempt {
                Some(value) => *found.lock().unwrap() = Some(value),
                None => {}
            }
        }))
    }

    for h in handles {
        h.join().expect("should join");
    }

    let found = found.lock().unwrap();
    found.ok_or(eyre::format_err!("wtf"))
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    search(input, "00000")
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    search(input, "000000")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "abcdef";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 609043);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 6742839);
        Ok(())
    }
}
