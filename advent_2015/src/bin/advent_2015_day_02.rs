use advent_2015::file_input;

fn part_1(dimensions: &Vec<Vec<u32>>) -> u32 {
    let mut sum_area = 0;
    for entry in dimensions {
        let largest_dimension = entry.iter().max().unwrap_or(&0);
        let (l, w, h) = (entry[0], entry[1], entry[2]);

        let wrap_area = 2 * l * w + 2 * l * h + 2 * w * h;
        let extra_area = (l * w * h) / largest_dimension;
        sum_area += wrap_area + extra_area;
    }
    sum_area
}

fn part_2(dimensions: &Vec<Vec<u32>>) -> u32 {
    let mut sum_ribbon = 0;
    for entry in dimensions {
        let largest_dimension = entry.iter().max().unwrap_or(&0);
        let (l, w, h) = (entry[0], entry[1], entry[2]);

        let smallest_face_perimeter = 2 * (l + w + h) - 2 * (largest_dimension);
        let volume = l * w * h;
        sum_ribbon += smallest_face_perimeter + volume;
    }
    sum_ribbon
}

fn split_to_int(line: &str) -> Vec<u32> {
    line.split('x')
        .map(|num| num.parse().unwrap_or(0))
        .collect::<Vec<u32>>()
}

fn main() {
    let lines = file_input::read_input_file();
    let dimensions: Vec<Vec<u32>> = lines.iter().map(|line| split_to_int(line)).collect();
    println!("{}", part_1(&dimensions));
    println!("{}", part_2(&dimensions));
}
