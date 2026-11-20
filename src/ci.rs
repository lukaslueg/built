use std::fmt;

pub(crate) fn detect_ci<T: AsRef<str>>(get: impl Fn(&str) -> Option<T>) -> Option<CIPlatform> {
    // Variable names collected by watson/ci-info.
    for (key, platform) in [
        ("TRAVIS", CIPlatform::Travis),
        ("CIRCLECI", CIPlatform::Circle),
        ("GITLAB_CI", CIPlatform::GitLab),
        ("APPVEYOR", CIPlatform::AppVeyor),
        ("DRONE", CIPlatform::Drone),
        ("MAGNUM", CIPlatform::Magnum),
        ("SEMAPHORE", CIPlatform::Semaphore),
        ("JENKINS_URL", CIPlatform::Jenkins),
        ("bamboo_planKey", CIPlatform::Bamboo),
        ("TF_BUILD", CIPlatform::TFS),
        ("TEAMCITY_VERSION", CIPlatform::TeamCity),
        ("BUILDKITE", CIPlatform::Buildkite),
        ("HUDSON_URL", CIPlatform::Hudson),
        ("GO_PIPELINE_LABEL", CIPlatform::GoCD),
        ("BITBUCKET_COMMIT", CIPlatform::BitBucket),
        ("GITHUB_ACTIONS", CIPlatform::GitHubActions),
    ] {
        if get(key).is_some() {
            return Some(platform);
        }
    }

    if get("TASK_ID").is_some() && get("RUN_ID").is_some() {
        return Some(CIPlatform::TaskCluster);
    }

    if get("CI_NAME").is_some_and(|name| name.as_ref() == "codeship") {
        return Some(CIPlatform::Codeship);
    }

    ["CI", "CONTINUOUS_INTEGRATION", "BUILD_NUMBER"]
        .into_iter()
        .any(|key| get(key).is_some())
        .then_some(CIPlatform::Generic)
}

/// Various Continuous Integration platforms whose presence can be detected.
pub enum CIPlatform {
    /// <https://travis-ci.org>
    Travis,
    /// <https://circleci.com>
    Circle,
    /// <https://about.gitlab.com/gitlab-ci>
    GitLab,
    /// <https://www.appveyor.com>
    AppVeyor,
    /// <https://codeship.com>
    Codeship,
    /// <https://github.com/drone/drone>
    Drone,
    /// <https://magnum-ci.com>
    Magnum,
    /// <https://semaphoreci.com>
    Semaphore,
    /// <https://jenkins.io>
    Jenkins,
    /// <https://www.atlassian.com/software/bamboo>
    Bamboo,
    /// <https://www.visualstudio.com/de/tfs>
    TFS,
    /// <https://www.jetbrains.com/teamcity>
    TeamCity,
    /// <https://buildkite.com>
    Buildkite,
    /// <http://hudson-ci.org>
    Hudson,
    /// <https://github.com/taskcluster>
    TaskCluster,
    /// <https://www.gocd.io>
    GoCD,
    /// <https://bitbucket.org>
    BitBucket,
    /// <https://github.com/features/actions>
    GitHubActions,
    /// Unspecific
    Generic,
}

impl fmt::Display for CIPlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CIPlatform::Travis => "Travis CI",
            CIPlatform::Circle => "CircleCI",
            CIPlatform::GitLab => "GitLab",
            CIPlatform::AppVeyor => "AppVeyor",
            CIPlatform::Codeship => "CodeShip",
            CIPlatform::Drone => "Drone",
            CIPlatform::Magnum => "Magnum",
            CIPlatform::Semaphore => "Semaphore",
            CIPlatform::Jenkins => "Jenkins",
            CIPlatform::Bamboo => "Bamboo",
            CIPlatform::TFS => "Team Foundation Server",
            CIPlatform::TeamCity => "TeamCity",
            CIPlatform::Buildkite => "Buildkite",
            CIPlatform::Hudson => "Hudson",
            CIPlatform::TaskCluster => "TaskCluster",
            CIPlatform::GoCD => "GoCD",
            CIPlatform::BitBucket => "BitBucket",
            CIPlatform::GitHubActions => "GitHub Actions",
            CIPlatform::Generic => "Generic CI",
        })
    }
}
