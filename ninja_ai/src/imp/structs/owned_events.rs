use bit_vec::BitVec;
use crate::imp::structs::event_id::EventID;


pub struct OwnedEvents{
    // Vec<bool> で実装したほうがいいような気がするけど、こっちのほうが実は早いかもしれないし・・・
    // 巨大なデータセットで実際に走らせるテストをして、性能に問題が出たら
    // 変えればよろしかろう
    vec : BitVec<u64>,
}

impl OwnedEvents{
    pub fn new(num_events : usize) -> OwnedEvents{
        let mut vec : BitVec<u64> = BitVec::default();
        vec.grow(num_events, false);
        OwnedEvents{ vec }
    }

    pub fn get(&self, id : EventID) -> bool{
        self.vec.get(id.num() as usize).unwrap()
    }

    pub fn set(&mut self, id : EventID){
        self.vec.set(id.num() as usize, true);
    }

    pub fn iter(&self) -> impl Iterator<Item=(EventID, bool)> + '_{
        self.vec.iter().enumerate().map(|(id, b)|{
            (EventID::new(id as u32), b)
        })
    }
}