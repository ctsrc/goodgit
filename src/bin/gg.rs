use std::fmt::{
    Debug,
    Display,
    Formatter,
};

use clap::Parser;
use opentelemetry::{
    global,
    global::shutdown_tracer_provider,
    KeyValue,
    trace::{
        TraceContextExt,
        TraceError,
        Tracer,
    },
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    runtime,
    trace::Config as SdkTraceConfig,
};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;
use url::Url;
use uuid::{
    ContextV7,
    Timestamp,
    Uuid,
};

/// Sets up tracing as shown in
/// <https://github.com/open-telemetry/opentelemetry-rust/tree/a1f02faf7c3a4f25f0ccebbc7e0cb8ccf1e80d82/examples/tracing-jaeger>
fn init_tracer_provider() -> Result<opentelemetry_sdk::trace::TracerProvider, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(SdkTraceConfig::default().with_resource(Resource::new(vec![
            KeyValue::new(SERVICE_NAME, "goodgit"),
            KeyValue::new(
                "gg-invocation",
                Uuid::new_v7(Timestamp::now(ContextV7::new())).to_string(),
            ),
        ])))
        .install_batch(runtime::Tokio)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Route: {0}")]
    Route(#[from] RouteError),
}

#[derive(Parser)]
struct Cli {
    /// URL of user or repo
    #[arg(env)]
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tracer_provider = init_tracer_provider().expect("Failed to initialize tracer provider.");
    global::set_tracer_provider(tracer_provider.clone());

    let tracer = global::tracer("goodgit-trace");

    // Command-line arguments
    let cli = Cli::parse();

    let url = cli.url;
    let url: Url = url.parse()?;
    let route = get_route(&url)?;

    tracer.in_span("main", |cx| {
        let span = cx.span();

        span.add_event(
            "Route found for URL",
            // Log the JSON serialized representation of the route
            vec![KeyValue::new(
                "route",
                serde_json::to_string(&route).expect("Route as JSON string"),
            )],
        );

        // Clone the initial repo if any and retrieve info about the repo from API if
        // applicable. Return the username for the user we will retrieve
        // additional data about from API.
        let user = tracer.in_span("Initial repo clone and repo info retrieval", |cx| {
            let span = cx.span();

            match &route {
                Route::GitHubRepo {
                    user,
                    repo_name,
                } => {
                    // TODO: Retrieve info about repo from GitHub API
                    span.add_event(
                        "Retrieved info about repo from GitHub API",
                        vec![KeyValue::new("bar", "1")],
                    );
                    // TODO: Use the username and repo name as it is written in the response from
                    // the API. TODO: Is there a stable unique ID for the repo
                    // in the response? Something we can use instead of "<username>/<repo name>"
                    // TODO: Clone GitHub repo
                    span.add_event("Cloned GitHub repo", vec![KeyValue::new("bar", "1")]);
                    User::GitHub(user.to_owned())
                }
                Route::GitlabRepo {
                    user, ..
                } => {
                    // TODO: Retrieve info about repo from Gitlab API
                    span.add_event(
                        "Retrieved info about repo from Gitlab API",
                        vec![KeyValue::new("bar", "1")],
                    );
                    // TODO: Use the username and repo name as it is written in the response from
                    // the API. TODO: Is there a stable unique ID for the repo
                    // in the response? Something we can use instead of "<username>/<repo name>"
                    // TODO: Clone Gitlab repo
                    span.add_event("Cloned Gitlab repo", vec![KeyValue::new("bar", "1")]);
                    User::Gitlab(user.to_owned())
                }
                Route::User(user) => {
                    span.add_event(
                        "URL is for a user. No initial repo to clone :)",
                        vec![KeyValue::new("user", user.to_string())],
                    );
                    user.to_owned()
                }
            }
        });

        // Retrieve info about user from API
        tracer.in_span("User info retrieval", |cx| {
            let span = cx.span();

            match &user {
                User::GitHub(user) => {
                    // TODO: Retrieve info about user from GitHub API
                    span.add_event(
                        "Retrieved info about user from GitHub API",
                        vec![KeyValue::new("bar", "1")],
                    );
                }
                User::Gitlab(user) => {
                    // TODO: Retrieve info about user from Gitlab API
                    span.add_event(
                        "Retrieved info about user from Gitlab API",
                        vec![KeyValue::new("bar", "1")],
                    );
                }
            }
        });
    });

    shutdown_tracer_provider();
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Username<'a>(&'a str);

impl<'a> Display for Username<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct RepoName<'a>(&'a str);

impl<'a> Display for RepoName<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct GitHubUser<'a> {
    username: Username<'a>,
}

impl<'a> Display for GitHubUser<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(GITHUB_BASE_URL)?;
        f.write_str(self.username.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct GitlabUser<'a> {
    username: Username<'a>,
}

impl<'a> Display for GitlabUser<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(GITLAB_BASE_URL)?;
        f.write_str(self.username.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(bound(deserialize = "'de: 'a"))]
pub enum User<'a> {
    GitHub(GitHubUser<'a>),
    Gitlab(GitlabUser<'a>),
}

impl<'a> Display for User<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            User::GitHub(user) => Display::fmt(user, f),
            User::Gitlab(user) => Display::fmt(user, f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(bound(deserialize = "'de: 'a"))]
pub enum Route<'a> {
    User(User<'a>),
    GitHubRepo {
        user: GitHubUser<'a>,
        repo_name: RepoName<'a>,
    },
    GitlabRepo {
        user: GitlabUser<'a>,
        repo_name: RepoName<'a>,
    },
}

const GITHUB_BASE_URL: &'static str = "https://github.com/";
const GITHUB_DOMAIN: &'static str = "github.com";
const GITLAB_BASE_URL: &'static str = "https://gitlab.com/";
const GITLAB_DOMAIN: &'static str = "gitlab.com";
const SLASH: &'static str = "/";

impl<'a> Display for Route<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Route::User(user) => Display::fmt(user, f),
            Route::GitHubRepo {
                user,
                repo_name,
            } => {
                Display::fmt(user, f)?;
                f.write_str(SLASH)?;
                f.write_str(repo_name.0)
            }
            Route::GitlabRepo {
                user,
                repo_name,
            } => {
                Display::fmt(user, f)?;
                f.write_str(SLASH)?;
                f.write_str(repo_name.0)
            }
        }
    }
}

impl<'a> Into<Url> for Route<'a> {
    fn into(self) -> Url {
        Url::parse(&self.to_string()).expect("Parse URL from String representation of self")
    }
}

#[derive(Error, Debug)]
pub enum RouteError {
    #[error("No domain found in the provided URL")]
    NoDomain,
    #[error("No route for the provided URL")]
    NoRoute,
    #[error("No path in the provided URL")]
    NoPath,
    #[error("No username in the provided URL")]
    NoUsername,
}

fn get_route(url: &Url) -> Result<Route, RouteError> {
    let domain = url.domain().ok_or(RouteError::NoDomain)?;
    let path = url.path();
    let path = path.split('/');

    if domain == GITHUB_DOMAIN {
        let mut iter = path.take(3);
        let _ = iter.next().ok_or(RouteError::NoPath)?;
        let username = iter.next().ok_or(RouteError::NoUsername)?;
        let repo_name = iter.next();
        match repo_name {
            None => {
                return Ok(Route::User(User::GitHub(GitHubUser {
                    username: Username(username),
                })));
            }
            Some(repo_name) => {
                let repo_name = repo_name.strip_suffix(".git").unwrap_or(repo_name);
                return Ok(Route::GitHubRepo {
                    user: GitHubUser {
                        username: Username(username),
                    },
                    repo_name: RepoName(repo_name),
                });
            }
        }
    } else if domain == GITLAB_DOMAIN {
        let mut iter = path.take(3);
        let _ = iter.next().ok_or(RouteError::NoPath)?;
        let username = iter.next().ok_or(RouteError::NoUsername)?;
        let repo_name = iter.next();
        match repo_name {
            None => {
                return Ok(Route::User(User::Gitlab(GitlabUser {
                    username: Username(username),
                })));
            }
            Some(repo_name) => {
                let repo_name = repo_name.strip_suffix(".git").unwrap_or(repo_name);
                return Ok(Route::GitlabRepo {
                    user: GitlabUser {
                        username: Username(username),
                    },
                    repo_name: RepoName(repo_name),
                });
            }
        }
    }

    Err(RouteError::NoRoute)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use url::Url;

    use crate::get_route;

    #[test_case("https://github.com/ctsrc/goodgit", "https://github.com/ctsrc/goodgit"; "Normal GitHub URL")]
    #[test_case("https://github.com/ctsrc/goodgit.git", "https://github.com/ctsrc/goodgit"; "GitHub URL with trailing .git")]
    #[test_case("https://GITHUB.COM/ctsrc/goodgit", "https://github.com/ctsrc/goodgit"; "Uppercase GitHub domain")]
    #[test_case("https://github.com/frondeus/test-case/wiki/Getting-Started", "https://github.com/frondeus/test-case"; "GitHub web UI path components in URL")]
    #[test_case("https://gitlab.com/qemu-project/qemu", "https://gitlab.com/qemu-project/qemu"; "Normal Gitlab URL")]
    #[test_case("https://gitlab.com/qemu-project/qemu.git", "https://gitlab.com/qemu-project/qemu"; "Gitlab URL with trailing .git")]
    #[test_case("https://GITLAB.com/qemu-project/qemu", "https://gitlab.com/qemu-project/qemu"; "Uppercase Gitlab domain")]
    #[test_case("https://gitlab.com/qemu-project/qemu/activity", "https://gitlab.com/qemu-project/qemu"; "Gitlab web UI path components in URL")]
    #[test_case("https://gitlab.com/qemu-project/qemu/-/network/master?ref_type=heads", "https://gitlab.com/qemu-project/qemu"; "Another Gitlab web UI path components in URL")]
    fn test_parse_and_route(
        input_url: &str,
        output_url: &str,
    ) {
        let input_url: Url = input_url.parse().expect("Parse URL");
        let route = get_route(&input_url).expect("Get route");
        let route_url = route.to_string();
        assert_eq!(&route_url, output_url);
    }
}
