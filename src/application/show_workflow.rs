use crate::domain::workflow::Workflow;
use crate::error::{Error, Result};

pub fn show_workflow(workflows: &[Workflow], id: &str) -> Result<Workflow> {
    workflows
        .iter()
        .find(|workflow| workflow_id(workflow) == id)
        .cloned()
        .ok_or_else(|| Error::Message(format!("workflow not found: {id}")))
}

fn workflow_id(workflow: &Workflow) -> String {
    format!("{}/{}", workflow.tool.id, workflow.id)
}

#[cfg(test)]
mod tests {
    use super::show_workflow;
    use crate::infrastructure::embedded_loader::load_embedded_workflows;

    #[test]
    fn finds_workflow_by_tool_scoped_id() {
        let workflows = load_embedded_workflows().unwrap();
        let workflow = show_workflow(&workflows, "git/undo-last-commit").unwrap();

        assert_eq!(workflow.id, "undo-last-commit");
        assert_eq!(workflow.tool.id, "git");
    }
}
