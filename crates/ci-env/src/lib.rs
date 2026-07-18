mod agola;
mod api;
mod appcenter;
mod appcircle;
mod appveyor;
mod aws_amplify;
mod aws_codebuild;
mod azure;
mod bamboo;
mod bitbucket;
mod bitrise;
mod buddy;
mod buildkite;
mod circleci;
mod cirrus;
mod cloudflare_pages;
mod codefresh;
mod codemagic;
mod codeship;
mod drone;
mod eas;
mod forgejo;
mod gitea;
mod github;
mod gitlab;
mod google_cloud_build;
mod harness;
mod heroku;
mod jenkins;
mod jenkins_x;
mod jetbrains_space;
mod netlify;
mod screwdriver;
mod scrutinizer;
mod semaphore;
mod sourcehut;
mod teamcity;
mod travisci;
mod vela;
mod vercel;
mod woodpecker;
mod xcode_cloud;
mod xcode_server;

pub use api::{CiEnvironment, CiOutput, CiProvider};
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;

/// Returns true if in a CI environment by checking the `CI` environment variable,
/// or by detecting a provider. An explicit `CI=false` opts out of detection.
pub fn is_ci() -> bool {
    match env::var("CI") {
        Ok(value) if value == "false" => false,
        Ok(value) if !value.is_empty() => true,
        _ => !matches!(detect_provider(), CiProvider::Unknown),
    }
}

static PROVIDER: OnceLock<CiProvider> = OnceLock::new();

/// Detects the CI provider by checking for the existence of environment variables
/// specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_provider() -> CiProvider {
    *PROVIDER.get_or_init(|| {
        let vars = env::vars().collect::<HashMap<_, _>>();

        detect_provider_from_vars(&vars)
    })
}

/// Keys are checked in order, so providers that also expose another provider's
/// variables must come before it: `GITEA_ACTIONS`/`FORGEJO_ACTIONS` before
/// `GITHUB_ACTIONS` (both forges set the `GITHUB_*` compatibility variables),
/// `HARNESS_BUILD_ID` before `DRONE` (Harness CI sets `DRONE_*` compatibility
/// variables), `JENKINS_X_URL` before `JENKINS_URL`, and the generic `BUILD_ID`
/// (Jenkins without a configured root URL) dead last, since Jenkins X, Netlify,
/// and Google Cloud Build can also set it.
#[rustfmt::skip]
const PROVIDER_KEYS: &[(&str, CiProvider)] = &[
    ("AC_APPCIRCLE", CiProvider::Appcircle),
    ("AGOLA_REPOSITORY_URL", CiProvider::Agola),
    ("APPCENTER_BUILD_ID", CiProvider::AppCenter),
    ("APPVEYOR", CiProvider::AppVeyor),
    ("bamboo_planKey", CiProvider::Bamboo),
    ("BAMBOO_PLANKEY", CiProvider::Bamboo),
    ("BITBUCKET_WORKSPACE", CiProvider::Bitbucket),
    ("BITBUCKET_COMMIT", CiProvider::Bitbucket),
    ("BITRISE_IO", CiProvider::Bitrise),
    ("BUDDY", CiProvider::Buddy),
    ("BUDDY_WORKSPACE_ID", CiProvider::Buddy),
    ("BUILDKITE", CiProvider::Buildkite),
    ("CF_ACCOUNT", CiProvider::Codefresh),
    ("CF_BUILD_ID", CiProvider::Codefresh),
    ("CF_PAGES", CiProvider::CloudflarePages),
    ("CIRCLECI", CiProvider::CircleCI),
    ("CIRRUS_CI", CiProvider::Cirrus),
    ("CI_XCODE_CLOUD", CiProvider::XcodeCloud),
    ("CI_XCODE_PROJECT", CiProvider::XcodeCloud),
    ("CM_BUILD_ID", CiProvider::Codemagic),
    ("CODEBUILD_BUILD_ARN", CiProvider::AwsCodebuild),
    ("EAS_BUILD", CiProvider::Eas),
    ("FORGEJO_ACTIONS", CiProvider::ForgejoActions),
    ("GITEA_ACTIONS", CiProvider::GiteaActions),
    ("GITHUB_ACTIONS", CiProvider::GithubActions),
    ("GITLAB_CI", CiProvider::Gitlab),
    ("HARNESS_BUILD_ID", CiProvider::Harness),
    ("DRONE", CiProvider::Drone),
    ("HEROKU_TEST_RUN_ID", CiProvider::Heroku),
    ("JB_SPACE_EXECUTION_NUMBER", CiProvider::JetbrainsSpace),
    ("JENKINS_X_URL", CiProvider::JenkinsX),
    ("JENKINS_URL", CiProvider::Jenkins),
    ("NETLIFY", CiProvider::Netlify),
    ("SCREWDRIVER", CiProvider::Screwdriver),
    ("SCRUTINIZER", CiProvider::Scrutinizer),
    ("SEMAPHORE", CiProvider::Semaphore),
    ("TEAMCITY_VERSION", CiProvider::TeamCity),
    ("TF_BUILD", CiProvider::Azure),
    ("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI", CiProvider::Azure),
    ("BUILD_BUILDURI", CiProvider::Azure),
    ("TRAVIS", CiProvider::TravisCI),
    ("VELA", CiProvider::Vela),
    ("VERCEL", CiProvider::Vercel),
    ("NOW_BUILDER", CiProvider::Vercel),
    ("XCS", CiProvider::XcodeServer),
    ("GOOGLE_CLOUD_BUILD", CiProvider::GoogleCloudBuild),
    ("BUILDER_OUTPUT", CiProvider::GoogleCloudBuild),
    ("BUILD_ID", CiProvider::Jenkins),
];

fn detect_provider_from_vars(vars: &HashMap<String, String>) -> CiProvider {
    let get = |key: &str| vars.get(key).map(|v| v.as_str()).filter(|v| !v.is_empty());

    if get("CI") == Some("woodpecker") {
        return CiProvider::Woodpecker;
    }

    match get("CI_NAME") {
        Some("codeship") => return CiProvider::Codeship,
        Some("sourcehut") => return CiProvider::Sourcehut,
        _ => {}
    }

    for (key, provider) in PROVIDER_KEYS {
        if get(key).is_some() {
            return *provider;
        }
    }

    // Amplify reserves the `AWS_` prefix but shares it with the AWS CLI
    // and SDKs, so require two of its variables to avoid false positives
    if get("AWS_APP_ID").is_some() && get("AWS_JOB_ID").is_some() {
        return CiProvider::AwsAmplify;
    }

    CiProvider::Unknown
}

/// Returns metadata and information about the current CI environment and CI provider.
pub fn get_environment() -> Option<CiEnvironment> {
    let environment = match detect_provider() {
        CiProvider::Agola => agola::create_environment(),
        CiProvider::AppCenter => appcenter::create_environment(),
        CiProvider::Appcircle => appcircle::create_environment(),
        CiProvider::AppVeyor => appveyor::create_environment(),
        CiProvider::AwsAmplify => aws_amplify::create_environment(),
        CiProvider::AwsCodebuild => aws_codebuild::create_environment(),
        CiProvider::Azure => azure::create_environment(),
        CiProvider::Bamboo => bamboo::create_environment(),
        CiProvider::Bitbucket => bitbucket::create_environment(),
        CiProvider::Bitrise => bitrise::create_environment(),
        CiProvider::Buddy => buddy::create_environment(),
        CiProvider::Buildkite => buildkite::create_environment(),
        CiProvider::CircleCI => circleci::create_environment(),
        CiProvider::Cirrus => cirrus::create_environment(),
        CiProvider::CloudflarePages => cloudflare_pages::create_environment(),
        CiProvider::Codefresh => codefresh::create_environment(),
        CiProvider::Codemagic => codemagic::create_environment(),
        CiProvider::Codeship => codeship::create_environment(),
        CiProvider::Drone => drone::create_environment(),
        CiProvider::Eas => eas::create_environment(),
        CiProvider::ForgejoActions => forgejo::create_environment(),
        CiProvider::GiteaActions => gitea::create_environment(),
        CiProvider::GithubActions => github::create_environment(),
        CiProvider::Gitlab => gitlab::create_environment(),
        CiProvider::GoogleCloudBuild => google_cloud_build::create_environment(),
        CiProvider::Harness => harness::create_environment(),
        CiProvider::Heroku => heroku::create_environment(),
        CiProvider::Jenkins => jenkins::create_environment(),
        CiProvider::JenkinsX => jenkins_x::create_environment(),
        CiProvider::JetbrainsSpace => jetbrains_space::create_environment(),
        CiProvider::Netlify => netlify::create_environment(),
        CiProvider::Screwdriver => screwdriver::create_environment(),
        CiProvider::Scrutinizer => scrutinizer::create_environment(),
        CiProvider::Semaphore => semaphore::create_environment(),
        CiProvider::Sourcehut => sourcehut::create_environment(),
        CiProvider::TeamCity => teamcity::create_environment(),
        CiProvider::TravisCI => travisci::create_environment(),
        CiProvider::Vela => vela::create_environment(),
        CiProvider::Vercel => vercel::create_environment(),
        CiProvider::Woodpecker => woodpecker::create_environment(),
        CiProvider::XcodeCloud => xcode_cloud::create_environment(),
        CiProvider::XcodeServer => xcode_server::create_environment(),
        CiProvider::Unknown => {
            return None;
        }
    };

    Some(environment)
}

/// Returns the output format for the current CI provider.
pub fn get_output() -> Option<CiOutput> {
    match detect_provider() {
        CiProvider::Azure => Some(azure::AZURE_OUTPUT),
        CiProvider::Buildkite => Some(buildkite::BUILDKITE_OUTPUT),
        CiProvider::GithubActions => Some(github::GITHUB_OUTPUT),
        CiProvider::TeamCity => Some(teamcity::TEAMCITY_OUTPUT),
        CiProvider::TravisCI => Some(travisci::TRAVISCI_OUTPUT),
        _ => None,
    }
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
    fn detects_github_actions() {
        let env = vars(&[("CI", "true"), ("GITHUB_ACTIONS", "true")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::GithubActions);
    }

    #[test]
    fn netlify_with_build_id_is_not_jenkins() {
        // Netlify also sets `BUILD_ID`, which must not win over `NETLIFY`,
        // regardless of where it appears in the environment
        let env = vars(&[
            ("BUILD_ID", "5d4aeac2"),
            ("CI", "true"),
            ("NETLIFY", "true"),
        ]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Netlify);
    }

    #[test]
    fn jenkins_x_with_jenkins_vars_is_jenkins_x() {
        let env = vars(&[("BUILD_ID", "123"), ("JENKINS_X_URL", "https://dash")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::JenkinsX);
    }

    #[test]
    fn jenkins_detected_via_url_or_build_id() {
        let env = vars(&[("BUILD_ID", "123"), ("JENKINS_URL", "https://jenkins")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Jenkins);

        let env = vars(&[("BUILD_ID", "123")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Jenkins);
    }

    #[test]
    fn harness_with_drone_compat_vars_is_harness() {
        let env = vars(&[("DRONE", "true"), ("HARNESS_BUILD_ID", "123")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Harness);
    }

    #[test]
    fn woodpecker_detected_via_ci_value() {
        let env = vars(&[("CI", "woodpecker"), ("CI_PIPELINE_NUMBER", "8")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Woodpecker);
    }

    #[test]
    fn codeship_detected_via_ci_name() {
        let env = vars(&[("CI", "true"), ("CI_NAME", "codeship")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Codeship);
    }

    #[test]
    fn gitea_with_github_compat_vars_is_gitea() {
        // Gitea/Forgejo Actions also set `GITHUB_ACTIONS=true` for
        // compatibility, and must win over GitHub Actions
        let env = vars(&[
            ("CI", "true"),
            ("GITEA_ACTIONS", "true"),
            ("GITHUB_ACTIONS", "true"),
        ]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::GiteaActions);

        let env = vars(&[("FORGEJO_ACTIONS", "true"), ("GITHUB_ACTIONS", "true")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::ForgejoActions);
    }

    #[test]
    fn detects_cloudflare_pages() {
        let env = vars(&[("CI", "true"), ("CF_PAGES", "1")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::CloudflarePages);
    }

    #[test]
    fn amplify_requires_both_variables() {
        let env = vars(&[("AWS_APP_ID", "abcd1234"), ("AWS_JOB_ID", "0000000001")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::AwsAmplify);

        // A lone `AWS_*` variable isn't enough to assume Amplify
        let env = vars(&[("AWS_APP_ID", "abcd1234")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Unknown);
    }

    #[test]
    fn sourcehut_detected_via_ci_name() {
        let env = vars(&[("CI_NAME", "sourcehut"), ("JOB_ID", "123")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Sourcehut);
    }

    #[test]
    fn empty_values_are_ignored() {
        let env = vars(&[("CI", "true"), ("NETLIFY", "")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Unknown);
    }

    #[test]
    fn unknown_without_provider_vars() {
        let env = vars(&[("PATH", "/usr/bin"), ("CI", "true")]);

        assert_eq!(detect_provider_from_vars(&env), CiProvider::Unknown);
    }
}
