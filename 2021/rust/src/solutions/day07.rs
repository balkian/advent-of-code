pub fn parse(input: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    nums.sort_unstable();
    nums
}

pub fn part1(nums: &[isize]) -> usize {
    let median = nums[nums.len() / 2];
    nums.iter().map(|n| (n - median).abs()).sum::<isize>() as usize
}

pub fn part2(nums: &[isize]) -> usize {
    // The minimum for the |x-m|^2 function is in the mean
    let mean = (nums.iter().sum::<isize>() as f32) / (nums.len() as f32);
    // but coordinates are restricted to integers,
    // so we have to check both the ceil and the floor:
    let mut opts: Vec<isize> = ((mean.floor() as isize)..=(mean.ceil() as isize))
        .map(|center| {
            nums.iter()
                .map(|n| {
                    let dist = (n - center).abs();
                    if dist == 0 {
                        0
                    } else {
                        ((dist as f32) * (1f32 + (dist as f32)) / 2f32) as isize
                    }
                })
                .sum::<isize>()
        })
        .collect();
    opts.sort_unstable();
    opts[0] as usize
}
