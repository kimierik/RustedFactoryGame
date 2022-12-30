
pub struct GameResources {
    temp_money: i32,
    temp_money_multiplier: f32,
    total_money: i32,
    last_collection_income: i32,
}

impl GameResources {
    pub fn make_instance() -> Self {
        GameResources {
            total_money: 1,
            temp_money: 0,
            temp_money_multiplier: 1.0,
            last_collection_income: 0,
        }
    }

    fn get_total_collection_money(&self) -> i32 {
        (self.temp_money as f32 * self.temp_money_multiplier) as i32
    }

    //mess
    pub fn collapse_money(&mut self) {
        self.last_collection_income = 0;
        self.last_collection_income += self.get_total_collection_money();
        self.total_money += self.last_collection_income;
        self.temp_money = 0;
        self.temp_money_multiplier = 1.0;
    }

    pub fn add_money(&mut self) {
        self.temp_money += 1;
    }

    pub fn subtract_money(&mut self ,value:i32){
        self.total_money-=value;
    }

    pub fn add_multiplier(&mut self) {
        self.temp_money_multiplier += 0.1;
    }

    pub fn get_money(&self)->&i32{
        &self.total_money
    }

    pub fn to_string(&self) -> String {
        format!(
            "money: {} \ncurrent income: {}",
            self.total_money, self.last_collection_income
        )
    }
}
