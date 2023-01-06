use std::fmt::{Display, Debug};

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

    pub fn process_and_write_part_1<F, O>(&self, processor: F)
    where
        F: Fn(&I) -> O,
        O: Display,
    {
        self.process_and_write("Part 1", processor)
    }

    pub fn process_and_write_part_2<F, O>(&self, processor: F)
    where
        F: Fn(&I) -> O,
        O: Display,
    {
        self.process_and_write("Part 2", processor)
    }

    fn process_and_write<F, O>(&self, string: &str, processor: F)
    where
        F: Fn(&I) -> O,
        O: Display,
    {
        use std::time::Instant;

        let now = Instant::now();
        let result = processor(&self.input);
        let elapsed = now.elapsed();

        println!("{}", string);
        println!("Duration: {:#?}", elapsed);
        println!("----------------");
        println!("{}", result);
        println!();
    }

    pub fn process_and_assert<F, O>(&self, processor: F, expected: O)
    where
        F: Fn(&I) -> O,
        O: PartialEq,
        O: Debug,
    {
        assert_eq!(processor(&self.input), expected)
    }
}
