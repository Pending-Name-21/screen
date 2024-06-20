use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MyMouseButton {
    MyLeft,
    MyRight,
    MyMiddle,
    MyOther(u16),
}
