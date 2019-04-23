use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

pub use rs_frame_macros::AppPath;

pub trait AppPath {
    fn path_pattern() -> String
    where
        Self: Sized;
    fn from_str(app_path: &str) -> Option<Self>
    where
        Self: Sized;
    fn query_string(&self) -> Option<String>;
    fn to_string(&self) -> String;
}

pub struct RouteData {
    pub name: String,
}

pub struct RouteParams {
    pub template: String,
    pub data: RouteData,
}

pub trait Controller {
    fn params(&mut self, params: &RouteParams) -> Option<()>;
    fn start(&self) -> Vec<String>;
    fn stop(&self) -> Vec<String> {
        vec![]
    }
}

struct ControllerState {
    controller: Rc<RefCell<dyn Controller>>,
    prev_params: Option<u64>,
    hasher: Box<Fn() -> u64>,
}

pub struct App {
    controllers: Vec<ControllerState>,
}

impl App {
    pub fn new() -> App {
        App {
            controllers: vec![],
        }
    }

    pub fn add_controller<C: 'static + Controller + Hash>(&mut self, controller: C) {
        let controller_rc = Rc::new(RefCell::new(controller));

        self.controllers.push(ControllerState {
            controller: controller_rc.clone(),
            prev_params: None,
            hasher: Box::new(move || {
                let mut s = DefaultHasher::new();
                controller_rc.borrow_mut().hash(&mut s);
                s.finish()
            }),
        });
    }

    pub fn new_route(&mut self, route: String) {
        println!("new route: {}", route);

        let route_params = RouteParams {
            template: "/whatever".to_string(),
            data: RouteData { name: route },
        };

        for c in &mut self.controllers {
            let new_params = { c.controller.borrow_mut().params(&route_params) };

            let new_params = new_params.map(|_| (c.hasher)());

            // println!("new_params is {:?}", new_params);

            match (&c.prev_params, new_params) {
                (None, None) => {
                    // println!("Do nothing");
                }
                (Some(ref prev_params), Some(ref new_params)) if *prev_params == *new_params => {
                    // println!("Do nothing");
                }
                (None, Some(ref _new_params)) => {
                    // println!("Call start");
                    c.controller.borrow().start();
                }
                (Some(ref _prev_params), None) => {
                    // println!("Call stop");
                    c.controller.borrow().stop();
                }
                (Some(ref prev_params), Some(ref new_params)) if *prev_params != *new_params => {
                    // Restart the controller
                    // println!("Call stop, then start");
                    c.controller.borrow().stop();
                    c.controller.borrow().start();
                }
                _ => {
                    unreachable!();
                }
            }

            c.prev_params = new_params;
        }

        println!();
    }
}
