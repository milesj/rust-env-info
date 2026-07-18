mod api;
mod aws_codedeploy;
mod coolify;
mod deno_deploy;
mod digital_ocean;
mod fly;
mod go_cd;
mod google_appengine;
mod google_cloud_run;
mod heroku;
mod netlify;
mod octopus;
mod railway;
mod release;
mod render;
mod seed;
mod vercel;

pub use api::*;
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;

/// Returns true if in a CD environment by checking for the existence of a deploy provider environment variable.
pub fn is_cd() -> bool {
    !matches!(detect_provider(), CdProvider::Unknown)
}

static PROVIDER: OnceLock<CdProvider> = OnceLock::new();

/// Detects the CD provider by checking for the existence of environment variables specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_provider() -> CdProvider {
    *PROVIDER.get_or_init(|| {
        let vars = env::vars().collect::<HashMap<_, _>>();

        detect_provider_from_vars(&vars)
    })
}

/// Keys are checked in order, so providers that also expose another provider's
/// variables must come before it: `GAE_SERVICE` before `K_SERVICE`, since App
/// Engine second-generation runtimes also set the Cloud Run `K_*` variables.
#[rustfmt::skip]
const PROVIDER_KEYS: &[(&str, CdProvider)] = &[
    ("COOLIFY_RESOURCE_UUID", CdProvider::Coolify),
    ("COOLIFY_FQDN", CdProvider::Coolify),
    ("COOLIFY_URL", CdProvider::Coolify),
    ("DENO_DEPLOYMENT_ID", CdProvider::DenoDeploy),
    ("DEPLOYMENT_GROUP_NAME", CdProvider::AwsCodedeploy),
    ("FLY_APP_NAME", CdProvider::Fly),
    ("GAE_SERVICE", CdProvider::GoogleAppEngine),
    ("GO_PIPELINE_NAME", CdProvider::GoCD),
    ("GO_PIPELINE_LABEL", CdProvider::GoCD),
    ("HEROKU_APP_ID", CdProvider::Heroku),
    ("DYNO", CdProvider::Heroku),
    ("K_SERVICE", CdProvider::GoogleCloudRun),
    ("CLOUD_RUN_JOB", CdProvider::GoogleCloudRun),
    ("NETLIFY", CdProvider::Netlify),
    ("OCTOPUS_RELEASE_ID", CdProvider::Octopus),
    ("RAILWAY_PROJECT_ID", CdProvider::Railway),
    ("RAILWAY_SERVICE_ID", CdProvider::Railway),
    ("RAILWAY_ENVIRONMENT_ID", CdProvider::Railway),
    ("RAILWAY_PUBLIC_DOMAIN", CdProvider::Railway),
    ("RAILWAY_STATIC_URL", CdProvider::Railway), // legacy
    ("RELEASE_BUILD_ID", CdProvider::Release),
    ("RENDER", CdProvider::Render),
    ("SEED_APP_NAME", CdProvider::Seed),
    ("VERCEL", CdProvider::Vercel),
];

fn detect_provider_from_vars(vars: &HashMap<String, String>) -> CdProvider {
    let get = |key: &str| vars.get(key).map(|v| v.as_str()).filter(|v| !v.is_empty());

    for (key, provider) in PROVIDER_KEYS {
        if get(key).is_some() {
            return *provider;
        }
    }

    // App Platform doesn't inject anything automatically; these are bindable
    // variables the user must map in their app spec, so detection only works
    // when both have been bound
    if get("COMMIT_HASH").is_some() && get("PUBLIC_URL").is_some() {
        return CdProvider::DigitalOceanAppPlatform;
    }

    CdProvider::Unknown
}

/// Returns metadata and information about the current deploy environment and CD provider.
pub fn get_environment() -> Option<CdEnvironment> {
    let environment = match detect_provider() {
        CdProvider::AwsCodedeploy => aws_codedeploy::create_environment(),
        CdProvider::Coolify => coolify::create_environment(),
        CdProvider::DenoDeploy => deno_deploy::create_environment(),
        CdProvider::DigitalOceanAppPlatform => digital_ocean::create_environment(),
        CdProvider::Fly => fly::create_environment(),
        CdProvider::GoCD => go_cd::create_environment(),
        CdProvider::GoogleAppEngine => google_appengine::create_environment(),
        CdProvider::GoogleCloudRun => google_cloud_run::create_environment(),
        CdProvider::Heroku => heroku::create_environment(),
        CdProvider::Netlify => netlify::create_environment(),
        CdProvider::Octopus => octopus::create_environment(),
        CdProvider::Railway => railway::create_environment(),
        CdProvider::Release => release::create_environment(),
        CdProvider::Render => render::create_environment(),
        CdProvider::Seed => seed::create_environment(),
        CdProvider::Vercel => vercel::create_environment(),
        CdProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(list: &[(&str, &str)]) -> HashMap<String, String> {
        list.iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn detects_render() {
        let env = vars(&[("RENDER", "true"), ("RENDER_SERVICE_ID", "srv-123")]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::Render);
    }

    #[test]
    fn app_engine_with_cloud_run_vars_is_app_engine() {
        // App Engine second-gen runtimes also set `K_SERVICE`/`K_REVISION`
        let env = vars(&[("K_SERVICE", "default"), ("GAE_SERVICE", "default")]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::GoogleAppEngine);
    }

    #[test]
    fn detects_coolify() {
        let env = vars(&[
            ("COOLIFY_RESOURCE_UUID", "abc123"),
            ("COOLIFY_FQDN", "app.example.com"),
        ]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::Coolify);
    }

    #[test]
    fn detects_deno_deploy() {
        let env = vars(&[
            ("DENO_DEPLOYMENT_ID", "dpl_abc123"),
            ("DENO_REGION", "gcp-us-east4"),
        ]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::DenoDeploy);
    }

    #[test]
    fn detects_cloud_run_jobs() {
        let env = vars(&[("CLOUD_RUN_JOB", "my-job")]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::GoogleCloudRun);
    }

    #[test]
    fn detects_modern_railway() {
        let env = vars(&[
            ("RAILWAY_PUBLIC_DOMAIN", "my-app.up.railway.app"),
            ("RAILWAY_SERVICE_ID", "abc-123"),
        ]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::Railway);
    }

    #[test]
    fn detects_digital_ocean_when_bound() {
        let env = vars(&[("COMMIT_HASH", "abc123"), ("PUBLIC_URL", "https://app")]);

        assert_eq!(
            detect_provider_from_vars(&env),
            CdProvider::DigitalOceanAppPlatform
        );

        // Requires both variables to be bound
        let env = vars(&[("PUBLIC_URL", "https://app")]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::Unknown);
    }

    #[test]
    fn harness_ci_is_not_a_cd_environment() {
        let env = vars(&[("HARNESS_BUILD_ID", "123"), ("DRONE_COMMIT", "abc")]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::Unknown);
    }

    #[test]
    fn unknown_without_provider_vars() {
        let env = vars(&[("PATH", "/usr/bin")]);

        assert_eq!(detect_provider_from_vars(&env), CdProvider::Unknown);
    }
}
