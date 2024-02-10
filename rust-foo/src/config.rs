use std::env;

use aws_config::{BehaviorVersion, SdkConfig};

pub async fn build_aws_config(profile: &String) -> SdkConfig {
    env::set_var("AWS_PROFILE", profile);
    aws_config::load_defaults(BehaviorVersion::latest()).await
}
