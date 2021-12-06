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
            if &self.program[self.index] != &Inst::ClBracket {
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
            self.storage
                .set((self.input[self.input_index] as u8) as i64);
            self.input_index += 1;
            Ok(())
        } else {
            Err(BfError::NoInput)
        }
    }
    fn output(&mut self) -> Result<(), BfError> {
        self.output
            .push_str(std::str::from_utf8(&[self.storage.get() as u8]).unwrap());
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

impl<Storage> BfSimu<Storage>
where
    Storage: BfStorageSimu,
{
    pub fn new_array_impl(
        prog_str: &str,
        input_str: &str,
    ) -> Result<BfSimu<BfArrayImplementation>, BfError> {
        let program: Vec<Inst> = BfSimu::<BfArrayImplementation>::program_from_str(prog_str)?;
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
    // TODO: implement this function
    pub fn program_from_str(_prog_str: &str) -> Result<Vec<Inst>, BfError> {
        return Err(BfError::InvalidProgram { invalid_char: '*' });
    }
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
