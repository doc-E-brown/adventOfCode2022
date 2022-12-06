pub(crate) use anyhow::{bail, Result};

pub type Stack = Vec<char>;

#[derive(Debug, PartialEq, Eq)]
pub enum CraneModel {
    CrateMover9001,
    CrateMover9000,
}

#[derive(Debug)]
pub struct Crane {
    pub stacks: Vec<Stack>,
    pub model: CraneModel,
}

impl Crane {
    pub fn new(mut initial_stack: Vec<&str>, model: CraneModel) -> Result<Crane> {
        // Get the number of stacks
        let num_stacks = initial_stack.pop().unwrap();
        let num_stacks: u8 = num_stacks
            .split_terminator(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x: &str| -> u8 {
                match x.parse::<u8>() {
                    Ok(n) => n,
                    Err(_) => 0,
                }
            })
            .max()
            .unwrap_or(0);

        // Construct the Crane
        let mut stacks: Vec<Stack> = Vec::new();
        for _ in 0..num_stacks {
            let mut new_stack: Stack = Vec::new();
            stacks.push(new_stack);
        }

        // Populate the Crane
        initial_stack.reverse();
        for level in &initial_stack {
            let crates: Vec<char> = level.chars().collect::<Vec<char>>();
            for (idx, sample) in crates.chunks(4).into_iter().enumerate() {
                if sample[1] != ' ' {
                    let mut some_stack: &mut Stack = stacks.get_mut(idx).unwrap();
                    some_stack.push(sample[1]);
                }
            }
        }

        let mut Crane = Crane { stacks, model };
        Ok(Crane)
    }

    pub fn execute_move(&mut self, command: &str) -> Result<()> {
        let command: Vec<&str> = command.split_whitespace().collect();
        let qty: usize = command[1].parse::<usize>()?;
        let src: usize = command[3].parse::<usize>()?;
        let dest: usize = command[5].parse::<usize>()?;

        match self.model {
            CraneModel::CrateMover9000 => {
                for _ in 0..qty {
                    self.rpoplpush(src, dest);
                }
            }
            CraneModel::CrateMover9001 => {
                let mut lift: Vec<char> = Vec::new();
                for _ in 0..qty {
                    lift.push(self.rpop(src));
                }
                for _ in 0..qty {
                    self.lpush(dest, lift.pop().unwrap());
                }
            }
        }

        Ok(())
    }

    pub fn rpoplpush(&mut self, src: usize, dest: usize) -> char {
        let src_stack: &mut Stack = self.stacks.get_mut(src - 1).unwrap();
        let val = src_stack.pop().unwrap();
        let dest_stack: &mut Stack = self.stacks.get_mut(dest - 1).unwrap();
        dest_stack.push(val.clone());
        val
    }

    pub fn rpop(&mut self, src: usize) -> char {
        let src_stack: &mut Stack = self.stacks.get_mut(src - 1).unwrap();
        let val = src_stack.pop().unwrap();
        val
    }

    pub fn lpush(&mut self, dest: usize, val: char) {
        let dest_stack: &mut Stack = self.stacks.get_mut(dest - 1).unwrap();
        dest_stack.push(val.clone());
    }

    pub fn top_crates(&self) -> String {
        let mut code = String::new();
        for stack in &self.stacks {
            code.push(stack[stack.len() - 1])
        }
        code
    }
}
