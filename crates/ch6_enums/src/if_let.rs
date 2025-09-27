use crate::match_flow::Coin;
use crate::match_flow::UsState;
use log::debug;

pub fn if_let() {
    let config_max = Some(3u8);

    // match shortcut - but not exhaustive!
    if let Some(max) = config_max {
        debug!("max is {}", max);
    }

    if let Some(description) = describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        debug!("State is '{}'", description);
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state == UsState::Alabama {
        Some(String::from("Sweet home Alabama!"))
    } else {
        Some(String::from("Another fine state!"))
    }
}
