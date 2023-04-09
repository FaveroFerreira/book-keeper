use std::sync::Arc;

use anyhow::{Context, Result};
use cdrs_tokio::authenticators::{
    NoneAuthenticatorProvider, SaslAuthenticatorProvider, StaticPasswordAuthenticatorProvider,
};
use cdrs_tokio::cluster::session::{Session, SessionBuilder, TcpSessionBuilder};
use cdrs_tokio::cluster::{NodeAddress, NodeTcpConfigBuilder, TcpConnectionManager};
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::retry::DefaultRetryPolicy;
use cdrs_tokio::transport::TransportTcp;
use tokio::sync::RwLock;
use tracing::debug;

use crate::config::environment::Environment;

pub type ScyllaDbSession = Session<
    TransportTcp,
    TcpConnectionManager,
    RoundRobinLoadBalancingStrategy<TransportTcp, TcpConnectionManager>,
>;

pub struct ScyllaDb {
    session: ScyllaDbSession,
}

impl ScyllaDb {
    pub async fn new(env: &Environment) -> Result<Self> {
        let auth = Self::get_authentication_provider(env);

        let node_addresses = env
            .scylla_db_nodes
            .iter()
            .map(NodeAddress::from)
            .collect::<Vec<NodeAddress>>();

        debug!("hitting scylla nodes: {node_addresses:?}");

        let node_config = NodeTcpConfigBuilder::new()
            .with_contact_points(node_addresses)
            .with_authenticator_provider(auth.clone())
            .build()
            .await
            .context("failed to hit scylla nodes")?;

        let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), node_config)
            .with_retry_policy(Box::new(DefaultRetryPolicy::default()))
            .build()
            .context("failed to create scylla db session")?;

        Ok(Self { session })
    }

    pub fn get_authentication_provider(
        env: &Environment,
    ) -> Arc<dyn SaslAuthenticatorProvider + Send + Sync> {
        if let (Some(user), Some(pass)) = (&env.scylla_db_username, &env.scylla_db_password) {
            Arc::new(StaticPasswordAuthenticatorProvider::new(user, pass))
        } else {
            Arc::new(NoneAuthenticatorProvider)
        }
    }
}
