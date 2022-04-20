pub struct NjEvent{
    id : String
}

impl NjEvent{
    pub fn id(&self) -> &str{ &self.id }
}