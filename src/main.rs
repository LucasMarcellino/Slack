use slack::{Event, RtmClient};
extern crate slack-hook;
use slack_hook::{Slack, PayloadBuilder};



fn main() {
        let slack = Slack::new("https://hooks.slack.com/services/abc/123/45z").unwrap();
        let p =PayloadBuilder::new()
        .text("Pesan pertama dari Bot yang sangat RAMAH !")
        .channel("#bot-connector")
        .username("SBot")
        .icon_emoji(":chart_with_upwards_trend:")
        .build()
        .unwrap();

        let res = slack.send(&p);
        match res {
                ok(()) => println("Ok"),
                Err(x) => println("ERR: {:?}", x)
        }
}