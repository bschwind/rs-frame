use rs_frame::{App, Controller, RouteParams};

#[derive(Default, Hash)]
struct EnvironmentDetailController {
	env_id: String,
}

impl Controller for EnvironmentDetailController {
	fn params(&mut self, params: &RouteParams) -> Option<()> {
		if params.data.name.starts_with("environments/") {
			let env_id = params.data.name.rsplit("/").next().unwrap();
			self.env_id = env_id.to_string();

			Some(())
		} else {
			None
		}
	}

	fn start(&self) -> Vec<String> {
		println!("Environment detail controller starting with env_id: {}", self.env_id);
		vec![format!("load env {}", self.env_id)]
	}

	fn stop(&self) -> Vec<String> {
		println!("Environment detail controller stopping");
		vec![]
	}
}

#[derive(Hash)]
struct HomeController;

impl Controller for HomeController {
	fn params(&mut self, params: &RouteParams) -> Option<()> {
		if params.data.name == "home" {
			Some(())
		} else {
			None
		}
	}

	fn start(&self) -> Vec<String> {
		println!("Home controller start!");
		vec![format!("load the home screen")]
	}
}

fn main() {
    // let controllers: Vec<Box<dyn Controller>> = vec!(Box::new(EnvironmentDetailController::default()), Box::new(HomeController));

    let mut app = App::new();
    app.add_controller(EnvironmentDetailController::default());
    app.add_controller(HomeController);

    app.new_route("environments/exam-copy".to_string());
    app.new_route("environments/deadline-extension".to_string());
    app.new_route("home".to_string());
    app.new_route("home".to_string());
}
