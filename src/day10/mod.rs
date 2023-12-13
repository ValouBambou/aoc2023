use crate::array2d::Array2D;
use crate::vec2d::Vec2D;
use crate::DaySolution;

pub struct Day10;

impl DaySolution for Day10 {
    const DAY: u8 = 10;
    const INPUT: &'static str = include_str!("input.txt");
    const EXAMPLE_1: &'static str = include_str!("example1.txt");
    const EXAMPLE_2: &'static str = include_str!("example2.txt");
    const EXAMPLE_ANSWER_1: usize = 8;
    const EXAMPLE_ANSWER_2: usize = 10;

    fn solve_part1(input: &str) -> usize {
        let (_, _, cycle) = find_cycle(input);
        cycle.len() / 2
    }

    fn solve_part2(input: &str) -> usize {
        let (width, height, cycle) = find_cycle(input);
        let mut map = bigger_map(width, height, cycle);
        // double the map size
        let mut count_out = usize::MAX;
        while count_out > 0 {
            // dfs until no more outisde points are discovered
            count_out = count_out_points(&mut map);
        }
        // count fully contained 2x2 square
        (0..(width * 2))
            .step_by(2)
            .flat_map(move |x| (0..(height * 2)).step_by(2).map(move |y| (x, y)))
            .filter(|&(x, y)| {
                (!map.get(x, y))
                    && (!map.get(x + 1, y))
                    && (!map.get(x, y + 1))
                    && (!map.get(x + 1, y + 1))
            })
            .count()
    }
}

fn bigger_map(width: usize, height: usize, cycle: Vec<(usize, usize)>) -> Array2D<bool> {
    let mut map = Array2D::<bool>::new_default(2 * width, 2 * height);
    for (i, &cur) in cycle.iter().enumerate() {
        let cur_v = Vec2D::from(cur);
        let next = cycle[(i + 1) % cycle.len()];
        let next_v = Vec2D::from(next);
        let dif_next = next_v - cur_v;
        let mapped_cur = cur_v * 2;
        map.setv(mapped_cur, true);
        map.setv(mapped_cur + dif_next, true);

        let prev = cycle[(i + cycle.len() - 1) % cycle.len()];
        let prev_v = Vec2D::from(prev);
        let dif_prev = prev_v - cur_v;
        if dif_prev.dot(&dif_next) == 0 {
            map.setv(mapped_cur + dif_prev, true);
        }
    }
    map
}

fn find_corner_out(map: &Array2D<bool>) -> Option<(usize, usize)> {
    let w = map.width();
    let h = map.height();
    if let Some(y) = (0..h).find(|&y| !map.get(0, y)) {
        return Some((0, y));
    }

    if let Some(y) = (0..h).find(|&y| !map.get(w - 1, y)) {
        return Some((w - 1, y));
    }
    if let Some(x) = (0..w).find(|&x| !map.get(x, 0)) {
        return Some((x, 0));
    }
    if let Some(x) = (0..w).find(|&x| !map.get(x, h - 1)) {
        return Some((x, h - 1));
    }
    None
}

fn count_out_points(map: &mut Array2D<bool>) -> usize {
    let start = if let Some(start) = find_corner_out(map) {
        start
    } else {
        return 0;
    };
    let mut count = 0;
    let mut stack = vec![start];
    map.set(start.0, start.1, true);
    while let Some((cur_x, cur_y)) = stack.pop() {
        count += 1;
        let neighbors: Vec<(usize, usize)> = map.neighbors4_indices(cur_x, cur_y).collect();
        for (x, y) in neighbors {
            if !map.get(x, y) {
                map.set(x, y, true);
                stack.push((x, y));
            }
        }
    }
    count
}

fn find_cycle(input: &str) -> (usize, usize, Vec<(usize, usize)>) {
    let input = input.trim();
    let height = input.lines().count();
    let width = input.len() / height;
    let buf: Vec<char> = input.lines().flat_map(str::chars).collect();
    let pipes = Array2D::new(width, height, buf);

    let start = pipes.find_index('S').unwrap();

    let mut neighbors = pipes.neighbors4_indices(start.0, start.1);
    let cycle = neighbors
        .find_map(|mut cur| {
            let mut prev = start;
            let mut cycle: Vec<(usize, usize)> = Vec::new();
            while let Some(next) = next_pipe(prev, cur, &pipes) {
                cycle.push(cur);
                prev = cur;
                cur = next;
            }
            (cur == start).then(move || {
                cycle.push(start);
                cycle
            })
        })
        .unwrap();
    (width, height, cycle)
}

fn next_pipe(
    prev: (usize, usize),
    cur: (usize, usize),
    pipes: &Array2D<char>,
) -> Option<(usize, usize)> {
    let go_east = prev.0 < cur.0;
    let go_west = prev.0 > cur.0;
    let go_south = prev.1 < cur.1;
    let go_north = prev.1 > cur.1;
    let not_border_north = cur.1 > 0;
    let not_border_west = cur.0 > 0;
    let not_border_south = cur.1 < pipes.height() - 1;
    let not_border_east = cur.0 < pipes.width() - 1;

    match pipes.get(cur.0, cur.1) {
        '-' if go_east && not_border_east => Some((cur.0 + 1, cur.1)),
        '-' if go_west && not_border_west => Some((cur.0 - 1, cur.1)),
        '|' if go_south && not_border_south => Some((cur.0, cur.1 + 1)),
        '|' if go_north && not_border_north => Some((cur.0, cur.1 - 1)),
        'L' if go_south && not_border_east => Some((cur.0 + 1, cur.1)),
        'L' if go_west && not_border_north => Some((cur.0, cur.1 - 1)),
        'F' if go_north && not_border_east => Some((cur.0 + 1, cur.1)),
        'F' if go_west && not_border_south => Some((cur.0, cur.1 + 1)),
        'J' if go_south && not_border_west => Some((cur.0 - 1, cur.1)),
        'J' if go_east && not_border_north => Some((cur.0, cur.1 - 1)),
        '7' if go_east && not_border_south => Some((cur.0, cur.1 + 1)),
        '7' if go_north && not_border_west => Some((cur.0 - 1, cur.1)),
        _ => None,
    }
}
