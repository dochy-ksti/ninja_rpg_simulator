use crate::imp::structs::ai::node_id::NodeID;

pub(crate) struct Node{
    id : NodeID,
    to : Vec<NodeID>,
    from : Vec<NodeID>,
}