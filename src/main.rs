extern crate slackbot;
extern crate regex;

use slackbot::{SlackBot, SlackCommand, SlackResponse};
use regex::Regex;

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

fn regex_cmd(cmd: &mut SlackCommand, resp: &mut SlackResponse) {
    if cmd.args.len() < 2 {
        resp.reply("Two arguments needed: [regex] [test string]");
        return
    }
    
    let regex = match Regex::new(cmd.args[0].to_string()) {
        Ok(r) => re,
        Err(err) => {
            resp.reply(format!("Invalid regex: {err}"));
            return
        }
    };
    
    let test_str = cmd.join_after(0u).as_slice();
    
    if regex.is_match(test_str) {
        cmd.reply("String matches regex.");
    } else {
        cmd.reply("String does not match regex.");
    }
}
