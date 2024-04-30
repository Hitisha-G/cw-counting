use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,PartialEq,Debug,Clone)]
pub enum QueryMsg{
    Value{},
}

#[derive(Deserialize,Serialize,PartialEq,Debug,Clone)]
pub struct ValueResp {
    pub value: u64,
}