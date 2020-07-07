pub struct StringBuilder {
    chunks : Vec<Vec<char>>
}

impl StringBuilder {
    pub fn new() -> StringBuilder {
        StringBuilder {
            chunks : Vec::new()
        }
    }

    pub fn append(&mut self, str : String) {
        let chars : Vec<char> = str.chars().collect();
        self.chunks.push(chars);
    }

    pub fn to_string(&self) -> String {
        self.chunks.iter().flatten().collect()
    }
}