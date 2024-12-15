pub const REPO_DETAILS_QUERY: &str = r#"
    query RepoDetails($organization: String!, $repository: String!) {
        organization(login: $organization) {
            repository(name: $repository) {
                name
                description
                url
                createdAt
                updatedAt
                pushedAt
                diskUsage
                watchers {
                    totalCount
                }
                stargazers {
                    totalCount
                }
                forks {
                    totalCount
                }
                open_issues: issues(states: [OPEN]) {
                    totalCount
                }
                open_prs: pullRequests(states: [OPEN]) {
                    totalCount
                }
            }
        }
    }
"#;
