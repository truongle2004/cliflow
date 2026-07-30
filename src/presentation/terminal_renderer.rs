use crate::domain::{risk::Risk, workflow::Workflow};
use owo_colors::OwoColorize;

pub fn render_workflow(workflow: &Workflow) {
    println!("{}", workflow.title.bold());
    println!(
        "{} {}/{}",
        "ID:".bold(),
        workflow.tool.id.cyan(),
        workflow.id.cyan()
    );
    println!("{} {}", "Tool:".bold(), workflow.tool.name.cyan());
    println!("{} {}", "Risk:".bold(), risk_label(workflow.risk));

    if !workflow.description.is_empty() {
        println!();
        println!("{}", workflow.description);
    }

    println!();
    println!("{}", "Steps:".bold());
    for (index, step) in workflow.steps.iter().enumerate() {
        println!("{}. {}", index + 1, step.title.bold());
        println!("   {} {}", "Risk:".bold(), risk_label(step.risk));
        println!("   {} {}", "Command:".bold(), render_command(step).green());

        if !step.description.is_empty() {
            println!("   {}", step.description);
        }
    }
}

fn render_command(step: &crate::domain::step::Step) -> String {
    let mut parts = vec![step.command.program.clone()];
    parts.extend(step.command.args.clone());
    parts.join(" ")
}

fn risk_label(risk: Risk) -> String {
    match risk {
        Risk::Low => "low".green().to_string(),
        Risk::Medium => "medium".yellow().to_string(),
        Risk::High => "high".red().bold().to_string(),
    }
}
