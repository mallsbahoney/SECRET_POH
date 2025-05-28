// SequenceLink Schema Implementation
use crate::{SchemaParams, ChainState, SchemaError};

pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    println!("Executing schema: SequenceLink");
    state.log_execution("SequenceLink", params)?;
    Ok(())
}
