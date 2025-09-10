# Steam-Vent

### Interact with the Steam network via rust

Allows communication with the steam servers using the same protocol as the
regular steam client.

## State

Most forms of authenticating to steam are implemented, and you can send requests
for using protobufs that are either packaged by the project or that you bring
yourself.

While the api isn't fully stable yet, it's unlikely to receive major changes at
this point.

- [x] Anonymous sessions
- [x] Password Authentication
- [ ] QR Authentication
- [x] Steam guard (device or email) confirmation
- [x] Device notification confirmation
- [x] Saved machine token confirmation
- [x] Sending and receiving raw messages
- [x] Making RPC calls over the connection
- [x] Communicating with the game coordinator
- [x] Allow using messages from protobufs not included in the project

## Non-goals

This crate intentionally does not include any high level apis, instead it's
encouraged to implement high level apis in separate crates that wrap a
`Connection`.

See [steam-vent-chat](https://codeberg.org/steam-vent/chat) for an example
high-level library.

## Usage

Note that this project is still in early development and apis might see large
changes.

```rust
use std::error::Error;
use steam_vent::connection::Connection;
use steam_vent::proto::steammessages_gameservers_steamclient::CGameServers_GetServerList_Request;
use steam_vent::serverlist::ServerList;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_list = ServerList::discover().await?;
    let mut connection = Connection::anonymous(server_list).await?;

    let mut req = CGameServers_GetServerList_Request::new();
    req.set_limit(16);
    req.set_filter(r"\appid\440".into());
    let some_tf2_servers = connection.service_method(req).await?;
    for server in some_tf2_servers.servers {
        println!(
            "{}({}) playing {}",
            String::from_utf8_lossy(server.name()),
            server.addr(),
            server.map()
        );
    }

    Ok(())
}
```

## Authentication

Steam vent supports both anonymous (server) authentication and user
authentication.

For user authentication the app needs to provide two parts besides the normal
username and password: a store for the steam guard machine data and a
confirmation handler.

### Steam guard machine data store

After authentication, steam provides the client with some machine specific data
to allow skipping (depending on various factors) the authentication step for the
next login.

For steam-vent to make use of this data, the app needs to provide a way for
storing this data by providing an implementation of the [`GuardDataStore`][GDS]
trait.

steam-vent bundles two implementations of the trait:

- [`NullGuardDataStore`][NGDS]: doesn't store any guard data, effectively
  disabling the mechanism.
- [`FileGuardDataStore`][FGDS]: store the machine data as json in the provided
  path.

  Additionally comes with a helper
  ([`FileGuardDataStore::user_cache()`][user_cache]) for using a file in the
  user's cache directory (exact location depends on the platform).

If none of these methods of storage are suitable for the specific use case, the
app can provide their own implementation for the trait. For example storing the
data in the application database.

### Confirmation handler

When logging into steam, a user needs to provide confirmation of the login
trough a second factor. Usually either trough the mobile app or by providing a
TOTP token.

An app can implement this confirmation by providing one or more implementations
of the [ `AuthConfirmationHandler`][ACH] trait.

steam-vent bundles the following implementations of the trait:

- [`DeviceConfirmationHandler`][DCH]: waits for the user to confirm the login
  trough the mobile app
- [`UserProvidedAuthConfirmationHandler`][UPACH]: asks the user for the TOTP
  token by sending details about the requested token to the provided output and
  reading the token from the provided input.
- [`ConsoleAuthConfirmationHandler`][CACH]: is convenience wrapper around the
  `UserProvidedAuthConfirmationHandler` that uses stdin and stdout.
- [`SharedSecretAuthConfirmationHandler`][SSACH]: generates the TOTP
  automatically by providing the shared secret, allowing for zero-interaction
  authentication.

Multiple authentication providers can be combined by using
[`AuthConfirmationHandler::or`][ACH::or], where the first backend that
successfully completes the confirmation will be used. A common use case for apps
will be combining the `DeviceConfirmationHandler` and one of the TOTP providers
to allow users to confirm the login trough either the app or TOTP.

Alternatively apps can provide their own implementation of the trait to
integrate whichever method of asking the user for the TOTP token.

## Protobuf packages

Game-specific probufs are packaged for the following games:

- [tf2](https://codeberg.org/steam-vent/proto-tf2)
- [csgo](https://codeberg.org/steam-vent/proto-csgo)
- [dota2](https://codeberg.org/steam-vent/proto-dota2)

They can be used by either enabling the features in this crate or by depending
on the protobuf package directly.

## Credit

This is in large parts inspired by and based of
[@DoctorMcKay's](https://github.com/DoctorMcKay) work on
[SteamUser](https://github.com/DoctorMcKay/node-steam-user/), massive credits go
to all who worked on that.

[GDS]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/trait.GuardDataStore.html
[NGDS]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/struct.NullGuardDataStore.html
[FGDS]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/struct.FileGuardDataStore.html
[user_cache]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/struct.FileGuardDataStore.html#method.user_cache
[ACH]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/trait.AuthConfirmationHandler.html
[DCH]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/struct.DeviceConfirmationHandler.html
[UPACH]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/type.ConsoleAuthConfirmationHandler.html
[CACH]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/struct.UserProvidedAuthConfirmationHandler.html
[SSACH]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/struct.SharedSecretAuthConfirmationHandler.html
[ACH::or]:
  https://docs.rs/steam-vent/latest/steam_vent/auth/trait.AuthConfirmationHandler.html#method.or
