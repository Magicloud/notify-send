pub mod notify_send {

    use dbus::{BusType, Connection, Error, Message};
    use std::collections::HashMap;
    pub fn notify(
        app_name: &str,
        id2replace: Option<u32>,
        icon_name: &str,
        summary: &str,
        body: &str,
        timeout: i32,
    ) -> Result<u32, Error> {
        let connection = Connection::get_private(BusType::Session)?;
        let method = Message::new_method_call(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "Notify",
        )
        .unwrap();
        let actions: Vec<String> = vec![];
        let hints: HashMap<String, String> = HashMap::new();
        let msg = method
            .append3(app_name, id2replace.unwrap_or(0), icon_name)
            .append3(summary, body, actions)
            .append2(hints, timeout);
        let reply = connection.send_with_reply_and_block(msg, 1000)?;
        Ok(reply.read1()?)
    }

}

pub use notify_send::*;

#[cfg(test)]
mod tests {
    use crate::notify_send::*;
    #[test]
    fn test_notify() {
        notify("app", None, "", "TEST", "test", 10000).expect("Send notify failed");
    }
}
