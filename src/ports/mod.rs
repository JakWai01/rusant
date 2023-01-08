use log::debug;

/// Representing the ports used the gstreamer pipeline
pub struct PortPool {
    listening_port_pool: Vec<i32>,
    serving_port: i32,
}

impl PortPool {
    /// Initialize new PortPool
    pub fn new() -> Self {
        PortPool {
            listening_port_pool: Vec::new(),
            serving_port: 5200,
        }
    }

    /// Allocate a new port. This port can't be used in another context
    pub fn allocate_listening_port(&mut self, port: i32) -> Result<(), String> {
        // Check if port is already in use
        match self.listening_port_pool.iter().position(|x| *x == port) {
            // Port already exists
            Some(_x) => Err(format!(
                "Port {} cannot be allocated since it it already in use",
                port
            )),

            // Port does not exist so far and can be allocated
            None => {
                self.listening_port_pool.push(port);

                debug!("Allocating port {} as a listening port", port);

                Ok(())
            }
        }
    }

    /// Free an allocated port
    pub fn free_listening_port(&mut self, port: i32) -> Result<(), String> {
        // Check if port was allocated
        match self.listening_port_pool.iter().position(|x| *x == port) {
            // Port was allocated and can be removed
            Some(x) => {
                self.listening_port_pool.remove(x);

                debug!("Freeing port {}", port);

                Ok(())
            }

            // Port was never allocated in the first place
            None => Err(format!(
                "Port {} cannot be freed since it was not allocated before",
                port
            )),
        }
    }

    /// Get the port of the updsink
    pub fn get_serving_port(&self) -> i32 {
        self.serving_port
    }

    /// Get all ports currently in use as a udpsrc
    pub fn get_listening_port_pool(&self) -> &Vec<i32> {
        &self.listening_port_pool
    }
}
