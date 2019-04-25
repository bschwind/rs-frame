use rs_frame::{App, AppPath, Controller, RouteParams};
use serde::{Deserialize, Serialize};

#[derive(Default, Hash)]
struct EnvironmentDetailController {
    env_id: String,
}

impl Controller for EnvironmentDetailController {
    fn params(&mut self, params: &RouteParams) -> Option<()> {
        if params.data.name.starts_with("/environments/") {
            let env_id = params.data.name.rsplit("/").next().unwrap();
            self.env_id = env_id.to_string();

            Some(())
        } else {
            None
        }
    }

    fn start(&self) -> Vec<String> {
        println!(
            "Environment detail controller starting with env_id: {}",
            self.env_id
        );
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
        if params.data.name == "/home" {
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

pub struct GenericRoute<T> {
    data: T,
    path_pattern: String,
}

#[derive(Debug)]
pub struct SomeOtherQuery {
    include_all: bool,
}

// #[route("/emails/{email_id}?{query}", "email_detail")]
// #[derive(AppPath)]
// #[path("/emails/{email_id}")]
// /p/{project_id}/exams/active?column=updated_at&direction=desc&keyword=test
// #[path("/p/{project_id}/exams/active")]
#[derive(Deserialize)]
struct ExamListPath {
    project_id: String,

    // #[query]
    column: Option<String>,

    // #[query]
    direction: Option<SortDirection>,

    // #[query]
    keyword: Option<String>,
}

// #[derive(AppPath)]
// #[path("/settings/account/profile")]
struct SelfProfilePath {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubmissionsQuery {
    column: Option<String>,
    direction: Option<SortDirection>,
    keyword: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LimitOffsetQuery {
    limit: Option<u64>,
    offset: Option<u64>,
}

#[derive(AppPath, Debug)]
#[path("/p/:project_id/exams/:exam_id/submissions_expired")]
struct ExpiredSubmissionsPath {
    project_id: String,
    exam_id: u64,

    #[query]
    query: std::option::Option<SubmissionsQuery>,

    #[query]
    limit: Option<LimitOffsetQuery>,

    #[query]
    required_query: LimitOffsetQuery,
}

// impl AppPath for ExpiredSubmissionsPath {
//     fn path_pattern() -> String {
//         r"^/p/(?P<project_id>[^/]+)/exams/(?P<exam_id>[^/]+)/submissions_expired$".to_string()
//     }

//     fn from_str(app_path: &str) -> Option<Self> {
//         let path_pattern = Regex::new(&Self::path_pattern()).ok()?;
//         let captures = path_pattern.captures(app_path)?;

//         // TODO - get query string

//         Some(ExpiredSubmissionsPath {
//             project_id: captures["project_id"].parse().ok()?,
//             exam_id: captures["exam_id"].parse().ok()?,
//             column: None,
//             direction: None,
//             keyword: None,
//         })
//     }

//     fn query_string(&self) -> Option<String> {
//         None
//     }

//     fn to_string(&self) -> String {
//         format!(
//             "/p/{}/exams/{}/submissions_expired",
//             self.project_id, self.exam_id
//         )
//     }
// }

// #[derive(Debug)]
// pub struct MyQuery {
//     name: String,

//     limit_offset: Option<LimitOffsetQuery>,
// }

fn main() {
    println!(
        "Pattern of ExpiredSubmissionsPath: {}",
        ExpiredSubmissionsPath::path_pattern()
    );

    let expired_path: ExpiredSubmissionsPath =
        "/p/43/exams/10/submissions_expired".parse().unwrap();
    println!("expired_path: {:#?}", expired_path);

    let expired_path: ExpiredSubmissionsPath =
        "/p/22/exams/10/submissions_expired?limit=20&keyword=test"
            .parse()
            .unwrap();
    println!("expired_path: {:#?}", expired_path);

    let expired_path: ExpiredSubmissionsPath =
        "/p/43/exams/10/submissions_expired?limit=20&offset=10&column=users.name"
            .parse()
            .unwrap();
    println!("expired_path: {:#?}", expired_path);

    let expired_path: ExpiredSubmissionsPath =
        "/p/43/exams/10/submissions_expired?limit=20&offset=10&column=users.name&direction=asc"
            .parse()
            .unwrap();
    println!("expired_path: {:#?}", expired_path);

    // let start = std::time::Instant::now();

    // let mut sum: u64 = 0;
    // for _ in 0..100_000 {
    //     let app_path_string =
    //         "/p/43/exams/11/submissions_expired?limit=20&offset=10&column=users.name&direction=asc";
    //     let expired_path = ExpiredSubmissionsPath::from_str(app_path_string);

    //     sum += expired_path.unwrap().exam_id;
    // }

    // println!("Sum: {}", sum);
    // println!("Time: {:?}", (std::time::Instant::now() - start) / 100_000);

    // println!("expired_path URL: {}", expired_path.unwrap().to_string());

    // TODO - object safety
    // let routes: Vec<Box<dyn AppPath>> = vec![Box::new(expired_path.unwrap())];

    // for route in routes {
    //     println!("Route: {}", route);
    // }

    // let controllers: Vec<Box<dyn Controller>> = vec!(Box::new(EnvironmentDetailController::default()), Box::new(HomeController));

    // let email_path = ExamListPath {
    //  email_id: "some_email".to_string(),
    //  limit_offset: Some(LimitOffsetQuery {
    //      limit: Some(20),
    //      offset: Some(20),
    //  }),
    //  some_other: SomeOtherQuery {
    //      include_all: false
    //  }
    // };

    // println!("Query string: {}", email_path.query_string());

    let mut app = App::new();
    app.add_controller(EnvironmentDetailController::default());
    app.add_controller(HomeController);

    app.new_route("/environments/exam-copy".to_string());
    app.new_route("/environments/deadline-extension".to_string());
    app.new_route("/home".to_string());
    app.new_route("/home".to_string());
}
