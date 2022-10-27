/// Represents an exported function signature.
#[derive(PartialEq, Eq, Debug, Default, Clone, Hash)]
pub struct Export<'input> {
    /// The export name.
    pub name: &'input str,

    /// The WIT function type being exported.
    pub function_type: u32,
}
