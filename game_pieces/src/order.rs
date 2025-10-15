use graph_cycles::Cycles;
use petgraph::{
    Directed,
    Direction::{Incoming, Outgoing},
    Graph,
    graph::NodeIndex,
    visit::EdgeRef,
};

use crate::province::ProvinceID;

type OrderGraph<'a> = Graph<&'a Order, (), Directed>;
type NodeList<'a> = Vec<(&'a Order, NodeIndex)>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    // These are legal orders for players to give:
    Hold,
    Move,
    Support,
    Convoy,

    // Below are updated order types that are given to units as part of resolution:

    // Something that is never a valid order (i.e. moving between two non-adjacent provinces)
    IllegalOrder,

    // A support or convoy order that was not followed by the required unit to take that support/convoy.
    RequiredOrderNotGiven,

    // A unit that was offering support was moved into by another player.
    SupportCut,
}

use OrderType::*;

pub struct Order {
    // The order the the player gave for this province
    original_order_type: OrderType,

    // The current order; may be updated to a different order than the original one during resolution.
    order_type: OrderType,

    // The location of the unit receiving this order.
    order_of: ProvinceID,

    // The origin for the order. Different than `order_of` for orders such as `Support` or `Convoy`
    order_from: ProvinceID,

    // The destination for the order.
    order_to: ProvinceID,

    // The strength of an order. The default, and by far most common value, is 1. This can only be increased with supports.
    order_strength: u8,

    // Whether or not this order has been fully resolved.
    resolved: bool,

    // Whether or not this unit is dislodged. Note that this may be updated even after `resolved` is true.
    dislodged: bool,
}

impl Order {
    pub fn is_moving_into(&self, destination: ProvinceID) -> bool {
        (self.order_type == OrderType::Move) && (self.order_to == destination)
    }

    pub fn is_support_holding(&self, supporting: ProvinceID) -> bool {
        // We are support holding iff we're supporting, and both the from and to are the same province.
        // If we are support holding this specific province, then return true.
        (self.order_type == OrderType::Support)
            && (self.order_from == supporting)
            && (self.order_to == supporting)
    }

    pub fn is_support_moving(&self, from: ProvinceID, to: ProvinceID) -> bool {
        (self.order_type == OrderType::Support)
            && (self.order_from == from)
            && (self.order_to == to)
    }

    pub fn is_convoying(&self, from: ProvinceID, to: ProvinceID) -> bool {
        (self.order_type == OrderType::Convoy) && (self.order_from == from) && (self.order_to == to)
    }

    pub fn increase_strength(&mut self) {
        self.order_strength += 1;
    }
}

pub fn create_order_dependency_graph<'a>(orders: Vec<&'a Order>) -> (OrderGraph<'a>, NodeList<'a>) {
    let mut ret_graph = OrderGraph::new();

    let mut nodes: NodeList = vec![];

    for order in orders {
        nodes.push((order, ret_graph.add_node(order)));
    }

    // All dependencies between orders are as follows:
    // 1. A holding (including supporting and convoying) unit is dependant on all units supporting it to hold. A moving unit is dependant on all units supporting its move.
    // 2. A supporting unit is dependant on all units moving into its territory
    // 3. A unit moving into a location is dependant on any unit already in that location. (This immediately creates a cycle)
    // 4. A unit moving into a location is dependant on any unit also moving into that location. (Causes a cycle)
    // 5. A unit moving into a location is dependant on any unit moving from its destination to the original units origin (Causes a cycle)
    // 6. A unit moving into a location is dependant on any unit that is convoying it. This can lead to a convoy paradox.
    for (current_order, current_order_idx) in &nodes {
        for (check_order, check_order_idx) in &nodes {
            // Helper function to reduce duplicate edge adding code.
            let mut add_edge = || {
                ret_graph.add_edge(*current_order_idx, *check_order_idx, ());
            };

            match current_order.order_type {
                Hold | Convoy | Support | RequiredOrderNotGiven | SupportCut => {
                    // (1) If we are Holding, then we are only dependant on moves that support hold us.
                    if check_order.is_support_holding(current_order.order_of) ||
                    // (2) If we are holding, then any unit moving into our province may dislodge us or cut support.
                    check_order.is_moving_into(current_order.order_of)
                    {
                        add_edge();
                    }
                }
                IllegalOrder => {
                    // (2) If an illegal order was given, then other units cannot support hold it. Thus we are only dependant on what other units are moving into our province.
                    if check_order.is_moving_into(current_order.order_of) {
                        add_edge();
                    }
                }
                Move => {
                    // (1) If we are moving, then we are dependant on moves that support us to move
                    if check_order
                        .is_support_moving(current_order.order_from, current_order.order_to) ||
                    // (3) Any unit that is already at the location (even if it's trying to move out)
                    (check_order.order_of == current_order.order_to) ||
                    // (4) Any unit that is also trying to move to this location
                    (check_order.order_to == current_order.order_to) ||
                    // (5) Any unit that is trying to move through this unit (i.e. swapping)
                    (check_order.order_to == current_order.order_from
                        && check_order.order_from == current_order.order_to) ||
                    // (6) Any unit that is trying to convoy this unit
                    check_order.is_convoying(current_order.order_from, current_order.order_to)
                    {
                        add_edge();
                    }
                }
            }
        }
    }

    (ret_graph, nodes)
}

// Takes in a depend
pub fn resolve_all_non_dependant_edges<'a>(order_graph: &'a mut OrderGraph, nodes: &'a NodeList) {
    // Whether or not any orders have been resolved this iteration. Starts as true so that we enter the while loop the first time.
    let mut any_resolved = true;

    while any_resolved {
        any_resolved = false;

        for (order, index) in nodes {
            if order_graph.edges_directed(*index, Outgoing).count() == 0 {
                // This means that we have an order with no dependencies; it is ready to be resolved!
                match order.order_type {
                    Support => {
                        // Find the unit that we are supporting, and increase its strength by one, then remove its dependency on this order.
                        let mut incoming_edges = order_graph.edges_directed(*index, Incoming);

                        // There should only ever be exactly one edge dependant on this one. TODO: Make sure this is the case.
                        let dependent_node_index = incoming_edges.next().unwrap().source();

                        // order_graph[dependent_node_index].increase_strength();
                    }
                    _ => {
                        unimplemented!();
                    }
                }
            }
        }
    }
}
