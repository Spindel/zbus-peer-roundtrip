use std::error::Error;
use zbus::{dbus_interface, dbus_proxy, fdo, Connection, MessageHeader, SignalContext};

#[dbus_proxy]
trait Callback1 {
    async fn do_something(&self, value: &str) -> zbus::Result<String>;
}

struct Greeter {
    count: u64,
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn say_hello(
        &mut self,
        // Header, needed to get the sender address
        #[zbus(header)] hdr: MessageHeader<'_>,
        // Signal Context, needed to get a reference to the connection.
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        name: &str,
    ) -> fdo::Result<String> {
        self.count += 1;
        let connection = ctxt.connection();

        // Extract the senders unique address.
        if let Some(addr) = hdr.sender()? {
            // We need to convert the address to an owned reference.
            let caller = addr.to_owned();
            // Build a Callback1 Proxy object, directed to  our "caller"
            let proxy = Callback1Proxy::builder(connection)
                .destination(caller)?
                .path("/org/zbus/Callback")?
                .interface("org.zbus.Callback1")?
                .build()
                .await?;

            // Generate a message to the caller
            let msg = format!("You are: {}", &addr);

            // Round-trip and print result.
            let callback_reply = proxy.do_something(&msg).await?;
            dbg!(callback_reply);
        };

        let res = format!("Hello {}! I have been called: {}", name, self.count);
        Ok(res)
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let greeter = Greeter { count: 0 };
    connection
        .object_server_mut()
        .await
        .at("/org/zbus/MyGreeter", greeter)?;
    connection.request_name("org.zbus.MyGreeter").await?;
    loop {
        std::thread::park();
    }
}
