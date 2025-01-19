// Structs  
// feld-cli
// -------------------------------------------------------- //

pub struct SimpleCommand {
    pub name: &'static str,
    pub action: fn() -> (),
}
