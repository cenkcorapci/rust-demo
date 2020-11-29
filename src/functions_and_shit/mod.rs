use crate::my;

pub fn function_calls_another_function_from_that_other_module() {
    my::nested::function();
}