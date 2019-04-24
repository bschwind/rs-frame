use rs_frame::AppPath;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize)]
struct SubmissionsQuery {
    column: Option<String>,
    direction: Option<SortDirection>,
    keyword: Option<String>,
}

#[derive(Debug, Deserialize)]
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
