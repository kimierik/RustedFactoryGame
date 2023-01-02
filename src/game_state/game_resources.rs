use super::tile_state::{Building, Material};

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


    pub fn make_instance_with_money(mon:i32) -> Self {
        GameResources {
            total_money: mon,
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

    pub fn add_resource(&mut self, building_info: Building) {
        match building_info.created_material {
            Material::Money => self.add_money(building_info.produced_amount as i32),
            Material::MoneyMultiplier => self.add_multiplier(building_info.produced_amount),
        }
    }

    pub fn add_money(&mut self, value: i32) {
        self.temp_money += value;
    }

    pub fn subtract_money(&mut self, value: i32) {
        self.total_money -= value;
    }

    pub fn add_multiplier(&mut self, val: f32) {
        self.temp_money_multiplier += val;
    }

    pub fn get_money(&self) -> &i32 {
        &self.total_money
    }

    pub fn to_string(&self) -> String {
        format!(
            "money: {} \ncurrent income: {}",
            self.total_money, self.last_collection_income
        )
    }

    //wip
    pub fn serialisable_string(&self)->String{
        format!(":w")
    }


}
