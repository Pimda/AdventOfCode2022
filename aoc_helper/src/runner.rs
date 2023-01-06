use std::fmt::{Debug, Display};

pub struct Runner<Input> {
    input: Input,
}

impl<I> Runner<I> {
    pub fn from_test_file<F>(input_parser: F) -> Self
    where
        F: Fn(String) -> I,
    {
        Self::from_file("test.txt", input_parser)
    }

    pub fn from_input_file<F>(input_parser: F) -> Self
    where
        F: Fn(String) -> I,
    {
        Self::from_file("input.txt", input_parser)
    }

    pub fn from_file<F>(path: &str, input_parser: F) -> Self
    where
        F: Fn(String) -> I,
    {
        let input: String = std::fs::read_to_string(path).expect("File could not be read");
        Runner {
            input: input_parser(input),
        }
    }
}

pub trait ProcessAndWrite<'a, I> {
    fn process_and_write<F, O>(&'a self, string: &str, processor: F)
    where
        F: Fn(I) -> O,
        O: Display;

    fn process_and_write_part_1<F, O>(&'a self, processor: F)
    where
        F: Fn(I) -> O,
        O: Display;

    fn process_and_write_part_2<F, O>(&'a self, processor: F)
    where
        F: Fn(I) -> O,
        O: Display;
}

impl<'a, I> ProcessAndWrite<'a, &'a [I]> for Runner<Vec<I>> {
    fn process_and_write<F, O>(&'a self, string: &str, processor: F)
    where
        F: Fn(&'a [I]) -> O,
        O: Display,
    {
        let now = start_timer();
        let result = processor(&self.input);
        stop_timer_and_write(now, string, result);
    }

    fn process_and_write_part_1<F, O>(&'a self, processor: F)
    where
        F: Fn(&'a [I]) -> O,
        O: Display,
    {
        self.process_and_write("Part 1", processor)
    }

    fn process_and_write_part_2<F, O>(&'a self, processor: F)
    where
        F: Fn(&'a [I]) -> O,
        O: Display,
    {
        self.process_and_write("Part 2", processor)
    }
}

impl<'a, I> ProcessAndWrite<'a, &'a I> for Runner<I> {
    fn process_and_write<F, O>(&'a self, string: &str, processor: F)
    where
        F: Fn(&'a I) -> O,
        O: Display,
    {
        let now = start_timer();
        let result = processor(&self.input);
        stop_timer_and_write(now, string, result);
    }

    fn process_and_write_part_1<F, O>(&'a self, processor: F)
    where
        F: Fn(&'a I) -> O,
        O: Display,
    {
        self.process_and_write("Part 1", processor)
    }

    fn process_and_write_part_2<F, O>(&'a self, processor: F)
    where
        F: Fn(&'a I) -> O,
        O: Display,
    {
        self.process_and_write("Part 2", processor)
    }
}

use std::time::Instant;

fn start_timer() -> Instant {
    Instant::now()
}

fn stop_timer_and_write<O>(now: Instant, string: &str, result: O)
where
    O: Display,
{
    let elapsed = now.elapsed();
    println!("{}", string);
    println!("Duration: {:#?}", elapsed);
    println!("----------------");
    println!("{}", result);
    println!();
}

pub trait ProcessAndAssert<'a, I> {
    fn process_and_assert<F, O>(&'a self, processor: F, expected: O)
    where
        F: Fn(I) -> O,
        O: PartialEq,
        O: Debug;
}

impl<'a, I> ProcessAndAssert<'a, &'a [I]> for Runner<Vec<I>> {
    fn process_and_assert<F, O>(&'a self, processor: F, expected: O)
    where
        F: Fn(&'a [I]) -> O,
        O: PartialEq,
        O: Debug,
    {
        assert_eq!(processor(&self.input), expected)
    }
}

impl<'a, I> ProcessAndAssert<'a, &'a I> for Runner<I> {
    fn process_and_assert<F, O>(&'a self, processor: F, expected: O)
    where
        F: Fn(&'a I) -> O,
        O: PartialEq,
        O: Debug,
    {
        assert_eq!(processor(&self.input), expected)
    }
}
