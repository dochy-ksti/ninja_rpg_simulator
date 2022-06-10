use crate::imp::structs::ai::skillmap::available_trainings::{AvailableTrainings};
use crate::imp::structs::ai::skillmap::skill_map::SkillMap;
use crate::imp::structs::ai::skillmap::slope::Slope;
use crate::imp::structs::ai::skillmap::trainings::Trainings;

pub(crate) fn calc_skill_map(mut trainings : Trainings, current_skill_point : u32) -> SkillMap{
    let mut skillmap = SkillMap::new(current_skill_point);
    let mut availables = AvailableTrainings::new();
    let mut state  = State::Achieved(current_skill_point);
    loop{
        match &state {
            State::Achieved(achieved) => {
                trainings.move_items(&mut availables, *achieved)
            },
            State::Slope(slope) => {
                trainings.move_sloped(&mut availables, *slope)
            }
        };

        if let Some(item) = availables.pop(){
            if item.training_ref().repeatable(){
                //repeatableより低いslopeのトレーニングはやる必要がないので削除
                availables.clear_heap();
                availables.set_current_repeatable(item.clone().training());
                skillmap.set_repeatable_training(item.training());
                state = State::Slope(availables.current_repeatable().unwrap().slope())
            } else{
                state = State::Achieved(skillmap.set_training(item));
            }
        } else{
            if let Some(current_repeatable) = availables.current_repeatable(){
                if skillmap.set_repeatable_training(current_repeatable.clone().training()) == false{
                    return skillmap;
                } else{
                    state = State::Slope(current_repeatable.slope());
                }
            } else{
                return skillmap;
            }
        }
    }

}

enum State{
    Slope(Slope),
    Achieved(u32),
}