use zbus::{Connection, fdo::ObjectManagerProxy};

#[tokio::main(flavor = "current_thread")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let connection = Connection::system().await?;

    let object_manager_proxy = ObjectManagerProxy::new(&connection, "org.bluez", "/").await?;

    for object in object_manager_proxy.get_managed_objects().await? {
        dbg!(object);
    }

    Ok(())
}
