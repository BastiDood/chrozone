fn main() -> anyhow::Result<()> {
    use std::{env::var, net};

    // Retrieve network port
    let port: u16 = var("PORT")?.parse()?;

    // Retrieve the Ed25519 public key
    let public = var("PUB_KEY")?;
    let mut pub_bytes = [0; 32];
    hex::decode_to_slice(public, &mut pub_bytes)?;

    use ring::signature;
    let pub_key = signature::UnparsedPublicKey::new(&signature::ED25519, pub_bytes);

    let listener = net::TcpListener::bind((net::Ipv4Addr::UNSPECIFIED, port))?;

    let runtime = tokio::runtime::Builder::new_multi_thread().enable_io().build()?;
    let tcp = {
        let _guard = runtime.enter();
        tokio::net::TcpListener::from_std(listener)?
    };

    // Listen for new connections
    let arc_pub_key = std::sync::Arc::new(pub_key);
    let mut http = hyper::server::conn::Http::new();
    http.http1_only(true);

    env_logger::init();
    runtime.block_on(async {
        loop {
            let (stream, _) = match tcp.accept().await {
                Ok(pair) => pair,
                _ => continue,
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

            let fut = http.serve_connection(stream, service);
            runtime.spawn(async move { fut.await.unwrap() });
        }
    });

    Ok(())
}
