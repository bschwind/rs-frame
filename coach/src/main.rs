use rs_frame::{App, Controller, RouteParams};
use rs_frame_macros::{Controller, route};

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



#[derive(Controller)]
#[trigger(route("/environments"))]
struct EnvListController {}

#[derive(Controller)]
#[trigger(route("/environments/:id"))]
struct EnvDetailController {
	#[hash]
	id: String,
}

impl EnvDetailController {
	const NAME: &'static str = "env_detail";
}

#[derive(Controller)]
#[trigger(once)]
struct UserBasicController;

impl UserBasicController {
	const NAME: &'static str = "user_basic";
}




#[route("/", "home")]
struct HomeRoute;

#[route("/login", "login")]
#[derive(Debug)]
struct LoginRoute;

#[route("/users", "user_list")]
struct UsersRoute;

#[route("/environments", "env_list")]
struct EnvironmentsRoute;

#[route("/environments/new", "new_env")]
struct NewEnvironmentRoute;

#[route("/environments/:id/detail", "env_detail")]
struct EnvironmentDetailRoute {
	id: String,
}

#[route("/environments/:id/logs", "env_logs")]
struct EnvironmentLogsRoute {
	id: String,
}

#[route("/environments/:id/vars", "env_vars")]
struct EnvironmentVarsRoute {
	id: String,
}











fn main() {
    // let controllers: Vec<Box<dyn Controller>> = vec!(Box::new(EnvironmentDetailController::default()), Box::new(HomeController));

    println!("EnvDetailController name: {}", EnvDetailController::NAME);
    println!("UserBasicController name: {}", UserBasicController::NAME);

    println!("dfssadfd name: {:?}", LoginRoute {});

    let stuff = EnvDetailController {
        id: "".to_string(),
    };

    let mut app = App::new();
    app.add_controller(EnvironmentDetailController::default());
    app.add_controller(HomeController);

    app.new_route("environments/exam-copy".to_string());
    app.new_route("environments/deadline-extension".to_string());
    app.new_route("home".to_string());
    app.new_route("home".to_string());
}
