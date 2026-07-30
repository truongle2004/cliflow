use crate::domain::workflow::Workflow;
use crate::error::Result;
use std::path::Path;

pub fn load_workflow_file(path: impl AsRef<Path>) -> Result<Workflow> {
    let text = std::fs::read_to_string(path)?;
    load_workflow_str(&text)
}

pub fn load_workflow_str(text: &str) -> Result<Workflow> {
    Ok(serde_yaml::from_str(text)?)
}

#[cfg(test)]
mod tests {
    use super::load_workflow_file;

    #[test]
    fn loads_docker_fixture() {
        let workflow = load_workflow_file("tests/fixtures/docker/list-containers.yaml").unwrap();

        assert_eq!(workflow.id, "list-containers");
        assert_eq!(workflow.tool.id, "docker");
        assert_eq!(workflow.steps.len(), 1);
        assert_eq!(workflow.steps[0].command.program, "docker");
    }
}
