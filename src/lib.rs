use worker::*;

enum WorkerError {
    Client(Error),
    Server(Error),
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerError::Client(e) | WorkerError::Server(e) => write!(f, "{e}"),
        }
    }
}

impl From<Error> for WorkerError {
    fn from(e: Error) -> Self {
        WorkerError::Server(e)
    }
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    match handle(req, env).await {
        Ok(res) => Ok(res),
        Err(WorkerError::Client(e)) => {
            console_warn!("client error: {}", e);
            Response::error(format!("Bad Request: {e}"), 400)
        }
        Err(WorkerError::Server(e)) => {
            console_error!("worker error: {}", e);
            Response::error(format!("Internal Server Error: {e}"), 500)
        }
    }
}

async fn handle(req: Request, env: Env) -> std::result::Result<Response, WorkerError> {
    let url = req.url().map_err(WorkerError::Client)?;
    let path = url.path();
    let base_url = env.var("BASE_URL")?.to_string();
    let base_hostname = env.var("BASE_HOSTNAME")?.to_string();

    // Serve /sitemap_index.xml and /robots.txt
    if path == "/sitemap_index.xml" || path == "/robots.txt" {
        let resolved_url = format!("{base_url}{path}");
        let assets = env.assets("ASSETS")?;
        return assets.fetch(resolved_url, None).await.map_err(Into::into);
    }

    // Redirect emeditor.com/* → www.emeditor.com/* (302)
    if url.host_str().unwrap_or("") == base_hostname {
        let resolved_url = format!("{base_url}{path}");
        let parsed = resolved_url.parse().map_err(|e: url::ParseError| {
            WorkerError::Client(Error::RustError(e.to_string()))
        })?;
        return Response::redirect_with_status(parsed, 302).map_err(Into::into);
    }

    // Pass everything else through to origin
    Fetch::Request(req).send().await.map_err(Into::into)
}