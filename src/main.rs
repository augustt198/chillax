extern crate slackbot;
extern crate regex;
extern crate time;

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

    slackbot.manager.register("version".to_string(),    version_cmd);
    slackbot.manager.register("is".to_string(),         is_cmd);
    slackbot.manager.register("regex".to_string(),      regex_cmd);
    slackbot.manager.register("coinflip".to_string(),   coinflip_cmd);
    slackbot.manager.register("lag".to_string(),        lag_cmd);

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
    
    let regex_str = cmd.args[0].as_slice();
    let regex = match Regex::new(regex_str) {
        Ok(re) => re,
        Err(err) => {
            resp.reply(format!("Invalid regex: {}", err).as_slice());
            return
        }
    };
    
    let test_str = cmd.join_after(1u);
    
    if regex.is_match(test_str.as_slice()) {
        resp.reply("String matches regex.");

        let captures = regex.captures(test_str.as_slice());
        if captures.is_some() {
            let caps = captures.unwrap();
            let mut pos = 0u;
            for group in caps.iter() {
                let capture_range = match caps.pos(pos) {
                    Some((start, end)) => format!(" [{} - {}]", start, end),
                    None => "".to_string()
                };
                resp.reply(format!("Capture{}: *{}*", capture_range, group).as_slice());
                pos += 1;
            }
        } else {
            resp.reply("No groups captured.");
        }

    } else {
        resp.reply("String does not match regex.");
    }
}

#[allow(unused_variable)]
fn coinflip_cmd(cmd: &mut SlackCommand, resp: &mut SlackResponse) {
    let options = vec!["heads", "tails"];
    resp.reply(options[std::rand::random::<uint>() % options.len()]);
}

fn lag_cmd(cmd: &mut SlackCommand, resp: &mut SlackResponse) {
    let tm = time::get_time();
    let time_s = tm.sec as f64 + (tm.nsec as f64 / 1000000000.0);
    resp.reply(format!("Your message lag was {}", time_s - cmd.timestamp).as_slice());
}
