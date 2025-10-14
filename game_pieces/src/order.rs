use graph_cycles::Cycles;
use petgraph::{Directed, Graph, graph::NodeIndex};

use crate::province::ProvinceID;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Hold,
    Move,
    Support,
    Convoy,
}

#[derive(Clone, Copy)]
pub struct Order {
    // The Order itself (e.g. `Move`)
    order_type: OrderType,

    // The location of the unit receiving this order.
    order_of: ProvinceID,

    // The origin for the order. Different than `order_of` for orders such as `Support` or `Convoy`
    order_from: ProvinceID,

    // The destination for the order.
    order_to: ProvinceID,

    // The strength of an order. The default, and by far most common value, is 1. This can only be increased with supports.
    order_strength: u8,
}

impl Order {
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
}

pub fn create_order_dependency_graph(orders: Vec<Order>) -> Graph<Order, (), Directed> {
    let mut ret_graph = Graph::<Order, ()>::new();

    let mut indexes: Vec<(Order, NodeIndex)> = vec![];

    for order in &orders {
        indexes.push((*order, ret_graph.add_node(*order)));
    }

    // All dependencies between orders are as follows:
    // 1. A holding (including supporting and convoying) unit is dependant on all units supporting it to hold. A moving unit is dependant on all units supporting its move.
    // 2. A unit moving into a location is dependant on any unit already in that location.
    // 3. A unit moving into a location is dependant on any unit also moving into that location. (Note that this immediately causes a cycle)
    // 4. A unit moving into a location is dependant on any unit moving from its destination to the original units origin (Causes a cycle)
    // 5. A unit moving into a location is dependant on any unit that is convoying it. The combination of rules 1, 2, and 5 can lead to a Convoy Paradox cycle.

    for (current_order, current_order_idx) in &indexes {
        match current_order.order_type {
            OrderType::Hold | OrderType::Convoy | OrderType::Support => {
                for (check_order, check_order_idx) in &indexes {
                    // If we are Holding, then we are only dependant on moves that support hold us.
                    if check_order.is_support_holding(current_order.order_of) {
                        ret_graph.add_edge(*current_order_idx, *check_order_idx, ());
                    }
                }
            }
            OrderType::Move => {
                for (check_order, check_order_idx) in &indexes {
                    // If we are moving, then we are dependant on moves that support us to move
                    if check_order
                        .is_support_moving(current_order.order_from, current_order.order_to) ||
                    // Any unit that is already at the location (even if it's trying to move out)
                    (check_order.order_of == current_order.order_to) ||
                    // Any unit that is also trying to move to this location
                    (check_order.order_to == current_order.order_to) ||
                    // Any unit that is trying to move through this unit (i.e. swapping)
                    (check_order.order_to == current_order.order_from
                        && check_order.order_from == current_order.order_to) ||
                    // Any unit that is trying to convoy this unit
                    check_order.is_convoying(current_order.order_from, current_order.order_to)
                    {
                        ret_graph.add_edge(*current_order_idx, *check_order_idx, ());
                    }
                }
            }
        }
    }

    ret_graph
}
