use crate::api::{opt_var, var, CiEnvironment, CiOutput, CiProvider};

pub const AZURE_OUTPUT: CiOutput = CiOutput {
    close_log_group: "##[endgroup]",
    open_log_group: "##[group]{name}",
};

// https://learn.microsoft.com/en-us/azure/devops/pipelines/build/variables?view=azure-devops&tabs=yaml
pub fn create_environment() -> CiEnvironment {
    let id = var("BUILD_BUILDID");

    // `BUILD_BUILDURI` is a `vstfs:///` URI, not a web link, so build one
    let url = match (
        opt_var("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI"),
        opt_var("SYSTEM_TEAMPROJECTID"),
    ) {
        (Some(base), Some(project)) if !id.is_empty() => Some(format!(
            "{}/{project}/_build/results?buildId={id}",
            base.trim_end_matches('/')
        )),
        _ => None,
    };

    CiEnvironment {
        base_branch: opt_var("SYSTEM_PULLREQUEST_TARGETBRANCH"),
        base_revision: None,
        branch: opt_var("SYSTEM_PULLREQUEST_SOURCEBRANCH")
            .or_else(|| opt_var("BUILD_SOURCEBRANCHNAME"))
            .unwrap_or_default(),
        env_prefix: Some("BUILD_".into()),
        head_revision: opt_var("SYSTEM_PULLREQUEST_SOURCECOMMITID"),
        id,
        provider: CiProvider::Azure,
        request_id: opt_var("SYSTEM_PULLREQUEST_PULLREQUESTNUMBER")
            .or_else(|| opt_var("SYSTEM_PULLREQUEST_PULLREQUESTID")),
        request_url: None,
        revision: var("BUILD_SOURCEVERSION"),
        url,
    }
}
