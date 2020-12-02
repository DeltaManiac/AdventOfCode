struct Policy{
min_count : u32,
max_count:u32,
character:char,
password:String,
}
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l|{
        items = l.split_ascii_whitespace().collect()})
}
