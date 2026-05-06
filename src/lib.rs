use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    let hostname = url.host_str().unwrap_or("");
    let path = url.path();

    // Serve /sitemap_index.xml and /robots.txt
    if path == "/sitemap_index.xml" || path == "/robots.txt" {
        let base_url = env.var("BASE_URL")?.to_string();
        let resolved_url = format!("{}{}", base_url, path);
        let assets = env.assets("ASSETS")?;
        return assets.fetch(resolved_url, None).await;
    }

    // Redirect emeditor.com/* → www.emeditor.com/* (302)
    if hostname == "emeditor.com" {
        let mut redirected = url.clone();
        redirected.set_host(Some("www.emeditor.com"))?;
        return Response::redirect_with_status(redirected, 302);
    }

    // Pass everything else through to origin
    Fetch::Request(req).send().await
}