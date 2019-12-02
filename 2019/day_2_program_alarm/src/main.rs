struct IntComputer {
    program_counter: usize,
    memory: Vec<u32>,
}

impl IntComputer {
    fn evaluate(&mut self) -> &Self {
        loop {
            let opcode = self.memory[self.program_counter];
            match opcode {
                1 => {
                    let read_address_1 = self.memory[self.program_counter + 1] as usize;
                    let read_address_2 = self.memory[self.program_counter + 2] as usize;
                    let store_address = self.memory[self.program_counter + 3] as usize;
                    self.memory[store_address] =
                        self.memory[read_address_1] + self.memory[read_address_2];
                }
                2 => {
                    let read_address_1 = self.memory[self.program_counter + 1] as usize;
                    let read_address_2 = self.memory[self.program_counter + 2] as usize;
                    let store_address = self.memory[self.program_counter + 3] as usize;
                    self.memory[store_address] =
                        self.memory[read_address_1] * self.memory[read_address_2];
                }
                99 => return self,
                _ => {
                    panic!("Unexpected opcode");
                }
            };
            self.program_counter += 4;
        }
    }
}

#[test]
fn test_evaluate() {
    assert_eq!(
        IntComputer {
            program_counter: 0,
            memory: vec!(1, 0, 0, 0, 99)
        }
        .evaluate()
        .memory,
        vec!(2, 0, 0, 0, 99)
    );
    assert_eq!(
        IntComputer {
            program_counter: 0,
            memory: vec!(2, 3, 0, 3, 99)
        }
        .evaluate()
        .memory,
        vec!(2, 3, 0, 6, 99)
    );
    assert_eq!(
        IntComputer {
            program_counter: 0,
            memory: vec!(2, 4, 4, 5, 99, 0)
        }
        .evaluate()
        .memory,
        vec!(2, 4, 4, 5, 99, 9801)
    );
    assert_eq!(
        IntComputer {
            program_counter: 0,
            memory: vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)
        }
        .evaluate()
        .memory,
        vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
    );
}

fn main() -> Result<(), std::num::ParseIntError> {
    let input = include_str!("../input");
    let mut memory = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()?;
    memory[1] = 12;
    memory[2] = 2;
    let mut computer = IntComputer {
        memory,
        program_counter: 0,
    };
    computer.evaluate();
    println!("{}", computer.memory[0]);

    Ok(())
}
