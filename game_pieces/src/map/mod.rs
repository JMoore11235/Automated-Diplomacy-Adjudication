use crate::{
    province::{Province, ProvinceID},
    unit::UnitType,
};

pub struct Connection {
    province_1_id: ProvinceID,
    province_2_id: ProvinceID,
    allowed_unit_types: Vec<UnitType>,
}

impl Connection {
    pub fn new(
        province_1_id: ProvinceID,
        province_2_id: ProvinceID,
        allowed_unit_types: Vec<UnitType>,
    ) -> Self {
        // TODO: Error handling?
        if province_1_id == province_2_id {
            panic!("Not allowed!")
        }

        let (id1, id2) = {
            if province_1_id < province_2_id {
                (province_1_id, province_2_id)
            } else {
                (province_2_id, province_1_id)
            }
        };
        Self {
            province_1_id: id1,
            province_2_id: id2,
            allowed_unit_types,
        }
    }

    pub fn allowed(&self, unit_type: &UnitType) -> bool {
        self.allowed_unit_types.contains(unit_type)
    }
}

pub struct Map {
    provinces: Vec<Province>,
    connections: Vec<Connection>,
}

impl Map {}
