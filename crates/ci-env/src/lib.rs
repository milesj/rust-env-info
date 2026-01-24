mod agola;
mod api;
mod appcenter;
mod appcircle;
mod appveyor;
mod aws_codebuild;
mod azure;
mod bamboo;
mod bitbucket;
mod bitrise;
mod buddy;
mod buildkite;
mod circleci;
mod cirrus;
mod codefresh;
mod codemagic;
mod codeship;
mod drone;
mod eas;
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
mod teamcity;
mod travisci;
mod vela;
mod vercel;
mod woodpecker;
mod xcode_cloud;
mod xcode_server;

pub use api::{CiEnvironment, CiOutput, CiProvider};
use std::env;
use std::sync::OnceLock;

/// Returns true if in a CI environment by checking for the existence of the `CI`
/// environment variable. It does not validate the variable value.
pub fn is_ci() -> bool {
    env::var("CI").is_ok_and(|v| !v.is_empty()) || !matches!(detect_provider(), CiProvider::Unknown)
}

static PROVIDER: OnceLock<CiProvider> = OnceLock::new();

/// Detects the CI provider by checking for the existence of environment variables
/// specific to each provider. Returns `Unknown` if no provider is detected.
pub fn detect_provider() -> CiProvider {
    *PROVIDER.get_or_init(|| {
        for (key, value) in env::vars() {
            if value.is_empty() {
                continue;
            }

            return match key.as_str() {
                "CI" => {
                    if value == "woodpecker" {
                        CiProvider::Woodpecker
                    } else {
                        continue;
                    }
                }
                "CI_NAME" => {
                    if value == "codeship" {
                        CiProvider::Codeship
                    } else {
                        continue;
                    }
                }
                "AC_APPCIRCLE" => CiProvider::Appcircle,
                "AGOLA_REPOSITORY_URL" => CiProvider::Agola,
                "APPCENTER_BUILD_ID" => CiProvider::AppCenter,
                "APPVEYOR" => CiProvider::AppVeyor,
                "AZURE_PIPELINES"
                | "BUILD_BUILDURI"
                | "SYSTEM_TEAMFOUNDATIONCOLLECTIONURI"
                | "TF_BUILD" => CiProvider::Azure,
                "BITBUCKET_WORKSPACE" | "BITBUCKET_COMMIT" => CiProvider::Bitbucket,
                "BITRISE_IO" => CiProvider::Bitrise,
                "BUDDY" | "BUDDY_WORKSPACE_ID" => CiProvider::Buddy,
                "BUILDKITE" => CiProvider::Buildkite,
                "CF_ACCOUNT" | "CF_BUILD_ID" => CiProvider::Codefresh,
                "CIRCLECI" => CiProvider::CircleCI,
                "CIRRUS_CI" => CiProvider::Cirrus,
                "CI_XCODE_PROJECT" | "CI_XCODE_CLOUD" => CiProvider::XcodeCloud,
                "CM_BUILD_ID" => CiProvider::Codemagic,
                "CODEBUILD_BUILD_ARN" => CiProvider::AwsCodebuild,
                "DRONE" => CiProvider::Drone,
                "EAS_BUILD" => CiProvider::Eas,
                "GITHUB_ACTIONS" => CiProvider::GithubActions,
                "GITLAB_CI" => CiProvider::Gitlab,
                "GOOGLE_CLOUD_BUILD" | "BUILDER_OUTPUT" => CiProvider::GoogleCloudBuild,
                "HARNESS_BUILD_ID" => CiProvider::Harness,
                "HEROKU_TEST_RUN_ID" => CiProvider::Heroku,
                "JB_SPACE_EXECUTION_NUMBER" => CiProvider::JetbrainsSpace,
                "JENKINS_URL" | "BUILD_ID" => CiProvider::Jenkins,
                "JENKINS_X_URL" => CiProvider::JenkinsX,
                "NETLIFY" => CiProvider::Netlify,
                "SCREWDRIVER" => CiProvider::Screwdriver,
                "SCRUTINIZER" => CiProvider::Scrutinizer,
                "SEMAPHORE" => CiProvider::Semaphore,
                "TEAMCITY_VERSION" => CiProvider::TeamCity,
                "TRAVIS" => CiProvider::TravisCI,
                "VELA" => CiProvider::Vela,
                "VERCEL" | "NOW_BUILDER" => CiProvider::Vercel,
                "XCS" => CiProvider::XcodeServer,
                "bamboo_planKey" | "BAMBOO_PLANKEY" => CiProvider::Bamboo,
                _ => {
                    continue;
                }
            };
        }

        CiProvider::Unknown
    })
}

/// Returns metadata and information about the current CI environment and CI provider.
pub fn get_environment() -> Option<CiEnvironment> {
    let environment = match detect_provider() {
        CiProvider::Agola => agola::create_environment(),
        CiProvider::AppCenter => appcenter::create_environment(),
        CiProvider::Appcircle => appcircle::create_environment(),
        CiProvider::AppVeyor => appveyor::create_environment(),
        CiProvider::AwsCodebuild => aws_codebuild::create_environment(),
        CiProvider::Azure => azure::create_environment(),
        CiProvider::Bamboo => bamboo::create_environment(),
        CiProvider::Bitbucket => bitbucket::create_environment(),
        CiProvider::Bitrise => bitrise::create_environment(),
        CiProvider::Buddy => buddy::create_environment(),
        CiProvider::Buildkite => buildkite::create_environment(),
        CiProvider::CircleCI => circleci::create_environment(),
        CiProvider::Cirrus => cirrus::create_environment(),
        CiProvider::Codefresh => codefresh::create_environment(),
        CiProvider::Codemagic => codemagic::create_environment(),
        CiProvider::Codeship => codeship::create_environment(),
        CiProvider::Drone => drone::create_environment(),
        CiProvider::Eas => eas::create_environment(),
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
