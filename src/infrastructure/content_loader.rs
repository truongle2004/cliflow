use crate::domain::workflow::Workflow;
use crate::error::Result;
use crate::infrastructure::yaml_loader::load_workflow_file;
use std::path::Path;
use walkdir::WalkDir;

pub fn load_content_workflows(root: impl AsRef<Path>) -> Result<Vec<Workflow>> {
    let mut workflows = Vec::new();

    for entry in WalkDir::new(root).into_iter() {
        let entry = entry?;
        let path = entry.path();

        if is_workflow_yaml(path) {
            workflows.push(load_workflow_file(path)?);
        }
    }

    Ok(workflows)
}

fn is_workflow_yaml(path: &Path) -> bool {
    path.is_file()
        && path.extension().and_then(|ext| ext.to_str()) == Some("yaml")
        && path.parent().and_then(|parent| parent.file_name()) == Some("workflows".as_ref())
}

#[cfg(test)]
mod tests {
    use super::load_content_workflows;

    #[test]
    fn loads_workflows_from_content_tree() {
        let workflows = load_content_workflows("content").unwrap();

        assert!(workflows.len() >= 18);
        assert!(
            workflows
                .iter()
                .all(|workflow| matches!(workflow.tool.id.as_str(), "git" | "linux"))
        );
        assert!(
            workflows.iter().all(|workflow| workflow.steps.len() > 1),
            "workflow content should describe multi-step flows, not single CLI recipes"
        );
        assert!(
            workflows
                .iter()
                .any(|workflow| workflow.id == "daily-coding-branch")
        );
        assert!(
            workflows
                .iter()
                .any(|workflow| workflow.id == "main-changes-to-feature-branch")
        );
        assert!(
            workflows
                .iter()
                .any(|workflow| workflow.id == "fix-commit-on-main")
        );
        assert!(
            workflows
                .iter()
                .any(|workflow| workflow.id == "process-and-port-troubleshooting")
        );
    }
}
