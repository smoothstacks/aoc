pub fn part1(input: &str) -> eyre::Result<usize> {
    let lines: Vec<_> = input.lines().collect();
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    const DIRECTIONS: [[(isize, isize); 4]; 8] = [
        [(0, 0), (0, 1), (0, 2), (0, 3)],       // RIGHT
        [(0, 0), (0, -1), (0, -2), (0, -3)],    // LEFT
        [(0, 0), (1, 0), (2, 0), (3, 0)],       // UP
        [(0, 0), (-1, 0), (-2, 0), (-3, 0)],    // DOWN
        [(0, 0), (1, 1), (2, 2), (3, 3)],       // UP LEFT
        [(0, 0), (1, -1), (2, -2), (3, -3)],    // DOWN RIGHT
        [(0, 0), (-1, 1), (-2, 2), (-3, 3)],    // DOWN LEFT
        [(0, 0), (-1, -1), (-2, -2), (-3, -3)], // DOWN RIGHT
    ];

    Ok(search(&lines, DIRECTIONS, XMAS))
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let lines: Vec<_> = input.lines().collect();
    const CROSS: [char; 5] = ['A', 'M', 'M', 'S', 'S'];
    const DIRECTIONS: [[(isize, isize); 5]; 4] = [
        [(0, 0), (1, 1), (1, -1), (-1, -1), (-1, 1)],
        [(0, 0), (1, -1), (-1, -1), (-1, 1), (1, 1)],
        [(0, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)],
        [(0, 0), (-1, 1), (1, 1), (1, -1), (-1, -1)],
    ];
    Ok(search(&lines, DIRECTIONS, CROSS))
}

fn search<const C: usize, const N: usize>(
    input: &[&str],
    searches: [[(isize, isize); N]; C],
    find: [char; N],
) -> usize {
    let mut total = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            'search: for search in searches {
                for (i, (nx, ny)) in search.into_iter().enumerate() {
                    let xx = x as isize + nx;
                    let yy = y as isize + ny;

                    let Some(char) = input
                        .get(yy as usize)
                        .and_then(|line| line.chars().nth(xx as usize))
                    else {
                        continue 'search;
                    };

                    if char != find[i as usize] {
                        continue 'search;
                    }
                }

                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_works() -> eyre::Result<()> {
        const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(super::part1(INPUT)?, 18);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        const INPUT: &str = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!(super::part2(INPUT)?, 9);
        Ok(())
    }
}
