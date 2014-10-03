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

    slackbot.manager.register("version".to_string(), version);

    slackbot.start();
}

/*==========================*/
/*         COMMANDS         */
/*==========================*/

#[allow(unused_variable)]
fn version(cmd: &mut SlackCommand, resp: &mut SlackResponse) {
    resp.reply("Current version is 0.0.1");
}
