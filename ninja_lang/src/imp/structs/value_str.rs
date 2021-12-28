pub(crate) struct ValueStr{
    kind : Option<u8>,
    first : String,
    second : Option<String>,
}

impl ValueStr{
    pub(crate) fn new(kind : Option<u8>, first : String, second : Option<String>) -> ValueStr{
        debug_assert!((kind.is_none() && second.is_some()) == false);
        ValueStr{ kind, first, second }
    }
    pub(crate) fn kind(&self) -> Option<u8>{ self.kind }
    pub(crate) fn first(&self) -> &str{ &self.first }
    pub(crate) fn second(&self) -> Option<&str>{ self.second.as_ref().map(|s|s) }
    pub(crate) fn deconstruct(self) -> (Option<u8>, String, Option<String>){ (self.kind, self.first, self.second) }

    pub(crate) fn to_string(&self) -> String{
        if let Some(kind) = self.kind{
            if let Some(second) = &self.second{
                format!("{}:{}:{}",kind, &self.first, second)
            } else{
                format!("{}:{}",kind, &self.first)
            }
        } else{
            self.first.to_string()
        }

    }
}