use randoid::randoid;

use crate::domain::model::Id;

pub fn generate_id() -> Id {
    Id { 0: randoid!(10) }
}
