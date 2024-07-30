// implementations exclusively for things in the model.rs

use actix::Message;

use crate::model::Individual;

/////////////////
// INDIVIDUAL //
///////////////

impl Message for Individual {
    type Result = ();
}