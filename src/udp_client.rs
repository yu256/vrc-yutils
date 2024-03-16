use async_once_cell::OnceCell;
use serde::Serialize;
use serde_json::to_vec;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::{io, net::UdpSocket};

#[cfg(not(feature = "websocket"))]
pub(crate) async fn send_message<T>(serializable: &T, port: Option<u16>) -> io::Result<()>
where
    T: Serialize,
{
    send(&to_vec(&serializable).unwrap(), port).await
}

#[cfg(not(feature = "websocket"))] // OSCを実装したら消す
pub(crate) async fn send(buf: &[u8], port: Option<u16>) -> io::Result<()> {
    const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

    const XSOVERLAY: SocketAddrV4 = SocketAddrV4::new(LOCALHOST, 42069);
    const OSC: SocketAddrV4 = SocketAddrV4::new(LOCALHOST, 9000);

    let to = match port {
        Some(42069) | None => XSOVERLAY,
        Some(9000) => OSC,
        Some(n) => SocketAddrV4::new(LOCALHOST, n),
    };

    static UDPSOCK: OnceCell<UdpSocket> = OnceCell::new();
    let sock = UDPSOCK
        .get_or_try_init(UdpSocket::bind(SocketAddrV4::new(LOCALHOST, 0)))
        .await?;

    sock.send_to(buf, to).await?;

    Ok(())
}
