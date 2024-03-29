use aoc_runner_derive::{aoc, aoc_generator};

type PublicKey = u64;
type EncryptionKey = u64;
type LoopSize = u32;

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Vec<PublicKey> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn decrypt(public_key: PublicKey, another_public_key: PublicKey) -> (EncryptionKey, LoopSize) {
    const DIVISOR: u64 = 20_201_227;
    const MULTIPLIER: u64 = 7;

    let mut value = 1;
    let mut encryption_key = 1;
    let mut loop_size = 0;

    while value != public_key {
        value = value * MULTIPLIER % DIVISOR;
        encryption_key = encryption_key * another_public_key % DIVISOR;
        loop_size += 1;
    }

    (encryption_key, loop_size)
}

#[aoc(day25, part1)]
fn part1(public_keys: &[PublicKey]) -> EncryptionKey {
    let card_pk = public_keys[0];
    let door_pk = public_keys[1];

    decrypt(card_pk, door_pk).0
}
