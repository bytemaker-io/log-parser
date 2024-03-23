#[derive(Debug)]
pub struct DataFrame {
    time:String,
    component:String,
    peer_component:String,
    event_type:String,
    event_description:String,
    port:i32,
    function_name:String,
    message:String,
}

impl DataFrame {
    pub fn new(time: String, component: String, peer_component: String, event_type: String, event_description: String, port: i32, function_name: String, message: String) -> Self {
        Self { time, component, peer_component, event_type, event_description, port, function_name, message }
    }


    pub fn set_time(&mut self, time: String) {
        self.time = time;
    }
    pub fn set_component(&mut self, component: String) {
        self.component = component;
    }
    pub fn set_peer_component(&mut self, peer_component: String) {
        self.peer_component = peer_component;
    }
    pub fn set_event_type(&mut self, event_type: String) {
        self.event_type = event_type;
    }
    pub fn set_event_description(&mut self, event_description: String) {
        self.event_description = event_description;
    }
    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }
    pub fn set_function_name(&mut self, function_name: String) {
        self.function_name = function_name;
    }
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
    pub fn time(&self) -> &str {
        &self.time
    }
    pub fn component(&self) -> &str {
        &self.component
    }
    pub fn peer_component(&self) -> &str {
        &self.peer_component
    }
    pub fn event_type(&self) -> &str {
        &self.event_type
    }
    pub fn event_description(&self) -> &str {
        &self.event_description
    }
    pub fn port(&self) -> i32 {
        self.port
    }
    pub fn function_name(&self) -> &str {
        &self.function_name
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}