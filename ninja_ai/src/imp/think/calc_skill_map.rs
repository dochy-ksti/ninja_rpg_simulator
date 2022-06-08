use crate::imp::structs::ai::skillmap::available_trainings::{AvailableTraining, AvailableTrainings};
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::training_drainer::TrainingDrainer;
use crate::imp::structs::ai::skillmap::trainings::Trainings;

pub(crate) fn calc_skill_map(mut trainings : Trainings) -> SkillMap{
    let mut skillmap = SkillMap::new();
    let mut availables = AvailableTrainings::new();
    let mut achieved : Option<u32> = Some(0);
    loop{
        if trainings.move_items(&mut availables, achieved) == false{ break; }

        if let Some(item) = availables.pop(){
            if item.training().repeatable(){
                //repeatableより低いslopeのトレーニングはもう全くやる必要がないので削除
                availables.clear_heap();
                achieved = None;
                skillmap.set_repeatable(item.t)
            }
        }
    }

}