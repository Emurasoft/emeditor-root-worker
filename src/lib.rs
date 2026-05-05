use worker::*;

const ROBOTS_TEXT: &str = r"User-agent: *
Content-Signal: ai-train=yes, search=yes, ai-input=yes
Allow: /

Sitemap: https://help.emeditor.com/sitemap_index.xml";

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
        let headers = Headers::new();
        headers.set("Content-Type", "text/plain")?;
        return Ok(Response::ok(ROBOTS_TEXT)?.with_headers(headers));
    }

    // Pass everything else through to origin
    Fetch::Request(req).send().await
}