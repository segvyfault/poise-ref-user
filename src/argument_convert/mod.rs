//! Contains the [`ArgumentConvert`] trait and implementations.

mod channel;
mod emoji;
mod guild;
mod member;
mod message;
mod role;
mod user;

pub use user::UserParseError;

use crate::serenity_prelude as serenity;

/// Message context, a helper for User and Member parsing
pub struct MessageContext {
    /// Message that will be used when parsing
    pub message: serenity::Message,
    /// State whether or not the referenced user has already been used
    pub used_referenced_user: bool,

    /// For situation where User consumes a string, that needs to be used in the next [rest] string arg
    pub consumed_string: Option<String>
}

impl MessageContext {
    /// new instance
    pub fn new(message: serenity::Message, uru: bool) -> Self {
        Self {
            message,
            used_referenced_user: uru,
            consumed_string: None
        }
    }
}

use crate::serenity_prelude as serenity;

/// Parse a value from a string in the context of a received message.
///
/// This trait is similar to [`std::str::FromStr`]. The difference is that this trait supports
/// Discord-specific types like [`Member`] or [`Message`].
///
/// Trait implementations may perform network requests during parsing.
///
/// [`Member`]: crate::serenity_prelude::Member
/// [`Message`]: crate::serenity_prelude::Message
#[async_trait::async_trait]
pub trait ArgumentConvert: Sized {
    /// The associated error which can be returned from parsing.
    type Err: std::error::Error + Send + Sync + 'static;

    /// Parses a string `s` as a command parameter of this type.
    async fn convert(
        ctx: impl serenity::CacheHttp,
        guild_id: Option<serenity::GuildId>,
        channel_id: Option<serenity::GenericChannelId>,
        s: &str,
        msg: Option<&mut MessageContext>,
    ) -> Result<Self, Self::Err>;
}

#[async_trait::async_trait]
impl ArgumentConvert for String {
    type Err = std::convert::Infallible;

    async fn convert(
        _: impl serenity::CacheHttp,
        _: Option<serenity::GuildId>,
        _: Option<serenity::GenericChannelId>,
        s: &str,
        _: Option<&mut MessageContext>,
    ) -> Result<Self, Self::Err> {
        Ok(s.to_owned())
    }
}
