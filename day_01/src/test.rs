#[cfg(test)]
mod test {
    use crate::{parse, part_1, part_2, Runner};
    use aoc_helper::runner::ProcessAndAssert;
    
    #[test]
    fn part_1_works_for_test() {
        Runner::from_test_file(parse).process_and_assert(part_1, 24000)
    }

    #[test]
    fn part_1_works_for_input() {
        Runner::from_input_file(parse).process_and_assert(part_1, 69883)
    }

    #[test]
    fn part_2_works_for_test() {
        Runner::from_test_file(parse).process_and_assert(part_2, 45000)
    }

    #[test]
    fn part_2_works_for_input() {
        Runner::from_input_file(parse).process_and_assert(part_2, 207576)
    }
}
