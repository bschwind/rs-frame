use rs_frame::AppPath;
use serde::{Deserialize, Serialize};

#[derive(AppPath, Debug, PartialEq)]
#[path("/users")]
struct UsersListPath {}

#[test]
fn no_params() {
    let users_path = UsersListPath::from_str("/users").unwrap();
    assert_eq!(users_path, UsersListPath {});
}

#[test]
fn trailing_slash() {
    let users_path = UsersListPath::from_str("/users/");
    assert!(users_path.is_none());
}

#[test]
fn no_leading_slash() {
    let users_path = UsersListPath::from_str("users");
    assert!(users_path.is_none());
}

#[derive(AppPath, Debug, PartialEq)]
#[path("/users/:user_id")]
struct UserDetailPath {
    user_id: u64,
}

#[test]
fn one_param() {
    let users_path = UserDetailPath::from_str("/users/642151").unwrap();
    assert_eq!(users_path, UserDetailPath { user_id: 642151 });
}

#[test]
fn invalid_param_type() {
    let users_path = UserDetailPath::from_str("/users/not_a_u64");
    assert!(users_path.is_none());
}

#[test]
fn one_param_no_leading_slash() {
    let users_path = UserDetailPath::from_str("users/4216");
    assert!(users_path.is_none());
}

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
}

#[test]
fn test_no_query() {
    let app_path_string = "/p/43/exams/10/submissions_expired";
    let expired_path = ExpiredSubmissionsPath::from_str(app_path_string);
    assert!(expired_path.is_some());
}
