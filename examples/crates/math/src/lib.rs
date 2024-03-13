mod bindings;

use crate::bindings::exports::component::math::addition::Guest;

struct Component;

bindings::export!(Component with_types_in bindings);

impl Guest for Component {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
