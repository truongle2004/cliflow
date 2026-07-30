use crate::domain::workflow::Workflow;
use crate::error::{Error, Result};
use crate::infrastructure::yaml_loader::load_workflow_str;
use include_dir::{Dir, DirEntry, include_dir};

static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content");

pub fn load_embedded_workflows() -> Result<Vec<Workflow>> {
    let mut workflows = Vec::new();
    collect_workflows(&CONTENT_DIR, &mut workflows)?;
    Ok(workflows)
}

fn collect_workflows(dir: &Dir<'_>, workflows: &mut Vec<Workflow>) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(child) => collect_workflows(child, workflows)?,
            DirEntry::File(file) if is_workflow_yaml(file.path()) => {
                let text = file.contents_utf8().ok_or_else(|| {
                    Error::Message(format!(
                        "embedded workflow is not UTF-8: {}",
                        file.path().display()
                    ))
                })?;
                workflows.push(load_workflow_str(text)?);
            }
            DirEntry::File(_) => {}
        }
    }

    Ok(())
}

fn is_workflow_yaml(path: &std::path::Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("yaml")
        && path.parent().and_then(|parent| parent.file_name()) == Some("workflows".as_ref())
}

#[cfg(test)]
mod tests {
    use super::load_embedded_workflows;

    #[test]
    fn loads_embedded_content_workflows() {
        let workflows = load_embedded_workflows().unwrap();

        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].id, "undo-last-commit");
    }
}
