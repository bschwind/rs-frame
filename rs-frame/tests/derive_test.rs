use rs_frame::AppPath;
use serde::{Deserialize, Serialize};

#[derive(AppPath, Debug, PartialEq)]
#[path("/users")]
struct UsersListPath {}

#[test]
fn no_params() {
    let path = UsersListPath::from_str("/users").unwrap();
    assert_eq!(path, UsersListPath {});
}

#[test]
fn trailing_slash() {
    let path = UsersListPath::from_str("/users/");
    assert!(path.is_none());
}

#[test]
fn no_leading_slash() {
    let path = UsersListPath::from_str("users");
    assert!(path.is_none());
}

#[derive(AppPath, Debug, PartialEq)]
#[path("/users/:user_id")]
struct UserDetailPath {
    user_id: u64,
}

#[test]
fn one_param() {
    let path = UserDetailPath::from_str("/users/642151").unwrap();
    assert_eq!(path, UserDetailPath { user_id: 642151 });
}

#[test]
fn invalid_param_type() {
    let path = UserDetailPath::from_str("/users/not_a_u64");
    assert!(path.is_none());
}

#[test]
fn one_param_no_leading_slash() {
    let path = UserDetailPath::from_str("users/4216");
    assert!(path.is_none());
}

#[derive(AppPath, Debug, PartialEq)]
#[path("/users/:user_id/friends/:friend_name")]
struct UserFriendDetailPath {
    user_id: u64,
    friend_name: String,
}

#[test]
fn two_params() {
    let path = UserFriendDetailPath::from_str("/users/612451/friends/steve").unwrap();
    assert_eq!(
        path,
        UserFriendDetailPath {
            user_id: 612451,
            friend_name: "steve".to_string()
        }
    );
}

#[test]
fn two_params_utf8_1() {
    let path = UserFriendDetailPath::from_str("/users/612451/friends/田中").unwrap();
    assert_eq!(
        path,
        UserFriendDetailPath {
            user_id: 612451,
            friend_name: "田中".to_string()
        }
    );
}

#[test]
fn two_params_utf8_2() {
    let path = UserFriendDetailPath::from_str("/users/612451/friends/🌮🌮🌮").unwrap();
    assert_eq!(
        path,
        UserFriendDetailPath {
            user_id: 612451,
            friend_name: "🌮🌮🌮".to_string()
        }
    );
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserListQuery {
    limit: Option<u64>,
    offset: Option<u64>,
    keyword: Option<String>,

    #[serde(default)]
    friends_only: bool,
}

#[derive(AppPath, Debug, PartialEq)]
#[path("/users")]
struct UsersListWithQuery {
    #[query]
    query: UserListQuery,
}

#[test]
fn no_params_simple_query_required() {
    let path = UsersListWithQuery::from_str("/users");
    assert!(path.is_none());
}

#[test]
fn no_params_simple_query() {
    let path = UsersListWithQuery::from_str("/users?friends_only=true").unwrap();
    assert_eq!(
        path,
        UsersListWithQuery {
            query: {
                UserListQuery {
                    limit: None,
                    offset: None,
                    keyword: None,
                    friends_only: true,
                }
            }
        }
    );
}

#[test]
fn no_params_simple_query_missing_bool_field() {
    let path = UsersListWithQuery::from_str("/users?").unwrap();
    assert_eq!(
        path,
        UsersListWithQuery {
            query: {
                UserListQuery {
                    limit: None,
                    offset: None,
                    keyword: None,
                    friends_only: false,
                }
            }
        }
    );
}

#[test]
fn no_params_simple_query_invalid_type() {
    let path = UsersListWithQuery::from_str("/users?offset=test");
    assert!(path.is_none());
}

#[test]
fn no_params_simple_query_all_defined() {
    let path = UsersListWithQuery::from_str(
        "/users?offset=10&limit=20&friends_only=false&keyword=some_keyword",
    )
    .unwrap();
    assert_eq!(
        path,
        UsersListWithQuery {
            query: {
                UserListQuery {
                    limit: Some(20),
                    offset: Some(10),
                    keyword: Some("some_keyword".to_string()),
                    friends_only: false,
                }
            }
        }
    );
}

#[test]
fn no_params_simple_query_url_decoding() {
    let path = UsersListWithQuery::from_str("/users?keyword=some%20keyword%20with%20ampersand-question-equals-stuff%26%3F%3d%3a%3b%40%23%25%5e%5b%5d%7b%7D%60%22%3c%3e%E6%97%A5%E6%9C%AC%E8%AA%9E").unwrap();
    assert_eq!(
        path,
        UsersListWithQuery {
            query: UserListQuery {
                limit: None,
                offset: None,
                keyword: Some("some keyword with ampersand-question-equals-stuff&?=:;@#%^[]{}`\"<>日本語".to_string()),
                friends_only: false,
            }
        }
    );
}

// TODO - uncomment after https://github.com/samscott89/serde_qs/pull/18
// #[test]
// fn no_params_simple_query_url_decoding_plus_sign() {
//     let path = UsersListWithQuery::from_str("/users?keyword=%2b").unwrap();
//     assert_eq!(path, UsersListWithQuery { query: {
//         UserListQuery {
//             limit: None,
//             offset: None,
//             keyword: Some("+".to_string()),
//             friends_only: false,
//         }
//     } });
// }

#[derive(AppPath, Debug, PartialEq)]
#[path("/users/:user_id")]
struct UserDetailExtraPath {
    user_id: u8,

    #[query]
    query: Option<UserListQuery>,
}

#[test]
fn one_param_optional_query_missing() {
    let path = UserDetailExtraPath::from_str("/users/8").unwrap();
    assert_eq!(
        path,
        UserDetailExtraPath {
            user_id: 8,
            query: None
        }
    );
}

#[test]
fn one_param_optional_query_present() {
    let path = UserDetailExtraPath::from_str("/users/8?limit=55").unwrap();
    assert_eq!(
        path,
        UserDetailExtraPath {
            user_id: 8,
            query: Some(UserListQuery {
                limit: Some(55),
                offset: None,
                keyword: None,
                friends_only: false,
            })
        }
    );
}

#[test]
fn one_param_num_out_of_range() {
    let path = UserDetailExtraPath::from_str("/users/256");
    assert!(path.is_none());
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SubmissionsQuery {
    column: Option<String>,
    direction: Option<SortDirection>,
    keyword: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct LimitOffsetQuery {
    limit: Option<u64>,
    offset: Option<u64>,
}

#[derive(AppPath, Debug, PartialEq)]
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
    let path = ExpiredSubmissionsPath::from_str("/p/43/exams/10/submissions_expired").unwrap();
    assert_eq!(path, ExpiredSubmissionsPath {
        project_id: "43".to_string(),
        exam_id: 10,
        query: None,
        limit: None,
    });
}

#[test]
fn test_only_question_mark() {
    let path = ExpiredSubmissionsPath::from_str("/p/43/exams/10/submissions_expired?").unwrap();
    assert_eq!(path, ExpiredSubmissionsPath {
        project_id: "43".to_string(),
        exam_id: 10,
        query: Some(SubmissionsQuery {
            column: None,
            direction: None,
            keyword: None,
        }),
        limit: Some(LimitOffsetQuery {
            limit: None,
            offset: None,
        }),
    });
}
