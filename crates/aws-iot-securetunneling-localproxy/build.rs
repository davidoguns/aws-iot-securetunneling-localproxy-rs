//though we are generating the Rust code with each build, we aren't referencing
//the output. Workout for neovim/rust-analyzer integration not seemingly to be
//able to reference OUT_DIR and the generated file despite 
//cargo.buildScripts.enable=true and procMacro.enable=true being enabled by 
//default in Mason's lspconfig.
use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/Message.proto"], &["src/"])?;
    Ok(())
}
