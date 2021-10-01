use std::error::Error;
use zbus::{dbus_interface, dbus_proxy, fdo, Connection};

#[dbus_proxy(
    default_service = "org.zbus.MyGreeter",
    interface = "org.zbus.MyGreeter1",
    default_path = "/org/zbus/MyGreeter"
)]
trait Server {
    // Can be `async` as well.
    fn say_hello(&self, name: &str) -> zbus::Result<String>;
}

struct Callback {}
#[dbus_interface(name = "org.zbus.Callback1")]
impl Callback {
    async fn do_something(&mut self, value: &str) -> fdo::Result<String> {
        println!("Doing something with {}", value);
        let res = format!("You get a {} back from me", value);
        Ok(res)
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let callback = Callback {};
    connection
        .object_server_mut()
        .await
        .at("/org/zbus/Callback", callback)?;

    let proxy = ServerProxy::new(&connection).await?;
    let reply = proxy.say_hello("to client").await?;
    println!("Got reply: {}", reply);
    Ok(())
}
