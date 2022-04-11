use crate::GuiID;

#[derive(Debug, Clone)]
pub struct TextItem {
    id : GuiID,
    title : String,
    next_indexes : Vec<usize>,
    prev_indexes : Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct TextInput {
    items: Vec<TextItem>,
    first : usize
}

impl TextItem {
    pub fn new(title : String, next_indexes : Vec<usize>, prev_indexes : Vec<usize>) -> TextItem {
        TextItem { id : GuiID::new(), title, next_indexes, prev_indexes }
    }
    pub fn id(&self) -> &GuiID{ &self.id }
    pub fn title(&self) -> &str{ &self.title }
    pub fn next_indexes(&self) -> &[usize]{ &self.next_indexes }
    pub fn prev_indexes(&self) -> &[usize]{ &self.prev_indexes }
}

impl TextInput {
    pub fn new(items : Vec<TextItem>, first : usize) -> TextInput { TextInput { items, first } }
    pub fn items(&self) -> &[TextItem]{ &self.items }
    pub fn into_items(self) -> Vec<TextItem>{ self.items }
    pub fn first(&self) -> usize{ self.first }
}
