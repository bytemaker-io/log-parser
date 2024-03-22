pub struct DataFrame {
    time:String,
    component:String,
    peer_component:String,
    event_description:String,
    port:i32,
    function_name:String,
    message:String,
}

impl DataFrame {
    pub fn new(time:String,component:String,peer_component:String,event_description:String,port:i32,function_name:String,message:String) -> DataFrame {
        DataFrame {
            time,
            component,
            peer_component,
            event_description,
            port,
            function_name,
            message,
        }
    }
    pub fn get_time(&self) -> String {
        self.time.clone()
    }
    pub fn get_component(&self) -> String {
        self.component.clone()
    }
    pub fn get_peer_component(&self) -> String {
        self.peer_component.clone()
    }
    pub fn get_event_description(&self) -> String {
        self.event_description.clone()
    }
    pub fn get_port(&self) -> i32 {
        self.port
    }
    pub fn get_function_name(&self) -> String {
        self.function_name.clone()
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn set_time(&mut self,time:String) {
        self.time = time;
    }
    pub fn set_component(&mut self,component:String) {
        self.component = component;
    }
    pub fn set_peer_component(&mut self,peer_component:String) {
        self.peer_component = peer_component;
    }
    pub fn set_event_description(&mut self,event_description:String) {
        self.event_description = event_description;
    }
    pub fn set_port(&mut self,port:i32) {
        self.port = port;
    }

    pub fn set_function_name(&mut self,function_name:String) {
        self.function_name = function_name;
    }
    pub fn set_message(&mut self,message:String) {
        self.message = message;
    }

}