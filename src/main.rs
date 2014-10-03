extern crate slackbot;

use slackbot::{SlackBot, SlackCommand, SlackResponse};

fn main() {
    let args = std::os::args();

    if args.len() < 2 {
        println!("No port number given.");
        return
    }

    let port = match from_str(args[1].as_slice()) {
        Some(p) => p,
        None => {
            println!("Invalid port number");
            return
        }
    };

    let mut slackbot = SlackBot::new(port);

    slackbot.username   = Some("Chillax".to_string());
    slackbot.icon_emoji = Some(":tropical_drink:".to_string());

    slackbot.manager.register("version".to_string(), version_cmd);
    slackbot.manager.register("is".to_string(), is_cmd);

    slackbot.start();
}

/*==========================*/
/*         COMMANDS         */
/*==========================*/

#[allow(unused_variable)]
fn version_cmd(cmd: &mut SlackCommand, resp: &mut SlackResponse) {
    resp.reply("Current version is 0.0.1");
}

#[allow(unused_variable)]
fn is_cmd(cmd: &mut SlackCommand, resp: &mut SlackResponse) {
    let options = vec![
        "Obviously it is so",
        "Not a chance",
        "Yes",
        "No",
        "I doubt it",
        "Very likely",
        "Nope",
        "Yep"
    ];

    resp.reply(options[std::rand::random::<uint>() % options.len()]);
}
