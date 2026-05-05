use worker::*;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    let hostname = url.host_str().unwrap_or("");
    let path = url.path();

    // Redirect emeditor.com/* → www.emeditor.com/* (302)
    if hostname == "emeditor.com" {
        let mut redirected = url.clone();
        redirected.set_host(Some("www.emeditor.com"))?;
        return Response::redirect_with_status(redirected, 302);
    }

    // Serve /robots.txt
    if path == "/robots.txt" {
        let body = "User-agent: *\nAllow: /\n\nSitemap: https://help.emeditor.com/sitemap.xml";
        let headers = Headers::new();
        headers.set("Content-Type", "text/plain")?;
        return Ok(Response::ok(body)?.with_headers(headers));
    }

    // Pass everything else through to origin
    Fetch::Request(req).send().await
}