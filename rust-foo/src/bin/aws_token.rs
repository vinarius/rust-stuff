use aws_sdk_iam;
use aws_sdk_iam::types::MfaDevice;
use aws_sdk_sts::{self, error::ProvideErrorMetadata, types::Credentials};
use chrono;
use chrono::DateTime;
use chrono::Local;
use rust_foo::config::build_aws_config;
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        eprintln!("Usage: aws_token <profile> <token>");
        process::exit(1);
    }

    let profile = &args[1];
    let token = &args[2];
    let profile_token = format!("{profile}-token");

    let stdout_list_profiles = process::Command::new("aws")
        .arg("configure")
        .arg("list-profiles")
        .output()
        .expect("failed to execute aws configure list-profiles")
        .stdout;

    let list_profiles = String::from_utf8_lossy(&stdout_list_profiles);
    let profiles = list_profiles.split("\n").collect::<Vec<&str>>();
    let profile_exists = profiles.contains(&profile.as_str());

    if !profile_exists {
        eprintln!("Profile {profile} not found in credentials");
        process::exit(1);
    }

    let aws_sdk_config = build_aws_config(profile).await;
    let iam_client = aws_sdk_iam::Client::new(&aws_sdk_config);

    let list_mfa_devices_output = iam_client
        .list_mfa_devices()
        .send()
        .await
        .expect("failed to list mfa devices");

    let mfa_device = list_mfa_devices_output.mfa_devices().get(0);

    if mfa_device.is_none() {
        eprintln!("No MFA devices found for profile {profile}");
        process::exit(1);
    }

    let MfaDevice { serial_number, .. } = mfa_device.unwrap();
    let sts_client = aws_sdk_sts::Client::new(&aws_sdk_config);
    let one_day_in_seconds = chrono::Duration::days(1).num_seconds() as i32;

    let get_session_token_output = sts_client
        .get_session_token()
        .duration_seconds(one_day_in_seconds)
        .serial_number(serial_number)
        .token_code(token)
        .send()
        .await;

    if let Err(e) = get_session_token_output {
        eprintln!("{:?}", e.message().unwrap().trim_matches('"'));
        process::exit(1);
    }

    let Credentials {
        access_key_id,
        secret_access_key,
        session_token,
        expiration,
        ..
    } = get_session_token_output.unwrap().credentials.unwrap();

    process::Command::new("aws")
        .arg("configure")
        .arg("set")
        .arg("aws_access_key_id")
        .arg(access_key_id)
        .arg("--profile")
        .arg(&profile_token)
        .output()
        .expect("failed to set aws_access_key_id");

    process::Command::new("aws")
        .arg("configure")
        .arg("set")
        .arg("aws_secret_access_key")
        .arg(secret_access_key)
        .arg("--profile")
        .arg(&profile_token)
        .output()
        .expect("failed to set aws_secret_access_key");

    process::Command::new("aws")
        .arg("configure")
        .arg("set")
        .arg("aws_session_token")
        .arg(session_token)
        .arg("--profile")
        .arg(&profile_token)
        .output()
        .expect("failed to set aws_session_token");

    println!("Set session token in profile {profile_token}, expires at {local_expiration}");
}
