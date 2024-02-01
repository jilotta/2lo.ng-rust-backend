pub struct Host(pub &'static str, pub u16);
impl std::net::ToSocketAddrs for Host {
    type Iter = std::vec::IntoIter<std::net::SocketAddr>;
    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        let Host(host, port) = *self;
        (host, port).to_socket_addrs()
    }
}
impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Host(hostname, port) = &self;
        if *port == 80 {
            write!(f, "http://{}", hostname)
        } else {
            write!(f, "http://{}:{}", hostname, port)
        }
    }
}
