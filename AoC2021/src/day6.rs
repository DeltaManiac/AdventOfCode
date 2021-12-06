type Input  = Vec<u32>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    input.split(',').into_iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut school = input.clone();
    let mut new_born_fish = 0;
    (0..80).for_each(|_| {
        for fish in school.iter_mut(){
           if *fish == 0{
            *fish = 6;
              new_born_fish +=1;
           }else{
               *fish -= 1;
           }
        }
        while new_born_fish >0{
            school.push(8);
            new_born_fish -=1;
        }
    });
    school.iter().count() 
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> usize {
     let mut fish_count = [0usize; 9];
    for d in input {
        fish_count[*d as usize] += 1;
    }
    for _ in 0..256 {
        fish_count.rotate_left(1);
        fish_count[6] += fish_count[8];
    }
    fish_count.iter().sum()
}
