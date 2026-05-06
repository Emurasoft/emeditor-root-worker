use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    let path = url.path();
    let base_url = env.var("BASE_URL")?.to_string();
    let base_hostname = env.var("BASE_HOSTNAME")?.to_string();

    // Serve /sitemap_index.xml and /robots.txt
    if path == "/sitemap_index.xml" || path == "/robots.txt" {
        let resolved_url = format!("{}{}", base_url, path);
        let assets = env.assets("ASSETS")?;
        return assets.fetch(resolved_url, None).await;
    }

    // Redirect emeditor.com/* → www.emeditor.com/* (302)
    if url.host_str().unwrap_or("") == base_hostname {
        let resolved_url = format!("{}{}", base_url, path);
        return Response::redirect_with_status(resolved_url.parse()?, 302);
    }

    // Pass everything else through to origin
    Fetch::Request(req).send().await
}