
pub mod gitea {
    use serde::{Serialize, Deserialize};
    //use serde::de::{IgnoredAny};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Msg<'a> {
        //pub secret: Option<&'a str>,
        pub before: &'a str, //"28e1879d02 9cb852e484 4d9c718537 df08844e03",
        pub after: &'a str, //"bffeb74224043ba2feb48d137756c8a9331c449a",
        //compare_url: Option<&'a str>,
        pub commits: Option<Vec<Commit<'a>>> ,
        //repository: Option<IgnoredAny> ,
        pub pusher: Option<User<'a>> ,
        //sender: Option<IgnoredAny> ,
    }
    /*

    "pusher": {
      "id": 1,
      "login": "gitea",
      "full_name": "Gitea",
      "email": "someone@gitea.io",
      "avatar_url": "https://localhost:3000/avatars/1",
      "username": "gitea"
    },
    */

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub login: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub full_name: Option<&'a str>,
        pub email: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub avatar_url: Option<&'a str>,
        pub username: &'a str,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Commit<'a> {
        pub id: &'a str,
        pub message: &'a str,
        //pub url: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub author: Option<User<'a>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub committer: Option<User<'a>>,
        pub timestamp: &'a str, 
    }
}

pub fn add_two(i: i32) -> i32 {
    i + 2
}
