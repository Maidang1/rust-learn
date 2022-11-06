use std::fs;
pub struct Read {
    pub name: String,
    data: String,
}

impl Read {
    pub fn new(name: String) -> Read {
      Read { name: name, data: "".to_string() }
    }
    pub fn start_read(&mut self) {
        let res = fs::read_to_string(&self.name);
        match res {
            Ok(data) => {
              self.store_data(&data);
            },
            Err(e) => println!("Error: {}", e),
        }
        println!("start_read {}", self.name);
    }
    pub fn get_data(&self) -> &String {
        &self.data
    }

    fn store_data(&mut self, data: &String) {
        self.data = data.to_string();
    }
}
