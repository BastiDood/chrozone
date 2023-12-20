fn main() -> anyhow::Result<()> {
    use std::{env::var, net};

    // Retrieve network port
    let port: u16 = var("PORT")?.parse()?;

    // Retrieve the Ed25519 public key
    let pub_key = var("PUB_KEY")?;
    let mut pub_bytes = [0; 32];
    hex::decode_to_slice(pub_key, &mut pub_bytes)?;
    let pub_key = ed25519_dalek::VerifyingKey::from_bytes(&pub_bytes)?;

    let listener = net::TcpListener::bind((net::Ipv4Addr::UNSPECIFIED, port))?;
    listener.set_nonblocking(true)?;

    let runtime = tokio::runtime::Builder::new_multi_thread().enable_io().build()?;
    let tcp = {
        let _guard = runtime.enter();
        tokio::net::TcpListener::from_std(listener)?
    };

    // Listen for new connections
    let arc_pub_key = std::sync::Arc::new(pub_key);
    let http = hyper::server::conn::http1::Builder::new();

    env_logger::init();
    runtime.block_on(async {
        loop {
            let Ok((stream, _)) = tcp.accept().await else {
                continue;
            };

            let outer = arc_pub_key.clone();
            let service = hyper::service::service_fn(move |req| {
                let inner = outer.clone();
                let (hyper::http::request::Parts { headers, method, uri, .. }, body) = req.into_parts();
                async move {
                    let response = chrozone::try_respond(body, method, uri.path(), &headers, inner.as_ref())
                        .await
                        .unwrap_or_else(chrozone::from_err_status);
                    Ok::<_, core::convert::Infallible>(response)
                }
            });

            let io = hyper_util::rt::TokioIo::new(stream);
            let fut = http.serve_connection(io, service);
            runtime.spawn(async move { fut.await.unwrap() });
        }
    });

    Ok(())
}
