use neon::prelude::*;

use crate::instance::Instance;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("initialize", get_num_cpus)?;
    Ok(())
}