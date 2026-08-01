use lazycmds::recipe::loader::load_embedded_recipes;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn embedded_recipes_are_valid() {
    let recipes = load_embedded_recipes().expect("embedded recipes should load");
    assert!(!recipes.is_empty(), "expected embedded recipes");

    for recipe in recipes {
        assert!(!recipe.id.trim().is_empty(), "recipe id is required");
        assert!(
            !recipe.namespace.trim().is_empty(),
            "recipe namespace is required for {}",
            recipe.id
        );
        assert!(
            !recipe.title.trim().is_empty(),
            "recipe title is required for {}",
            recipe.key()
        );
        assert!(
            !recipe.command.trim().is_empty(),
            "recipe command is required for {}",
            recipe.key()
        );
        assert!(
            !recipe.example.trim().is_empty(),
            "recipe example is required for {}",
            recipe.key()
        );

        let arg_names = recipe
            .args
            .iter()
            .map(|arg| arg.name.as_str())
            .collect::<BTreeSet<_>>();

        for token in command_tokens(&recipe.command) {
            assert!(
                arg_names.contains(token.as_str()),
                "{} uses {{{}}} but has no matching arg",
                recipe.key(),
                token
            );
        }
    }
}

#[test]
fn embedded_recipe_catalog_has_expected_counts() {
    let recipes = load_embedded_recipes().expect("embedded recipes should load");
    let mut counts = BTreeMap::<String, usize>::new();

    for recipe in &recipes {
        *counts.entry(recipe.namespace.clone()).or_default() += 1;
    }

    assert_eq!(recipes.len(), 201);
    assert_eq!(counts.get("aws"), Some(&59));
    assert_eq!(counts.get("docker"), Some(&52));
    assert_eq!(counts.get("git"), Some(&44));
    assert_eq!(counts.get("linux"), Some(&46));
}

#[test]
fn embedded_aws_recipes_include_daily_operations() {
    let keys = load_embedded_recipes()
        .expect("embedded recipes should load")
        .into_iter()
        .map(|recipe| recipe.key())
        .collect::<BTreeSet<_>>();

    for key in [
        "aws/create-s3-bucket",
        "aws/list-objects-in-bucket",
        "aws/copy-file-to-s3",
        "aws/copy-file-from-s3",
        "aws/remove-s3-object",
        "aws/remove-s3-bucket",
        "aws/generate-presigned-url",
        "aws/enable-bucket-versioning",
        "aws/list-iam-users",
        "aws/list-iam-roles",
        "aws/assume-role",
        "aws/list-access-keys",
        "aws/create-access-key",
        "aws/rotate-access-key",
        "aws/list-ec2-instances",
        "aws/start-ec2-instance",
        "aws/stop-ec2-instance",
        "aws/terminate-ec2-instance",
        "aws/describe-instance",
        "aws/ssh-into-instance",
        "aws/get-instance-public-ip",
        "aws/list-security-groups",
        "aws/open-security-group-port",
        "aws/list-vpcs",
        "aws/list-subnets",
        "aws/list-lambda-functions",
        "aws/invoke-lambda-function",
        "aws/update-lambda-code",
        "aws/view-lambda-logs",
        "aws/delete-lambda-function",
        "aws/tail-log-group",
        "aws/list-log-groups",
        "aws/get-log-events",
        "aws/list-cloudwatch-alarms",
        "aws/list-ecs-clusters",
        "aws/list-ecs-services",
        "aws/list-ecs-tasks",
        "aws/update-ecs-service",
        "aws/exec-into-ecs-task",
        "aws/push-image-to-ecr",
        "aws/get-ecr-login",
        "aws/list-stacks",
        "aws/describe-stack",
        "aws/deploy-stack",
        "aws/delete-stack",
        "aws/view-stack-events",
        "aws/list-rds-instances",
        "aws/describe-rds-instance",
        "aws/create-rds-snapshot",
        "aws/restore-rds-from-snapshot",
        "aws/configure-profile",
        "aws/list-profiles",
        "aws/switch-profile",
        "aws/get-current-region",
        "aws/get-cost-and-usage",
        "aws/list-cost-by-service",
    ] {
        assert!(keys.contains(key), "missing embedded recipe: {key}");
    }
}

#[test]
fn embedded_linux_recipes_include_daily_operations() {
    let keys = load_embedded_recipes()
        .expect("embedded recipes should load")
        .into_iter()
        .map(|recipe| recipe.key())
        .collect::<BTreeSet<_>>();

    for key in [
        "linux/find-files-by-name",
        "linux/search-text",
        "linux/follow-log",
        "linux/list-processes",
        "linux/check-disk-usage",
        "linux/create-tar-gz",
        "linux/check-port",
        "linux/create-directory",
        "linux/sync-directory",
        "linux/show-memory-usage",
        "linux/check-service-status",
    ] {
        assert!(keys.contains(key), "missing embedded recipe: {key}");
    }
}

fn command_tokens(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut rest = command;

    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            break;
        };
        tokens.push(rest[..end].to_string());
        rest = &rest[end + 1..];
    }

    tokens
}
