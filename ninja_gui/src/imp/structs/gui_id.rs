use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GuiID{
    id : Arc<()>
}

impl PartialEq for GuiID{
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.id, &other.id)
    }
}

impl Eq for GuiID{}
impl Hash for GuiID{
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.id).hash(state)
    }
}
