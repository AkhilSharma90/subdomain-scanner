use crate::core::scanner;
use crate::json_serialization::message::Message;

pub async fn list_scanners() -> Message {
    let mut message = String::new();
    let subdomain_scanners = scanner::get_scanners();

    message += "Available scanners:\n";
    for scanner in subdomain_scanners {
        let line = format!("* {}. {}\n", scanner.name(), scanner.about());
        message = message + &line;
    }

    Message::new(message)
}

const ABOUT_MESSAGE: &str = "
    Welcome to subdomain scanner!\n
    Authored by: akhil.\n
    Written in: Rust.\n
    What it does: Scans a target domain for its subdomains.\n
    Have fun!\n
";

pub async fn about() -> Message {
    Message::new(ABOUT_MESSAGE.to_string())
}
