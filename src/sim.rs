use super::error::BfError;
use super::storage::{BfArrayImplementation, BfStorageSimu};

pub struct BfSimu<Storage>
where
    Storage: BfStorageSimu,
{
    program: Vec<Inst>,
    index: usize,
    output: String,
    input: Vec<char>,
    input_index: usize,
    storage: Storage,
}

pub trait CoreFuncBfSimu {
    fn run(&mut self) -> Result<String, BfError>;
    fn right(&mut self) -> Result<(), BfError>;
    fn left(&mut self) -> Result<(), BfError>;
    fn add(&mut self) -> Result<(), BfError>;
    fn sub(&mut self) -> Result<(), BfError>;
    fn input(&mut self) -> Result<(), BfError>;
    fn output(&mut self) -> Result<(), BfError>;
    fn cl_bracket(&mut self) -> Result<(), BfError>;
    fn find_matching_bracket(&mut self) -> Result<usize, BfError>;
}

impl<Storage> CoreFuncBfSimu for BfSimu<Storage>
where
    Storage: BfStorageSimu,
{
    fn run(&mut self) -> Result<String, BfError> {
        while self.index < self.program.len() {
            let is_cl_bracket: bool = &self.program[self.index] != &Inst::ClBracket;
            match &self.program[self.index] {
                &Inst::Add => self.add()?,
                &Inst::Sub => self.sub()?,
                &Inst::Right => self.right()?,
                &Inst::Left => self.left()?,
                &Inst::OpBracket => (),
                &Inst::ClBracket => self.cl_bracket()?,
                &Inst::Input => self.input()?,
                &Inst::Output => self.output()?,
            }
            if is_cl_bracket {
                self.index += 1;
            }
        }
        Ok(self.output.clone())
    }
    fn right(&mut self) -> Result<(), BfError> {
        self.storage.right();
        Ok(())
    }
    fn left(&mut self) -> Result<(), BfError> {
        self.storage.left();
        Ok(())
    }
    fn add(&mut self) -> Result<(), BfError> {
        self.storage.add();
        Ok(())
    }
    fn sub(&mut self) -> Result<(), BfError> {
        self.storage.sub();
        Ok(())
    }
    fn input(&mut self) -> Result<(), BfError> {
        if self.input_index < self.input.len() {
            self.storage.set(self.input[self.input_index] as u8);
            self.input_index += 1;
            Ok(())
        } else {
            Err(BfError::NoInput)
        }
    }
    fn output(&mut self) -> Result<(), BfError> {
        self.output.push(self.storage.get() as char);
        Ok(())
    }
    fn cl_bracket(&mut self) -> Result<(), BfError> {
        if self.storage.get() == 0 {
            self.index += 1;
            Ok(())
        } else {
            self.index = self.find_matching_bracket()?;
            Ok(())
        }
    }
    fn find_matching_bracket(&mut self) -> Result<usize, BfError> {
        let mut count_closed: u32 = 1;
        let mut count_open: u32 = 0;
        let mut index: usize = self.index;
        loop {
            if index < 1 {
                return Err(BfError::RuntimeError);
            }
            index -= 1;
            match &self.program[index] {
                &Inst::OpBracket => count_open += 1,
                &Inst::ClBracket => count_closed += 1,
                _ => (),
            }
            if count_open == count_closed {
                return Ok(index);
            }
        }
    }
}

impl BfSimu<BfArrayImplementation> {
    pub fn new(prog_str: &str, input_str: &str) -> Result<BfSimu<BfArrayImplementation>, BfError> {
        let program: Vec<Inst> = program_from_str(prog_str)?;
        let index: usize = 0;
        let output: String = String::new();
        let input: Vec<char> = input_str.to_string().chars().collect();
        let input_index: usize = 0;
        let storage: BfArrayImplementation = BfArrayImplementation::new();
        Ok(BfSimu {
            program,
            index,
            output,
            input,
            input_index,
            storage,
        })
    }
}

pub fn program_from_str(prog_str: &str) -> Result<Vec<Inst>, BfError> {
    let mut res: Vec<Inst> = vec![];
    for c in prog_str
        .trim()
        .replace("\n", "")
        .replace("\t", "")
        .replace(' ', "")
        .chars()
    {
        match c {
            '>' => res.push(Inst::Right),
            '<' => res.push(Inst::Left),
            '+' => res.push(Inst::Add),
            '-' => res.push(Inst::Sub),
            '[' => res.push(Inst::OpBracket),
            ']' => res.push(Inst::ClBracket),
            '.' => res.push(Inst::Output),
            ',' => res.push(Inst::Output),
            x => return Err(BfError::InvalidProgram { invalid_char: x }),
        }
    }
    check_valid_prog(&res)?;
    Ok(res)
}

pub fn check_valid_prog(prog: &Vec<Inst>) -> Result<(), BfError> {
    let mut unclosed: u64 = 0;
    for (i, inst) in prog.iter().enumerate() {
        match inst {
            &Inst::OpBracket => unclosed += 1,
            &Inst::ClBracket => {
                if unclosed == 0 {
                    return Err(BfError::InvalidProgramBrackets { unmatched: i });
                } else {
                    unclosed -= 1;
                }
            }
            _ => (),
        }
    }
    Ok(())
}

#[derive(PartialEq)]
pub enum Inst {
    Right,
    Left,
    Add,
    Sub,
    OpBracket,
    ClBracket,
    Input,
    Output,
}
