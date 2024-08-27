use log::trace;

use crate::parser::Node;

pub struct Runner {
    address: usize,
    memory: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl Runner {
    pub fn new(mem_size: usize) -> Runner {
        Runner {
            address: 0,
            memory: vec![0; mem_size],
            output_buffer: vec![],
        }
    }
    fn get_memory(&self) -> u8 {
        self.memory[self.address].clone()
    }
    fn set_memory(&mut self, value: u8) {
        self.memory[self.address] = value;
    }
    fn decrement_pointer(&mut self) {
        if self.address == 0 {
            self.address = self.memory.len() - 1;
            return;
        }

        self.address -= 1;
    }
    fn increment_pointer(&mut self) {
        if self.address == self.memory.len() - 1 {
            self.address = 0;
            return;
        }

        self.address += 1;
    }
    fn decrement(&mut self) {
        let value = self.get_memory();

        if value == u8::MIN {
            self.set_memory(u8::MAX);
            return;
        }

        self.set_memory(value - 1);
    }
    fn increment(&mut self) {
        let value = self.get_memory();

        if value == u8::MAX {
            self.set_memory(u8::MIN);
            return;
        }

        self.set_memory(value + 1);
    }
    fn output(&mut self) {
        let value = self.get_memory();
        self.output_buffer.push(value);
    }
    pub fn run(&mut self, ast: &Vec<Node>) -> String {
        for node in ast {
            match node {
                Node::DecrementPointer => self.decrement_pointer(),
                Node::IncrementPointer => self.increment_pointer(),
                Node::Decrement => self.decrement(),
                Node::Increment => self.increment(),
                Node::Output => self.output(),
                Node::Block(ast) => loop {
                    if self.get_memory() == 0 {
                        break;
                    }

                    self.run(&ast);
                },
            }
            trace!("address: {}", &self.address);
            trace!("memory: {:?}", &self.memory);
            trace!("output buffer: {:?}", &self.output_buffer);
        }

        String::from_utf8_lossy(&self.output_buffer).to_string()
    }
}
