pub struct PortPool {
    listening_port_pool: Vec<i32>,
    serving_port: i32,
}

impl PortPool {
    pub fn new() -> Self {
        PortPool {
            listening_port_pool: Vec::new(),
            serving_port: 5200,
        }
    }

    pub fn allocate_listening_port(&mut self, port: i32) -> Result<(), String> {
        match self.listening_port_pool.iter().position(|x| *x == port) {
            Some(_x) => Err(format!(
                "Port {} cannot be allocated since it it already in use",
                port
            )),
            None => {
                self.listening_port_pool.push(port);
                Ok(())
            }
        }
    }

    pub fn free_listening_port(&mut self, port: i32) -> Result<(), String> {
        match self.listening_port_pool.iter().position(|x| *x == port) {
            Some(x) => {
                self.listening_port_pool.remove(x);
                Ok(())
            }
            None => Err(format!(
                "Port {} cannot be freed since it was not allocated before",
                port
            )),
        }
    }

    pub fn get_serving_port(&self) -> i32 {
        self.serving_port
    }

    pub fn get_listening_port_pool(&self) -> &Vec<i32> {
        &self.listening_port_pool
    }
}
