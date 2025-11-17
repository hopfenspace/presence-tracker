use galvyn::rorm::Model;
use galvyn::rorm::fields::types::MaxStr;

#[derive(Model)]
pub struct LocationModel {
    #[rorm(primary_key)]
    pub location: MaxStr<255>,
}
