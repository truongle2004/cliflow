
# Scalable CLI Cheatsheet Tool Architecture

## Project Vision

Build a terminal-based tool that helps developers remember commands and workflows for many command-line tools.

Initial support:

* [ ] Git
* [ ] GitHub CLI

Future support:

* [ ] Docker CLI
* [ ] Docker Compose
* [ ] AWS CLI
* [ ] Kubernetes CLI
* [ ] Terraform CLI
* [ ] Azure CLI
* [ ] Google Cloud CLI
* [ ] Custom team commands

The application should treat each CLI tool as an independent module or content pack.

## Proposed Tool Name

Use a generic name instead of a Git-specific name.

Possible names:

* `cliflow`
* `cmdflow`
* `shellguide`
* `termflow`
* `clirecall`

Example commands in this plan use:

```bash
cliflow
```

## Example Usage

```bash
cliflow list
cliflow tools
cliflow search "undo commit"
cliflow show git/undo-last-commit
cliflow show github/create-pr
cliflow show docker/clean-unused-images
cliflow show aws/list-s3-buckets
```

Tool-specific browsing:

```bash
cliflow git list
cliflow github list
cliflow docker list
cliflow aws list
```

Interactive mode:

```bash
cliflow
```

Example interface:

```text
Choose a CLI tool:

> Git
  GitHub CLI
  Docker
  AWS CLI
  Kubernetes
```

After selecting Docker:

```text
What are you trying to do?

> Build an image
  Run a container
  Inspect a container
  View logs
  Clean unused resources
```

## Core Design Principle

Separate the application engine from cheatsheet content.

```text
CLI application
├── Command parser
├── Content loader
├── Search engine
├── Renderer
├── Validator
├── Context detection
└── Plugin or pack system

Content packs
├── Git
├── GitHub
├── Docker
├── AWS
└── Kubernetes
```

The core application should not contain Git-specific or Docker-specific business logic unless context detection requires it.

## Domain Model

### Tool

Represents one command-line application.

Examples:

* Git
* GitHub CLI
* Docker
* AWS CLI

Required fields:

* [ ] Tool ID
* [ ] Display name
* [ ] Executable name
* [ ] Description
* [ ] Documentation URL
* [ ] Version detection command
* [ ] Categories
* [ ] Workflows

Example:

```yaml
id: docker
name: Docker
executable: docker
description: Build, run, and manage containers.
version_command:
  - docker
  - --version
```

### Category

Groups related workflows inside a tool.

Docker examples:

* Images
* Containers
* Networks
* Volumes
* Compose
* Cleanup
* Troubleshooting

AWS examples:

* Authentication
* EC2
* S3
* IAM
* Lambda
* CloudWatch
* ECS

### Workflow

Represents a real task rather than a single command.

Examples:

* Create a GitHub pull request
* Clean unused Docker images
* Upload a file to Amazon S3
* Restart a Kubernetes deployment

Required workflow fields:

* [ ] Unique ID
* [ ] Tool ID
* [ ] Category
* [ ] Title
* [ ] Description
* [ ] Tags
* [ ] Aliases
* [ ] Risk level
* [ ] Prerequisites
* [ ] Steps
* [ ] Alternatives
* [ ] Related workflows

### Step

Each workflow contains one or more steps.

A step may contain:

* [ ] Step title
* [ ] Explanation
* [ ] One or more commands
* [ ] Notes
* [ ] Expected result
* [ ] Risk warning

### Command

Commands should be represented as structured data rather than plain strings.

```yaml
commands:
  - run:
      - git
      - status
    display: git status
    explanation: Show the current repository state.
    risk: safe
```

Using an argument list makes command execution safer in future versions because the application does not need to parse a shell command string.

## Suggested Content Structure

```text
content/
├── git/
│   ├── tool.yaml
│   ├── categories/
│   │   ├── branches.yaml
│   │   ├── commits.yaml
│   │   └── recovery.yaml
│   └── workflows/
│       ├── start-feature.yaml
│       ├── undo-last-commit.yaml
│       └── recover-lost-commit.yaml
├── github/
│   ├── tool.yaml
│   └── workflows/
│       ├── create-pr.yaml
│       ├── review-pr.yaml
│       └── check-ci.yaml
├── docker/
│   ├── tool.yaml
│   └── workflows/
│       ├── build-image.yaml
│       ├── inspect-container.yaml
│       └── clean-resources.yaml
└── aws/
    ├── tool.yaml
    └── workflows/
        ├── configure-profile.yaml
        ├── list-s3-buckets.yaml
        └── upload-to-s3.yaml
```

## Example Docker Workflow

```yaml
schema_version: 1

id: clean-unused-resources
tool: docker
category: cleanup

title: Clean unused Docker resources
description: Remove Docker resources that are no longer being used.

tags:
  - clean
  - prune
  - disk
  - storage

aliases:
  - free docker space
  - remove unused docker data

risk: caution

prerequisites:
  - Docker is installed
  - Docker daemon is running

steps:
  - title: Inspect Docker disk usage
    commands:
      - run:
          - docker
          - system
          - df
        display: docker system df
        explanation: Show disk usage before deleting anything.
        risk: safe

  - title: Remove unused resources
    warning: This may remove stopped containers and unused networks.
    commands:
      - run:
          - docker
          - system
          - prune
        display: docker system prune
        explanation: Remove unused Docker resources.
        risk: caution

related:
  - docker/remove-unused-images
  - docker/remove-unused-volumes
```

## Example AWS Workflow

```yaml
schema_version: 1

id: list-s3-buckets
tool: aws
category: s3

title: List S3 buckets
description: Display S3 buckets available to the active AWS profile.

tags:
  - aws
  - s3
  - bucket
  - storage

risk: safe

prerequisites:
  - AWS CLI is installed
  - An AWS profile is configured

steps:
  - title: Check the active AWS identity
    commands:
      - run:
          - aws
          - sts
          - get-caller-identity
        display: aws sts get-caller-identity
        explanation: Confirm which AWS account and identity are active.
        risk: safe

  - title: List S3 buckets
    commands:
      - run:
          - aws
          - s3
          - ls
        display: aws s3 ls
        explanation: List buckets accessible to the current identity.
        risk: safe
```

## Recommended Rust Architecture

```text
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   ├── arguments.rs
│   └── commands/
│       ├── list.rs
│       ├── show.rs
│       ├── search.rs
│       ├── doctor.rs
│       └── interactive.rs
├── domain/
│   ├── mod.rs
│   ├── tool.rs
│   ├── category.rs
│   ├── workflow.rs
│   ├── step.rs
│   ├── command.rs
│   └── risk.rs
├── application/
│   ├── mod.rs
│   ├── list_workflows.rs
│   ├── show_workflow.rs
│   ├── search_workflows.rs
│   └── detect_context.rs
├── infrastructure/
│   ├── mod.rs
│   ├── content_loader.rs
│   ├── yaml_loader.rs
│   ├── embedded_loader.rs
│   ├── filesystem_loader.rs
│   ├── process_runner.rs
│   └── version_detector.rs
├── presentation/
│   ├── mod.rs
│   ├── terminal_renderer.rs
│   ├── markdown_renderer.rs
│   ├── json_renderer.rs
│   └── interactive_ui.rs
├── search/
│   ├── mod.rs
│   ├── index.rs
│   └── fuzzy.rs
├── validation/
│   ├── mod.rs
│   └── schema.rs
└── error.rs
```

## Workspace Structure

As the project grows, convert it into a Cargo workspace.

```text
cliflow/
├── Cargo.toml
├── crates/
│   ├── cliflow-cli/
│   ├── cliflow-core/
│   ├── cliflow-content/
│   ├── cliflow-search/
│   └── cliflow-renderer/
├── packs/
│   ├── git/
│   ├── github/
│   ├── docker/
│   └── aws/
├── schemas/
│   └── workflow.schema.json
└── tests/
```

Responsibilities:

```text
cliflow-cli
└── Parses terminal commands and coordinates use cases

cliflow-core
└── Contains Tool, Workflow, Step, Command, and Risk models

cliflow-content
└── Loads and validates built-in and custom content packs

cliflow-search
└── Handles exact, tag-based, and fuzzy searching

cliflow-renderer
└── Produces terminal, Markdown, and JSON output
```

Do not start with multiple crates immediately. Begin with one crate and split it into a workspace when boundaries become stable.

## Content Pack System

Each CLI tool should be installable as a content pack.

Built-in packs:

```text
git
github
docker
aws
```

Community packs:

```text
kubectl
terraform
azure
gcloud
npm
cargo
dotnet
```

A pack should contain:

```text
my-pack/
├── pack.yaml
├── workflows/
├── categories/
└── README.md
```

Example manifest:

```yaml
schema_version: 1

id: kubernetes
name: Kubernetes CLI
version: 1.0.0
executable: kubectl
description: Workflows for managing Kubernetes resources.

authors:
  - name: CLIFlow Community

workflow_directory: workflows
```

## Custom User Packs

Support user-defined workflows in a configuration directory.

Linux:

```text
~/.config/cliflow/packs/
```

macOS:

```text
~/Library/Application Support/cliflow/packs/
```

Windows:

```text
%APPDATA%\cliflow\packs\
```

Possible commands:

```bash
cliflow pack list
cliflow pack validate ./my-pack
cliflow pack add ./my-pack
cliflow pack remove custom-team
```

Remote pack installation can be considered later:

```bash
cliflow pack install owner/repository
```

Do not implement remote installation until package verification and security rules are defined.

## Search Design

Search across:

* [ ] Tool names
* [ ] Workflow titles
* [ ] Descriptions
* [ ] Categories
* [ ] Tags
* [ ] Aliases
* [ ] Command text

Examples:

```bash
cliflow search "undo commit"
cliflow search "free docker disk"
cliflow search "upload s3"
cliflow search "restart deployment"
```

Filter by tool:

```bash
cliflow search "logs" --tool docker
cliflow search "logs" --tool aws
cliflow search "logs" --tool kubernetes
```

## Output Formats

The application should support multiple renderers.

Terminal:

```bash
cliflow show docker/view-logs
```

Compact:

```bash
cliflow show docker/view-logs --compact
```

Markdown:

```bash
cliflow show docker/view-logs --format markdown
```

JSON:

```bash
cliflow show docker/view-logs --format json
```

This allows the same content to be used by:

* Terminal users
* Documentation websites
* Editor extensions
* Shell integrations
* Other developer tools

## Context Detection

Each tool may provide a context detector.

Git detector:

* Current repository
* Current branch
* Changed files
* Upstream branch
* Active pull request

Docker detector:

* Docker installation
* Docker daemon status
* Running containers
* Compose configuration

AWS detector:

* AWS CLI installation
* Active profile
* Current region
* Current account identity

Context detectors should implement a common interface.

```rust
pub trait ContextProvider {
    fn tool_id(&self) -> &'static str;

    fn detect(&self) -> Result<ToolContext>;
}
```

Context detection must be:

* [ ] Read-only by default
* [ ] Optional
* [ ] Clearly displayed
* [ ] Subject to timeouts
* [ ] Isolated from rendering and content loading

## Command Execution

The first versions should only display commands.

Later, optional execution may be added.

Execution rules:

* [ ] Never execute commands automatically
* [ ] Require explicit user action
* [ ] Show the exact command first
* [ ] Represent commands as executable plus arguments
* [ ] Avoid passing commands through a shell
* [ ] Require confirmation for modifying commands
* [ ] Require stronger confirmation for dangerous commands
* [ ] Allow execution to be disabled globally

Example Rust structure:

```rust
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
    pub display: String,
    pub explanation: String,
    pub risk: RiskLevel,
}
```

Safer execution:

```rust
std::process::Command::new(&command.program)
    .args(&command.args)
    .status()?;
```

Avoid:

```rust
std::process::Command::new("sh")
    .arg("-c")
    .arg(user_controlled_command)
    .status()?;
```

## Risk Classification

Use one risk model for all CLI tools.

```rust
pub enum RiskLevel {
    Safe,
    Modifies,
    Caution,
    Dangerous,
}
```

Examples:

```text
SAFE
git status
docker ps
aws sts get-caller-identity

MODIFIES
git commit
docker run
aws s3 cp

CAUTION
git reset --hard
docker system prune
kubectl delete pod

DANGEROUS
aws s3 rm --recursive
kubectl delete namespace
terraform destroy
```

Risk may depend on arguments, not only the executable.

For example:

```text
docker volume ls       → safe
docker volume create   → modifies
docker volume prune    → caution
```

## Versioning

Every content pack should include:

* [ ] Pack version
* [ ] Schema version
* [ ] Supported application version
* [ ] Optional supported CLI versions

Example:

```yaml
schema_version: 1
pack_version: 1.2.0

requires:
  cliflow: ">=0.3.0"
  docker: ">=24"
```

The loader should return a clear error for unsupported schema versions.

## MVP Scope

### Version 0.1

* [ ] Implement the general domain model
* [ ] Implement `list`, `show`, and `search`
* [ ] Load YAML workflows
* [ ] Support multiple tool IDs
* [ ] Add terminal and compact renderers
* [ ] Add risk levels
* [ ] Add Git workflows
* [ ] Add GitHub CLI workflows
* [ ] Add schema validation
* [ ] Do not execute commands

### Version 0.2

* [ ] Add Docker workflows
* [ ] Add interactive browsing
* [ ] Add fuzzy search
* [ ] Add `doctor`
* [ ] Add custom local packs
* [ ] Add Markdown and JSON output
* [ ] Add shell completion

### Version 0.3

* [ ] Add AWS workflows
* [ ] Add read-only context detection
* [ ] Add tool version detection
* [ ] Add workflow suggestions
* [ ] Add pack validation commands
* [ ] Add documentation generation

### Later Versions

* [ ] Add Kubernetes and Terraform packs
* [ ] Add community pack support
* [ ] Add signed pack distribution
* [ ] Add editor integrations
* [ ] Add an optional terminal UI
* [ ] Add carefully controlled command execution

## Important Architectural Rules

* [ ] Do not put Git-specific fields in the generic workflow model
* [ ] Do not hardcode categories in Rust
* [ ] Keep workflow content outside application logic
* [ ] Give every workflow a globally unique ID
* [ ] Version the content schema from the first release
* [ ] Keep search independent from storage
* [ ] Keep rendering independent from search
* [ ] Keep context detection independent from workflows
* [ ] Represent executable commands as program and argument lists
* [ ] Allow built-in and user-created content to use the same model
* [ ] Add new CLI tools without modifying the core application

## Definition of Scalable

The architecture is scalable when adding Docker support only requires:

* [ ] Creating a Docker content pack
* [ ] Adding Docker workflows
* [ ] Optionally adding a Docker context provider
* [ ] Adding Docker-specific tests

It should not require rewriting:

* CLI parsing
* Search
* Rendering
* Validation
* Risk handling
* Content loading

The main design goal is:

> New CLI tools should be added primarily as data, not as new application code.
