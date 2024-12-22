pub fn part1(input: &str) -> eyre::Result<usize> {
    let mut disk = parse(input);

    let mut left = 0;
    let mut right = disk.len() - 1;

    loop {
        // if we have crossed over, we are done
        if right <= left {
            break;
        }

        // loop until we have free space on the left
        if disk[left].is_some() {
            left += 1;
            continue;
        }

        // we have hit free space
        // loop until we have a file on the right
        if disk[right].is_none() {
            right -= 1;
            continue;
        }

        // we can now swap
        disk.swap(left, right);
    }

    Ok(checksum(&disk))
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let mut _disk = parse(input);
    Ok(0)
}

#[derive(Clone, derive_more::Deref, derive_more::DerefMut)]
struct Disk(Vec<Option<usize>>);

fn checksum(disk: &Disk) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, contents)| contents.map(|c| (i, c)))
        .map(|(i, c)| i * c)
        .sum()
}

fn parse(input: &str) -> Disk {
    let digits = input.chars().filter_map(|c| c.to_digit(10));
    let blocks: u32 = digits.clone().sum::<u32>();

    let mut disk = Disk(vec![None; blocks as usize]);

    let mut free_space = false;
    let mut idx = 0;
    let mut file_id = 0;

    for block_size in input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
    {
        let contents = if !free_space { Some(file_id) } else { None };
        if !free_space {
            file_id += 1;
        }

        free_space = !free_space;

        disk[idx..(idx + block_size)].fill(contents);
        idx += block_size;
    }

    disk
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 1928);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
