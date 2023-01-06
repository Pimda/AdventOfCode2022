#[cfg(test)]
use super::*;

#[test]
fn part_1_works_for_test() {
    Runner::from_test_file(parse).process_and_assert(part_1, 0)
}

#[test]
fn part_1_works_for_input() {
    Runner::from_input_file(parse).process_and_assert(part_1, 0)
}

#[test]
fn part_2_works_for_test() {
    Runner::from_test_file(parse).process_and_assert(part_2, 0)
}

#[test]
fn part_2_works_for_input() {
    Runner::from_input_file(parse).process_and_assert(part_2, 0)
}
