
use super::*;


#[derive(Debug,Clone,Copy)]
pub enum PlayerActions{
    MovePlayerUp,
    MovePlayerDown,
    MovePlayerLeft,
    MovePlayerRight,

    Demolish,
    MakeFactory,
    MakeDefault,
    NoAction,
}


impl std::fmt::Display for PlayerActions{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}





impl PlayerActions{

    //returns something that we can use to iterate the playeractions enumn
    //make into macro or something that upkeeps it self
    fn get_enum_iterator()->[PlayerActions;8]{
        use PlayerActions::*;
        [MovePlayerRight,MovePlayerLeft,MovePlayerDown,MovePlayerUp,Demolish,MakeDefault,MakeFactory,NoAction]
    }



    #[allow(non_snake_case)]
    pub fn apply_effect(self,game: &mut super::MainState){
        match self {
            PlayerActions::MovePlayerUp=> game.change_player_location_y(-1.0),
            PlayerActions::MovePlayerDown=>game.change_player_location_y(1.0),
            PlayerActions::MovePlayerLeft=>game.change_player_location_x(-1.0),
            PlayerActions::MovePlayerRight=>game.change_player_location_x(1.0),

            PlayerActions::Demolish=>panic!("not implemented"),
            PlayerActions::MakeFactory=>game.check_and_place_tile(State::FactoryBlock),
            PlayerActions::MakeDefault=>game.check_and_place_tile(State::DefaultBlock),
            PlayerActions::NoAction=>(),
        }

    }


    pub fn get_enum_from_string(find:&str)->Self{
        for i in PlayerActions::get_enum_iterator(){
            if i.to_string() == find{
                return i;
            }
        }
        return PlayerActions::NoAction;
    }

}
