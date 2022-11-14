pub mod intcode {
    use std::collections::VecDeque;

    pub enum ParameterMode {
        Position,
        Immediate,
    }

    pub enum Operation {
        Add {
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        },
        Multiply {
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        },
        Input,
        Output(ParameterMode),
        JumpIfTrue {
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        },
        JumpIfFalse {
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        },
        LessThan {
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        },
        Equals {
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        },
        Terminate,
    }

    pub struct ParseOperationError;

    impl ParameterMode {
        fn from_char(c: char) -> Result<ParameterMode, ParseOperationError> {
            c.to_string()
                .parse::<i32>()
                .map_err(|_| ParseOperationError)
                .and_then(|i| match i {
                    0 => Ok(ParameterMode::Position),
                    1 => Ok(ParameterMode::Immediate),
                    _ => Err(ParseOperationError),
                })
        }
    }

    impl Operation {
        fn from_single_digit_integer(integer: i32) -> Result<Operation, ParseOperationError> {
            match integer {
                1 => Ok(Operation::Add {
                    fst_mode: ParameterMode::Position,
                    snd_mode: ParameterMode::Position,
                }),
                2 => Ok(Operation::Multiply {
                    fst_mode: ParameterMode::Position,
                    snd_mode: ParameterMode::Position,
                }),
                3 => Ok(Operation::Input),
                4 => Ok(Operation::Output(ParameterMode::Position)),
                5 => Ok(Operation::JumpIfTrue {
                    fst_mode: ParameterMode::Position,
                    snd_mode: ParameterMode::Position,
                }),
                6 => Ok(Operation::JumpIfFalse {
                    fst_mode: ParameterMode::Position,
                    snd_mode: ParameterMode::Position,
                }),
                7 => Ok(Operation::LessThan {
                    fst_mode: ParameterMode::Position,
                    snd_mode: ParameterMode::Position,
                }),
                8 => Ok(Operation::Equals {
                    fst_mode: ParameterMode::Position,
                    snd_mode: ParameterMode::Position,
                }),
                _ => Err(ParseOperationError),
            }
        }

        fn from_three_digit_integer(integer: i32) -> Result<Operation, ParseOperationError> {
            match integer {
                101 => Ok(Operation::Add {
                    fst_mode: ParameterMode::Immediate,
                    snd_mode: ParameterMode::Position,
                }),
                102 => Ok(Operation::Multiply {
                    fst_mode: ParameterMode::Immediate,
                    snd_mode: ParameterMode::Position,
                }),
                104 => Ok(Operation::Output(ParameterMode::Immediate)),
                105 => Ok(Operation::JumpIfTrue {
                    fst_mode: ParameterMode::Immediate,
                    snd_mode: ParameterMode::Position,
                }),
                106 => Ok(Operation::JumpIfFalse {
                    fst_mode: ParameterMode::Immediate,
                    snd_mode: ParameterMode::Position,
                }),
                107 => Ok(Operation::LessThan {
                    fst_mode: ParameterMode::Immediate,
                    snd_mode: ParameterMode::Position,
                }),
                108 => Ok(Operation::Equals {
                    fst_mode: ParameterMode::Immediate,
                    snd_mode: ParameterMode::Position,
                }),
                _ => Err(ParseOperationError),
            }
        }

        fn from_four_digit_integer(digits: &[char]) -> Result<Operation, ParseOperationError> {
            format!("{}{}", digits[2], digits[3])
                .parse::<i32>()
                .map_err(|_| ParseOperationError)
                .and_then(|i| ParameterMode::from_char(digits[1]).map(|mode| (i, mode)))
                .and_then(|(i, fst_mode)| {
                    ParameterMode::from_char(digits[0]).map(|mode| (i, fst_mode, mode))
                })
                .and_then(|(i, fst_mode, snd_mode)| match i {
                    1 => Ok(Operation::Add { fst_mode, snd_mode }),
                    2 => Ok(Operation::Multiply { fst_mode, snd_mode }),
                    5 => Ok(Operation::JumpIfTrue { fst_mode, snd_mode }),
                    6 => Ok(Operation::JumpIfFalse { fst_mode, snd_mode }),
                    7 => Ok(Operation::LessThan { fst_mode, snd_mode }),
                    8 => Ok(Operation::Equals { fst_mode, snd_mode }),
                    _ => Err(ParseOperationError),
                })
        }

        fn from_integer(integer: i32) -> Result<Operation, ParseOperationError> {
            let digits: Vec<char> = integer.to_string().chars().collect();
            match digits.len() {
                1 => Operation::from_single_digit_integer(integer),
                2 => {
                    if integer != 99 {
                        Err(ParseOperationError)
                    } else {
                        Ok(Operation::Terminate)
                    }
                }
                3 => Operation::from_three_digit_integer(integer),
                4 => Operation::from_four_digit_integer(&digits),
                _ => Err(ParseOperationError),
            }
        }
    }

    pub fn parse_code(str: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
        return str.trim().split(',').map(|x| x.parse::<i32>()).collect();
    }

    #[derive(PartialEq, Eq)]
    pub enum HaltReason
    {
        Terminate,
        Input
    }

    #[derive(Debug)]
    pub struct ExecError;

    pub struct Computer {
        memory: Vec<i32>,
        ptr: usize,
        input: VecDeque<i32>,
        output: Vec<i32>,
    }

    impl Computer {
        pub fn new(code: &[i32], input: Vec<i32>) -> Computer {
            Computer {
                memory: Vec::<i32>::from(code),
                ptr: 0,
                input: VecDeque::<i32>::from(input),
                output: Vec::<i32>::new(),
            }
        }

        fn exec_add(
            &mut self,
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        ) -> Result<(), ExecError> {
            let left_operand = match fst_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 1] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 1],
            };

            let right_operand = match snd_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 2] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 2],
            };

            let dest = self.memory[self.ptr + 3] as usize;
            self.memory[dest] = left_operand + right_operand;
            self.ptr += 4;
            Ok(())
        }

        fn exec_multiply(
            &mut self,
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        ) -> Result<(), ExecError> {
            let left_operand = match fst_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 1] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 1],
            };

            let right_operand = match snd_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 2] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 2],
            };

            let dest = self.memory[self.ptr + 3] as usize;
            self.memory[dest] = left_operand * right_operand;
            self.ptr += 4;
            Ok(())
        }

        fn exec_jump_if_true(
            &mut self,
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        ) -> Result<(), ExecError> {
            let value = match fst_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 1] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 1],
            };

            if value != 0 {
                let destination = match snd_mode {
                    ParameterMode::Position => self.memory[self.memory[self.ptr + 2] as usize],
                    ParameterMode::Immediate => self.memory[self.ptr + 2],
                };

                self.ptr = destination as usize;
                Ok(())
            } else {
                self.ptr += 3;
                Ok(())
            }
        }

        fn exec_jump_if_false(
            &mut self,
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        ) -> Result<(), ExecError> {
            let value = match fst_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 1] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 1],
            };

            if value == 0 {
                let destination = match snd_mode {
                    ParameterMode::Position => self.memory[self.memory[self.ptr + 2] as usize],
                    ParameterMode::Immediate => self.memory[self.ptr + 2],
                };

                self.ptr = destination as usize;
                Ok(())
            } else {
                self.ptr += 3;
                Ok(())
            }
        }

        fn exec_less_than(
            &mut self,
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        ) -> Result<(), ExecError> {
            let left_operand = match fst_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 1] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 1],
            };

            let right_operand = match snd_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 2] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 2],
            };

            let dest = self.memory[self.ptr + 3] as usize;
            self.memory[dest] = if left_operand < right_operand { 1 } else { 0 };
            self.ptr += 4;
            Ok(())
        }

        fn exec_equals(
            &mut self,
            fst_mode: ParameterMode,
            snd_mode: ParameterMode,
        ) -> Result<(), ExecError> {
            let left_operand = match fst_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 1] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 1],
            };

            let right_operand = match snd_mode {
                ParameterMode::Position => self.memory[self.memory[self.ptr + 2] as usize],
                ParameterMode::Immediate => self.memory[self.ptr + 2],
            };

            let dest = self.memory[self.ptr + 3] as usize;
            self.memory[dest] = if left_operand == right_operand { 1 } else { 0 };
            self.ptr += 4;
            Ok(())
        }

        fn exec_op(&mut self, op: Operation) -> Result<(), ExecError> {
            match op {
                Operation::Add { fst_mode, snd_mode } => self.exec_add(fst_mode, snd_mode),
                Operation::Multiply { fst_mode, snd_mode } => {
                    self.exec_multiply(fst_mode, snd_mode)
                }
                Operation::Input => match self.input.pop_front() {
                    None => Err(ExecError),
                    Some(i) => {
                        let dest = self.memory[self.ptr + 1];
                        self.memory[dest as usize] = i;
                        self.ptr += 2;
                        Ok(())
                    }
                },
                Operation::Output(mode) => match mode {
                    ParameterMode::Position => {
                        self.output
                            .push(self.memory[self.memory[self.ptr + 1] as usize]);
                        self.ptr += 2;
                        Ok(())
                    }
                    ParameterMode::Immediate => {
                        self.output.push(self.memory[self.ptr + 1]);
                        self.ptr += 2;
                        Ok(())
                    }
                },
                Operation::JumpIfTrue { fst_mode, snd_mode } => {
                    self.exec_jump_if_true(fst_mode, snd_mode)
                }
                Operation::JumpIfFalse { fst_mode, snd_mode } => {
                    self.exec_jump_if_false(fst_mode, snd_mode)
                }
                Operation::LessThan { fst_mode, snd_mode } => {
                    self.exec_less_than(fst_mode, snd_mode)
                }
                Operation::Equals { fst_mode, snd_mode } => {
                    self.exec_equals(fst_mode, snd_mode)
                }
                Operation::Terminate => Err(ExecError),
            }
        }

        pub fn run(&mut self) -> Result<(), ExecError> {
            Operation::from_integer(self.memory[self.ptr])
                .map_err(|_| ExecError {})
                .and_then(|op| match op {
                    Operation::Terminate => Ok(()),
                    _ => self.exec_op(op).and_then(|_| self.run()),
                })
        }

        pub fn run_adv(&mut self) -> Result<HaltReason, ExecError> {
            Operation::from_integer(self.memory[self.ptr])
                .map_err(|_| ExecError {})
                .and_then(|op| match op {
                    Operation::Terminate => Ok(HaltReason::Terminate),
                    Operation::Input if self.input.is_empty() => Ok(HaltReason::Input),
                    _ => self.exec_op(op).and_then(|_| self.run_adv()),
                })
        }

        pub fn push_input(&mut self, i: i32)
        {
            self.input.push_back(i)
        }

        pub fn output(&self) -> &[i32] {
            &self.output
        }

        pub fn pop_output(&mut self) -> Option<i32>
        {
            self.output.pop()
        }
    }
}
