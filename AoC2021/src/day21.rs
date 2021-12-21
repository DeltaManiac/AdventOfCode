#![feature(let_else)]

pub fn input_generator(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let player1 = lines.next().expect("at least two player");
    let player2 = lines.next().expect("at least two player");
    let player1 = player1
        .strip_prefix("Player 1 starting position: ")
        .expect("player 1 info")
        .parse()
        .expect("player 1 position is numeric");
    let player2 = player2
        .strip_prefix("Player 2 starting position: ")
        .expect("player 2 info")
        .parse()
        .expect("player 2 position is numeric");
    (player1, player2)
}

pub fn solve_part1((player1, player2): &(usize, usize)) -> usize {
    let mut dienum = 6;
    let mut dierolls = 0;
    let mut curr_player = (*player1, 0);
    let mut prev_player = (*player2, 0);
    while prev_player.1 < 1000 {
        let (curr_pos, curr_score) = &mut curr_player;
        *curr_pos += dienum;
        *curr_pos = if *curr_pos > 9 {
            *curr_pos - 10
        } else {
            *curr_pos
        };
        *curr_score += if *curr_pos == 0 { 10 } else { *curr_pos };
        std::mem::swap(&mut curr_player, &mut prev_player);
        dierolls += 3;
        dienum = if dienum == 0 { 9 } else { dienum - 1 };
    }
    curr_player.1 * dierolls
}
pub fn solve_part2((player1, player2): &(usize, usize)) -> usize {
    const POINTS: usize = 21;
    let mut p1steps = [(0, 0); POINTS];
    let mut p2steps = [(0, 0); POINTS];
    part2_playersimulate(*player1, POINTS, 0, 1, &mut p1steps);
    part2_playersimulate(*player2, POINTS, 0, 1, &mut p2steps);
    let p1wins: usize = p1steps[0..POINTS - 1]
        .iter()
        .skip(1)
        .zip(p2steps[0..POINTS - 1].iter())
        .map(|((winspath, _), (_, losepath))| winspath * losepath)
        .sum();
    let p2wins: usize = p2steps[0..POINTS]
        .iter()
        .zip(p1steps[0..POINTS].iter())
        .map(|((winspath, _), (_, losepath))| winspath * losepath)
        .sum();
    if p1wins > p2wins {
        p1wins
    } else {
        p2wins
    }
}

// probabiliy value of rolling 3 x 3 sided dice
const P2DIEOUTCOMES: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
fn part2_playersimulate(
    pos: usize,
    scoreleft: usize,
    step: usize,
    stepmultiplier: usize,
    stepscore: &mut [(usize, usize)],
) {
    match stepscore.split_first_mut() {
        Some(((stepwins, steplose), stepscore)) => {
            let step = step + 1;
            for (rollnum, multiplier) in P2DIEOUTCOMES {
                let pos = pos + rollnum;
                let pos = if pos > 9 { pos - 10 } else { pos };
                let score = if pos == 0 { 10 } else { pos };
                let unipaths = multiplier * stepmultiplier;
                if score >= scoreleft {
                    *stepwins += unipaths;
                } else {
                    *steplose += unipaths;
                    part2_playersimulate(pos, scoreleft - score, step, unipaths, stepscore);
                }
            }
        }
        _ => return,
    }
}
