use zbus::{
    Connection,
    fdo::{ManagedObjects, ObjectManagerProxy},
    zvariant::ObjectPath,
};

pub(crate) const PREFIX: &str = "/org/bluez/";
pub(crate) const SERVICE_NAME: &str = "org.bluez";
pub(crate) const INTERFACE: &str = "org.bluez.Adapter1";

pub(crate) fn parse_dbus_path_prefix<'a>(path: &'a ObjectPath) -> Option<(&'a str, &'a str)> {
    match path.strip_prefix(PREFIX) {
        Some(p) => {
            let sep = p.find('/').unwrap_or(p.len());
            Some((&p[0..sep], &p[sep..]))
        }
        None => None,
    }
}

pub(crate) fn parse_dbus_path<'a>(path: &'a ObjectPath) -> Option<&'a str> {
    match parse_dbus_path_prefix(path) {
        Some((v, "")) => Some(v),
        _ => None,
    }
}

/// Enumerate connected Bluetooth adapters and return their names.
pub async fn adapter_names(all_dbus_objects: &ManagedObjects) -> zbus::Result<Vec<String>> {
    let mut names = Vec::new();
    for (path, interfaces) in all_dbus_objects {
        match parse_dbus_path(path) {
            Some(name) if interfaces.contains_key(INTERFACE) => {
                names.push(name.to_string());
            }
            _ => (),
        }
    }
    Ok(names)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let connection = Connection::system().await?;

    let object_manager_proxy = ObjectManagerProxy::new(&connection, SERVICE_NAME, "/").await?;

    let names = adapter_names(&object_manager_proxy.get_managed_objects().await?).await?;

    dbg!(names);

    Ok(())
}
