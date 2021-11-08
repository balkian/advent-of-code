fn main() {
    println!("Solution 1: {}", solve1(5468));
    println!("Solution 2: {}", solve2(5468));
}

fn solve1(serial: isize) -> String {
    let power = initialize(serial);
    let (max_coords, _) = solve(&power, 3);
    format!("{},{}", max_coords.0+1,max_coords.1+1)
}

/// This kind of brute-forces the problem.
/// We could speed it up by remembering the values per grid and adding only
/// the cells that differ (i.e., one row and one column).
fn solve2(serial: isize) -> String {
    let mut max_power = isize::MIN;
    let mut max_coords = (0, 0);
    let mut max_size = 0;
    let power = initialize(serial);

    for size in 1..300 {
        let (coords, value) = solve(&power, size);
        if value > max_power {
            max_power = value;
            max_coords = coords;
            max_size = size;
        }
    }
    format!("{},{},{}", max_coords.0+1,max_coords.1+1,max_size)
}

fn initialize(serial: isize) -> [[isize; 300]; 300] {
    let mut power = [[0isize; 300]; 300];
    for j in 0..300usize {
        for i in 0..300usize {
            let rack_id = (i as isize)+1+10;
            let mut power_level = (rack_id * ((j as isize)+1) + serial) * rack_id;
            power_level = ((power_level / 100) % 10) - 5;
            power[j][i] = power_level;
        }
    }
    power
}

fn solve(power: &[[isize;300];300], size: usize) -> ((usize, usize), isize) {
    let mut max_power = isize::MIN;
    let mut max_coords = (0, 0);
    for j in 0..(300-size) {
        for i in 0..(300-size) {
            let mut grid_power = 0;
            for dj in j..(j+size) {
                for di in i..(i+size) {
                    grid_power += power[dj][di];
                }
            }
            if grid_power > max_power {
                max_power = grid_power;
                max_coords = (i, j);
            }
        }
    }
    (max_coords, max_power)

}

#[test]
fn test_example(){
    assert_eq!(solve1(18), "33,45");
}
