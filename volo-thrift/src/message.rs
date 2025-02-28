use std::sync::Arc;

pub use pilota::thrift::{Message, Size};
use pilota::{
    thrift::{TAsyncBinaryProtocol, TInputProtocol, TMessageIdentifier, TOutputProtocol},
    AsyncRead,
};

use crate::Error;

#[async_trait::async_trait]
pub trait EntryMessage: Sized + Send {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), Error>;

    fn decode<T: TInputProtocol>(
        protocol: &mut T,
        msg_ident: &TMessageIdentifier,
    ) -> Result<Self, Error>;

    async fn decode_async<R>(
        protocol: &mut TAsyncBinaryProtocol<R>,
        msg_ident: &TMessageIdentifier,
    ) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send;
}

#[async_trait::async_trait]
impl<Message> EntryMessage for Arc<Message>
where
    Message: EntryMessage + Sync,
{
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), Error> {
        (**self).encode(protocol)
    }

    fn decode<T: TInputProtocol>(
        protocol: &mut T,
        msg_ident: &TMessageIdentifier,
    ) -> Result<Self, Error> {
        Message::decode(protocol, msg_ident).map(Arc::new)
    }

    async fn decode_async<R>(
        protocol: &mut TAsyncBinaryProtocol<R>,
        msg_ident: &TMessageIdentifier,
    ) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send,
    {
        Message::decode_async(protocol, msg_ident)
            .await
            .map(Arc::new)
    }
}
