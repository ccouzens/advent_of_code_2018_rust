struct IntComputer {
    instruction_pointer: usize,
    memory: Vec<u32>,
}

impl IntComputer {
    fn new(memory: Vec<u32>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
        }
    }

    fn follow(&mut self, offset: usize) -> &mut u32 {
        let address = self.memory[self.instruction_pointer + offset] as usize;
        &mut self.memory[address]
    }

    fn evaluate(&mut self) {
        loop {
            let opcode = self.memory[self.instruction_pointer];
            match opcode {
                1 => {
                    *self.follow(3) = *self.follow(1) + *self.follow(2);
                    self.instruction_pointer += 4;
                }
                2 => {
                    *self.follow(3) = *self.follow(1) * *self.follow(2);
                    self.instruction_pointer += 4;
                }
                99 => return,
                _ => {
                    panic!("Unexpected opcode");
                }
            };
        }
    }
}

fn evaluate(memory: Vec<u32>) -> Vec<u32> {
    let mut computer = IntComputer::new(memory);
    computer.evaluate();
    computer.memory
}

#[test]
fn test_evaluate() {
    assert_eq!(evaluate(vec!(1, 0, 0, 0, 99)), vec!(2, 0, 0, 0, 99));
    assert_eq!(evaluate(vec!(2, 3, 0, 3, 99)), vec!(2, 3, 0, 6, 99));
    assert_eq!(
        evaluate(vec!(2, 4, 4, 5, 99, 0)),
        vec!(2, 4, 4, 5, 99, 9801)
    );
    assert_eq!(
        evaluate(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)),
        vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
    );
}

fn main() -> Result<(), std::num::ParseIntError> {
    let input = include_str!("../input");
    let program = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()?;
    for (noun, verb) in (0..=99)
        .flat_map(|noun| (0..=99).map(move |verb| (noun, verb)))
        .filter(|&(noun, verb)| {
            let mut memory = program.clone();
            memory[1] = noun;
            memory[2] = verb;
            evaluate(memory)[0] == 19690720
        })
    {
        println!(
            "noun: {}, verb: {}, answer: {}",
            noun,
            verb,
            100 * noun + verb
        );
    }
    Ok(())
}
