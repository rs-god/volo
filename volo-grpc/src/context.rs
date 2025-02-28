use std::time::Duration;

pub use volo::context::*;
use volo::newtype_impl_context;

pub struct ClientCxInner;

/// A context for client to pass information such as `RpcInfo` and `Config` between middleware
/// during the rpc call lifecycle.
pub struct ClientContext(pub(crate) volo::context::RpcCx<ClientCxInner, Config>);

newtype_impl_context!(ClientContext, Config, 0);

impl ClientContext {
    pub fn new(ri: RpcInfo<Config>) -> Self {
        Self(RpcCx::new(ri, ClientCxInner))
    }
}

impl Default for ClientContext {
    fn default() -> Self {
        Self(RpcCx::new(RpcInfo::with_role(Role::Client), ClientCxInner))
    }
}

impl std::ops::Deref for ClientContext {
    type Target = volo::context::RpcCx<ClientCxInner, Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ClientContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct ServerCxInner;

/// A context for server to pass information such as `RpcInfo` and `Config` between middleware
/// during the rpc call lifecycle.
pub struct ServerContext(pub(crate) volo::context::RpcCx<ServerCxInner, Config>);

newtype_impl_context!(ServerContext, Config, 0);

impl Default for ServerContext {
    fn default() -> Self {
        Self(RpcCx::new(RpcInfo::with_role(Role::Server), ServerCxInner))
    }
}

impl std::ops::Deref for ServerContext {
    type Target = volo::context::RpcCx<ServerCxInner, Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ServerContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Config {
    /// Amount of time to wait connecting.
    pub(crate) connect_timeout: Option<Duration>,
    /// Amount of time to wait reading.
    pub(crate) read_timeout: Option<Duration>,
    /// Amount of time to wait reading response.
    pub(crate) write_timeout: Option<Duration>,
}

impl Config {
    pub fn merge(&mut self, other: Self) {
        if let Some(t) = other.connect_timeout {
            self.connect_timeout = Some(t);
        }
        if let Some(t) = other.read_timeout {
            self.read_timeout = Some(t);
        }
        if let Some(t) = other.write_timeout {
            self.write_timeout = Some(t);
        }
    }
}
