pub mod service;
pub mod controller;

use crate::service::{RPNSolver};
use crate::controller::ConsoleController;

fn main() {
    let solver = RPNSolver {};
    let controller = ConsoleController::new(&solver);

    controller.mainloop();
}
