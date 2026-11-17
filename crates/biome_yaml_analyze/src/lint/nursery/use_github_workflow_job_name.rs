use crate::github_workflow::{GitHubWorkflow, is_github_workflow_path};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::use_github_workflow_job_name::UseGithubWorkflowJobNameOptions;
use biome_yaml_syntax::YamlRoot;

declare_lint_rule! {
    /// Require a `name` for the jobs of GitHub Actions workflows.
    ///
    /// GitHub shows the `name` of a job in the workflow run, in pull request checks, and in
    /// notifications. When a job has no `name`, GitHub shows its job ID instead, for example
    /// `build`. A descriptive name makes the job easier to understand, and lets the ID stay short
    /// and stable for use in `needs`.
    ///
    /// The rule only checks files in a `.github/workflows` directory that end in `.yml` or
    /// `.yaml`. A job is also reported when `name` has no value, or when its value is empty or
    /// only whitespace.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,file=.github/workflows/ci.yml,expect_diagnostic
    /// name: CI
    /// on: push
    /// jobs:
    ///   build:
    ///     runs-on: ubuntu-latest
    ///     steps:
    ///       - run: echo "Hello"
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml,file=.github/workflows/ci.yml
    /// name: CI
    /// on: push
    /// jobs:
    ///   build:
    ///     name: Build
    ///     runs-on: ubuntu-latest
    ///     steps:
    ///       - run: echo "Hello"
    /// ```
    ///
    pub UseGithubWorkflowJobName {
        version: "next",
        name: "useGithubWorkflowJobName",
        language: "yaml",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::GitHub],
    }
}

impl Rule for UseGithubWorkflowJobName {
    type Query = Ast<YamlRoot>;
    /// The range of the job ID.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = UseGithubWorkflowJobNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_github_workflow_path(ctx.file_path()) {
            return Box::default();
        }

        let (Some(workflow), errors) = GitHubWorkflow::from_ast(ctx.file_path(), ctx.query())
        else {
            return Box::default();
        };

        if !errors.is_empty() {
            return Box::default();
        }

        workflow
            .jobs
            .iter()
            .flatten()
            .filter(|(_, job)| job.name.as_deref().is_none_or(|name| name.trim().is_empty()))
            .map(|(id, _)| id.range)
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "This job doesn't have a "<Emphasis>"name"</Emphasis>"."
                },
            )
            .note(markup! {
                "Without a name, GitHub shows the job ID in the workflow run and in pull request checks, which makes jobs harder to understand."
            })
            .note(markup! {
                "Add a "<Emphasis>"name"</Emphasis>" key to the job that describes what it does."
            }),
        )
    }
}
