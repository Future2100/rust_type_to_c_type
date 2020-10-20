#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum RoleState {
    Down,
    UnInit,
    Replica(bool),
    Master(i64),
}


#[derive(Debug)]
#[repr(C)]
pub enum NodeError {
    LockError,
}


#[derive(Debug)]
#[repr(C)]
pub struct NodeInfo {
    pub node_id: String,
    pub state: RoleState,
}

#[no_mangle]
pub extern "C" fn test(r: Result<RoleState, NodeError>, role: RoleState, err: NodeError, node_info: NodeInfo) -> Result<RoleState, NodeError> {
    println!("{:?}", r);
    r
}

#[repr(C)]
pub struct Test {}


impl Test {
    #[no_mangle]
    pub extern "C" fn test11(&self, n: NodeInfo) -> Result<RoleState, NodeInfo> {
        return Ok(RoleState::UnInit);
    }
}






