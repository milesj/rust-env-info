use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-env-vars.html
pub fn create_environment() -> CiEnvironment {
    let trigger = opt_var("CODEBUILD_WEBHOOK_TRIGGER");

    CiEnvironment {
        base_branch: opt_var("CODEBUILD_WEBHOOK_BASE_REF"),
        base_revision: None,
        branch: opt_var("CODEBUILD_WEBHOOK_HEAD_REF")
            .or_else(|| match &trigger {
                Some(value) => value
                    .strip_prefix("branch/")
                    .map(|branch| branch.to_owned()),
                None => None,
            })
            .unwrap_or_default(),
        env_prefix: Some("CODEBUILD_".into()),
        head_revision: None,
        id: var("CODEBUILD_BUILD_ID"),
        provider: CiProvider::AwsCodebuild,
        request_id: match &trigger {
            Some(value) => value.strip_prefix("pr/").map(|pr| pr.to_owned()),
            None => None,
        },
        request_url: None,
        revision: var("CODEBUILD_RESOLVED_SOURCE_VERSION"),
        // `CODEBUILD_PUBLIC_BUILD_URL` is only set for public build projects
        url: opt_var("CODEBUILD_BUILD_URL").or_else(|| opt_var("CODEBUILD_PUBLIC_BUILD_URL")),
    }
}
