use pathfinding::prelude::bfs;
fn main() {
    let raw_data = &include_str!("../data/input.txt");
    let raw_data = raw_data.replace('S', "a");
    let graph: Vec<_> = raw_data
        .bytes()
        .filter(|c| c != &b'\n')
        .map(|c| c.to_ascii_lowercase() - b'a')
        .collect();
    let width = raw_data.bytes().position(|c| c == b'\n').unwrap() as i32;
    let height = graph.len() as i32 / width;
    //let start = raw_data.bytes().position(|c| c == b'S').unwrap() as i32;
    //start = start - start / (width + 1);
    let mut end = raw_data.bytes().position(|c| c == b'E').unwrap() as i32;
    end = end - end / (width + 1);
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    //println!("END: {},{}", end % width, end / width);
    let result = graph
        .iter()
        .enumerate()
        .filter(|p| p.1 == &0)
        .map(|(start, _)| {
            bfs(
                &(start as i32 % width, start as i32 / width),
                |(curr_x, curr_y)| {
                    let cur = graph[(curr_x + curr_y * width) as usize];
                    DIRECTIONS
                        .iter()
                        .map(|p| (curr_x + p.0, curr_y + p.1))
                        .filter(|(x, y)| {
                            x >= &0
                                && y >= &0
                                && x < &width
                                && y < &height
                                && (cur >= graph[(x + y * width) as usize]
                                    || graph[(x + y * width) as usize] - cur == 1)
                        })
                        .collect::<Vec<_>>()
                },
                |(x, y)| x + y * width == end,
            )
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap().len() - 1)
        .min();
    println!("{:?}", result);
}
