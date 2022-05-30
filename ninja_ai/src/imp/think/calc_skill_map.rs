use crate::imp::structs::ai::skillmap::available_trainings::AvailableTrainings;
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::training_drainer::TrainingDrainer;
use crate::imp::structs::ai::skillmap::trainings::Trainings;

pub(crate) fn calc_skill_map(mut trainings : Trainings) -> SkillMap{
    let mut availables = AvailableTrainings::new();
    while let Some(mut drainer) = trainings.drain(){
        while let Some(training) = drainer.next(){

        }

    }
}