// implementations exclusively for things in the model.rs

use actix::Message;

use crate::{
    model::{Individual, IndividualBuilder, ProcessedIndividualBuilder},
    system::Request,
};

/////////////////
// INDIVIDUAL //
///////////////

impl Message for Request<Individual> {
    type Result = ();
}

///////////////////////////
// PROCESSED INDIVIDUAL //
/////////////////////////

pub fn test() {
    
}
