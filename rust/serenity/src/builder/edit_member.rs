#[cfg(feature = "http")]
use super::Builder;
#[cfg(feature = "http")]
use crate::http::CacheHttp;
#[cfg(feature = "http")]
use crate::internal::prelude::*;
use crate::model::prelude::*;

/// A builder which edits the properties of a [`Member`], to be used in conjunction with
/// [`Member::edit`].
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild#modify-guild-member)
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct EditMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<RoleId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Box<Option<ChannelId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    communication_disabled_until: Option<Box<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<GuildMemberFlags>,

    #[serde(skip)]
    audit_log_reason: Option<&'a str>,
}

impl<'a> EditMember<'a> {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether to deafen the member.
    ///
    /// **Note**: Requires the [Deafen Members] permission.
    ///
    /// [Deafen Members]: Permissions::DEAFEN_MEMBERS
    pub fn deafen(mut self, deafen: bool) -> Self {
        self.deaf = Some(deafen);
        self
    }

    /// Whether to mute the member.
    ///
    /// **Note**: Requires the [Mute Members] permission.
    ///
    /// [Mute Members]: Permissions::MUTE_MEMBERS
    pub fn mute(mut self, mute: bool) -> Self {
        self.mute = Some(mute);
        self
    }

    /// Changes the member's nickname. Pass an empty string to reset the nickname.
    ///
    /// **Note**: Requires the [Manage Nicknames] permission.
    ///
    /// [Manage Nicknames]: Permissions::MANAGE_NICKNAMES
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nick = Some(nickname.into());
        self
    }

    /// Set the list of roles that the member should have.
    ///
    /// **Note**: Requires the [Manage Roles] permission to modify.
    ///
    /// [Manage Roles]: Permissions::MANAGE_ROLES
    pub fn roles(mut self, roles: impl IntoIterator<Item = impl Into<RoleId>>) -> Self {
        self.roles = Some(roles.into_iter().map(Into::into).collect());
        self
    }

    /// Move the member into a voice channel.
    ///
    /// **Note**: Requires the [Move Members] permission.
    ///
    /// [Move Members]: Permissions::MOVE_MEMBERS
    #[inline]
    pub fn voice_channel(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.channel_id = Some(Some(channel_id.into()).into());
        self
    }

    /// Disconnects the user from their voice channel, if any.
    ///
    /// **Note**: Requires the [Move Members] permission.
    ///
    /// [Move Members]: Permissions::MOVE_MEMBERS
    pub fn disconnect_member(mut self) -> Self {
        self.channel_id = Some(None.into());
        self
    }

    /// Times the user out until `time`, an ISO8601-formatted datetime string.
    ///
    /// `time` is considered invalid if it is not a valid ISO8601 timestamp or if it is greater
    /// than 28 days from the current time.
    ///
    /// **Note**: Requires the [Moderate Members] permission.
    ///
    /// [Moderate Members]: Permissions::MODERATE_MEMBERS
    #[doc(alias = "timeout")]
    pub fn disable_communication_until(mut self, time: String) -> Self {
        self.communication_disabled_until = Some(Some(time).into());
        self
    }

    /// Times the user out until `time`.
    ///
    /// `time` is considered invalid if it is greater than 28 days from the current time.
    ///
    /// **Note**: Requires the [Moderate Members] permission.
    ///
    /// [Moderate Members]: Permissions::MODERATE_MEMBERS
    #[doc(alias = "timeout")]
    pub fn disable_communication_until_datetime(self, time: Timestamp) -> Self {
        self.disable_communication_until(time.to_string())
    }

    /// Allow a user to communicate, removing their timeout, if there is one.
    ///
    /// **Note**: Requires the [Moderate Members] permission.
    ///
    /// [Moderate Members]: Permissions::MODERATE_MEMBERS
    #[doc(alias = "timeout")]
    pub fn enable_communication(mut self) -> Self {
        self.communication_disabled_until = Some(None.into());
        self
    }

    pub fn flags(mut self, flags: GuildMemberFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Sets the request's audit log reason.
    pub fn audit_log_reason(mut self, reason: &'a str) -> Self {
        self.audit_log_reason = Some(reason);
        self
    }
}

#[cfg(feature = "http")]
#[async_trait::async_trait]
impl Builder for EditMember<'_> {
    type Context<'ctx> = (GuildId, UserId);
    type Built = Member;

    /// Edits the properties of the guild member.
    ///
    /// For details on permissions requirements, refer to each specific method.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if invalid data is given.
    async fn execute(
        self,
        cache_http: impl CacheHttp,
        ctx: Self::Context<'_>,
    ) -> Result<Self::Built> {
        cache_http.http().edit_member(ctx.0, ctx.1, &self, self.audit_log_reason).await
    }
}
