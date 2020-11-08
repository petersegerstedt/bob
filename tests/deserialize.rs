use bob;
#[test]
fn it_adds_two() {
    assert_eq!(4, bob::add_two(2));
}

#[test]
fn parse_body() {
    let body = include_str!("sample.json");
    assert_eq!(2161, body.len())
}
#[test]
fn parse_tiny() {
    let body = include_str!("sample.json");
    let msg: bob::gitea::Msg = serde_json::from_str(&body).unwrap();
    assert_eq!(40, msg.before.len());

    //let pusher :String = String::from(msg.pusher.as_ref().unwrap().username);
    assert_eq!("gitea", msg.pusher.as_ref().unwrap().username);
    println!("{:?}", &msg);

    match msg.commits {
        Some(ref commits) => {
            commits.iter().for_each(|commit| println!("{:?}", commit));
        },
        _ => {},
    }

    let res = serde_json::to_string_pretty(&msg).unwrap();
    println!("\n\n{}", res);

    assert_eq!("28e1879d029cb852e4844d9c718537df08844e03", msg.before)

}
