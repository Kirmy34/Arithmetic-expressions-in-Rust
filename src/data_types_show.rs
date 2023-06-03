use crate::data_types::DataTypes;

impl DataTypes {
    pub fn show(&self) -> String {
        match self {
            Self::TInt => String::from("number"),
            Self::TBool => String::from("bool"),
        }
    }
}
