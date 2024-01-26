mod bindings;

use bindings::exports::example::calculator::calculate::Guest;

// Bring the imported add function into scope
use bindings::component::math::addition::add;

struct Component;

impl Guest for Component {
    fn evaluate(_expr: String) -> i32 {
        // Cleverly parse `expr` into values and operations, and evaluate
        // them meticulously.
        add(2, 2)
    }
}
