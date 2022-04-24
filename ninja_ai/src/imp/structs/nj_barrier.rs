
///スキルはスキル番号を入れる感じに
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NjSkill(pub u32);

pub struct NjBarrier{
    skill : NjSkill,
    val : u32,
}