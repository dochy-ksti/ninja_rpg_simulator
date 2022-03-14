pub struct GuiItem{
    title : String,
    next_indexes : Vec<usize>,
    prev_indexes : Vec<usize>,
}

pub struct GuiItems{
    items: Vec<GuiItem>,
    priority : Vec<usize>
}

impl GuiItem{
    pub fn new(title : String, next_indexes : Vec<usize>, prev_indexes : Vec<usize>) -> GuiItem{
        GuiItem{ title, next_indexes, prev_indexes }
    }
    pub fn title(&self) -> &str{ &self.title }
    pub fn next_indexes(&self) -> &[usize]{ &self.next_indexes }
    pub fn prev_indexes(&self) -> &[usize]{ &self.prev_indexes }
}

impl GuiItems{
    pub fn new(items : Vec<GuiItem>, priority : Vec<usize>) -> GuiItems{ GuiItems{ items, priority } }
    pub fn items(&self) -> &[GuiItem]{ &self.items }
    pub fn priority(&self) -> &[usize]{ &self.priority }
}
