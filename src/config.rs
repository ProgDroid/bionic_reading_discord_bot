#[cfg(feature = "cloud")]
use std::env;

#[cfg(feature = "cloud")]
use secret_vault::{
    gcp,
    ring_encryption::{self, SecretVaultRingAeadEncryption},
};

#[cfg(not(feature = "cloud"))]
use secret_vault::SecretVaultNoEncryption;

use anyhow::{Context, Error};
use secret_vault::{
    InsecureEnvSource, MultipleSecretsSources, SecretNamespace, SecretVault, SecretVaultBuilder,
    SecretVaultRef, SecretVaultView,
};
use secret_vault_value::SecretValue;

const DISCORD_APP_ID_HANDLE: &str = "BIONIC_DISCORD_APP_ID";
const DISCORD_PUBLIC_KEY_HANDLE: &str = "BIONIC_DISCORD_PUBLIC_KEY";
const DISCORD_TOKEN_HANDLE: &str = "BIONIC_DISCORD_TOKEN";
const BIONIC_API_KEY_VAR_HANDLE: &str = "BIONIC_RAPID_API_KEY";
const PORT_HANDLE: &str = "PORT";
const SOURCE_ENV_NAMESPACE: &str = "env";
const DEFAULT_PORT: u16 = 8080;

#[cfg(feature = "cloud")]
const PROJECT_ID_HANDLE: &str = "PROJECT_ID";
#[cfg(feature = "cloud")]
const SOURCE_GCP_NAMESPACE: &str = "gcp";

#[derive(Clone)]
pub struct Config {
    pub app_id: SecretValue,
    pub public_key: SecretValue,
    pub token: SecretValue,
    pub port: u16,
    pub bionic_api_key: SecretValue,
}

impl Config {
    pub async fn load() -> Result<Self, Error> {
        let (vault, vault_refs) = get_vault().await?;

        vault.refresh().await?;

        let app_id = vault
            .get_secret_by_ref(
                vault_refs
                    .get(0)
                    .context("Could not get app_id secret ref")?,
            )
            .await?;

        let public_key = vault
            .get_secret_by_ref(
                vault_refs
                    .get(1)
                    .context("Could not get public_key secret ref")?,
            )
            .await?;

        let token = vault
            .get_secret_by_ref(
                vault_refs
                    .get(2)
                    .context("Could not get token secret ref")?,
            )
            .await?;

        let port_secret = vault
            .get_secret_by_ref(vault_refs.get(3).context("Could not get port secret ref")?)
            .await?;

        let port = if let Some(secret) = port_secret {
            secret.value.as_sensitive_str().parse::<u16>()?
        } else {
            DEFAULT_PORT
        };

        let bionic_api_key = vault
            .get_secret_by_ref(
                vault_refs
                    .get(4)
                    .context("Could not get api_key secret ref")?,
            )
            .await?;

        Ok(Self {
            app_id: app_id.unwrap().value,
            public_key: public_key.unwrap().value,
            token: token.unwrap().value,
            port,
            bionic_api_key: bionic_api_key.unwrap().value,
        })
    }
}

#[cfg(feature = "cloud")]
async fn get_vault() -> Result<
    (
        SecretVault<MultipleSecretsSources, SecretVaultRingAeadEncryption>,
        Vec<SecretVaultRef>,
    ),
    Error,
> {
    let secret_env_namespace: SecretNamespace = SOURCE_ENV_NAMESPACE.into();
    let secret_gcp_namespace: SecretNamespace = SOURCE_GCP_NAMESPACE.into();

    let source = MultipleSecretsSources::new()
        .add_source(&secret_env_namespace, InsecureEnvSource::new())
        .add_source(
            &secret_gcp_namespace,
            gcp::GcpSecretManagerSource::new(&env::var(PROJECT_ID_HANDLE)?).await?,
        );

    let secret_app_key = SecretVaultRef::new(DISCORD_APP_ID_HANDLE.into())
        .with_namespace(secret_gcp_namespace.clone());
    let secret_public_key = SecretVaultRef::new(DISCORD_PUBLIC_KEY_HANDLE.into())
        .with_namespace(secret_gcp_namespace.clone());
    let secret_token = SecretVaultRef::new(DISCORD_TOKEN_HANDLE.into())
        .with_namespace(secret_gcp_namespace.clone());
    let secret_port = SecretVaultRef::new(PORT_HANDLE.into()).with_namespace(secret_env_namespace);
    let secret_bionic_api_key =
        SecretVaultRef::new(BIONIC_API_KEY_VAR_HANDLE.into()).with_namespace(secret_gcp_namespace);

    let vault = SecretVaultBuilder::with_source(source)
        .with_encryption(ring_encryption::SecretVaultRingAeadEncryption::new()?)
        .with_secret_refs(vec![
            &secret_app_key,
            &secret_public_key,
            &secret_token,
            &secret_port,
            &secret_bionic_api_key,
        ])
        .build()
        .map_err(Error::msg)?;

    Ok((
        vault,
        vec![
            secret_app_key,
            secret_public_key,
            secret_token,
            secret_port,
            secret_bionic_api_key,
        ],
    ))
}

#[allow(clippy::unused_async)]
#[cfg(not(feature = "cloud"))]
async fn get_vault() -> Result<
    (
        SecretVault<MultipleSecretsSources, SecretVaultNoEncryption>,
        Vec<SecretVaultRef>,
    ),
    Error,
> {
    let secret_env_namespace: SecretNamespace = SOURCE_ENV_NAMESPACE.into();

    let source =
        MultipleSecretsSources::new().add_source(&secret_env_namespace, InsecureEnvSource::new());

    let secret_app_key = SecretVaultRef::new(DISCORD_APP_ID_HANDLE.into())
        .with_namespace(secret_env_namespace.clone());
    let secret_public_key = SecretVaultRef::new(DISCORD_PUBLIC_KEY_HANDLE.into())
        .with_namespace(secret_env_namespace.clone());
    let secret_token = SecretVaultRef::new(DISCORD_TOKEN_HANDLE.into())
        .with_namespace(secret_env_namespace.clone());
    let secret_port = SecretVaultRef::new(PORT_HANDLE.into())
        .with_namespace(secret_env_namespace.clone())
        .with_required(false);
    let secret_bionic_api_key =
        SecretVaultRef::new(BIONIC_API_KEY_VAR_HANDLE.into()).with_namespace(secret_env_namespace);

    let vault = SecretVaultBuilder::with_source(source)
        .with_secret_refs(vec![
            &secret_app_key,
            &secret_public_key,
            &secret_token,
            &secret_port,
            &secret_bionic_api_key,
        ])
        .build()
        .map_err(Error::msg)?;

    Ok((
        vault,
        vec![
            secret_app_key,
            secret_public_key,
            secret_token,
            secret_port,
            secret_bionic_api_key,
        ],
    ))
}
