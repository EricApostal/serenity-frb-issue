#[cfg(feature = "http")]
use super::Builder;
use super::CreateForumTag;
#[cfg(feature = "http")]
use crate::http::CacheHttp;
#[cfg(feature = "http")]
use crate::internal::prelude::*;
use crate::model::prelude::*;

/// A builder to edit a [`GuildChannel`] for use via [`GuildChannel::edit`].
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#modify-channel-json-params-guild-channel).
///
/// # Examples
///
/// Edit a channel, providing a new name and topic:
///
/// ```rust,no_run
/// # use serenity::builder::EditChannel;
/// # use serenity::http::Http;
/// # use serenity::model::channel::GuildChannel;
/// #
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let http: Http = unimplemented!();
/// # let mut channel: GuildChannel = unimplemented!();
/// let builder = EditChannel::new().name("new name").topic("a test topic");
/// if let Err(why) = channel.edit(&http, builder).await {
///     // properly handle the error
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct EditChannel<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    kind: Option<ChannelType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<PermissionOverwriteData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Box<Option<ChannelId>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rtc_region: Option<Box<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_quality_mode: Option<VideoQualityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<ChannelFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_tags: Option<Vec<CreateForumTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_reaction_emoji: Option<Box<Option<ForumEmoji>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_thread_rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_forum_layout: Option<ForumLayoutType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,

    #[serde(skip)]
    audit_log_reason: Option<&'a str>,
}

impl<'a> EditChannel<'a> {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// The bitrate of the channel in bits.
    ///
    /// This is for [voice] channels only.
    ///
    /// [voice]: ChannelType::Voice
    pub fn bitrate(mut self, bitrate: u32) -> Self {
        self.bitrate = Some(bitrate);
        self
    }

    /// The camera video quality mode of the channel.
    ///
    /// This is for [voice] channels only.
    ///
    /// [voice]: ChannelType::Voice
    pub fn video_quality_mode(mut self, quality: VideoQualityMode) -> Self {
        self.video_quality_mode = Some(quality);
        self
    }

    /// The voice region of the channel. It is automatic when `None`.
    ///
    /// This is for [voice] channels only.
    ///
    /// [voice]: ChannelType::Voice
    pub fn voice_region(mut self, id: Option<String>) -> Self {
        self.rtc_region = Some(id.into());
        self
    }

    /// The name of the channel.
    ///
    /// Must be between 2 and 100 characters long.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// The position of the channel in the channel list.
    pub fn position(mut self, position: u16) -> Self {
        self.position = Some(position);
        self
    }

    /// The topic of the channel. Can be empty.
    ///
    /// Must be between 0 and 1024 characters long.
    ///
    /// This is for [text] channels only.
    ///
    /// [text]: ChannelType::Text
    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }

    /// The status of the voice channel. Can be empty.
    ///
    /// Must be between 0 and 1024 characters long.
    ///
    /// This is for [voice] channels only.
    ///
    /// [voice]: ChannelType::Voice
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Is the channel inappropriate for work?
    ///
    /// This is for [text] channels only.
    ///
    /// [text]: ChannelType::Text
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// The number of users that may be in the channel simultaneously.
    ///
    /// This is for [voice] channels only.
    ///
    /// [voice]: ChannelType::Voice
    pub fn user_limit(mut self, user_limit: u32) -> Self {
        self.user_limit = Some(user_limit);
        self
    }

    /// The parent category of the channel.
    ///
    /// This is for [text] and [voice] channels only.
    ///
    /// [text]: ChannelType::Text
    /// [voice]: ChannelType::Voice
    #[inline]
    pub fn category<C: Into<Option<ChannelId>>>(mut self, category: C) -> Self {
        self.parent_id = Some(category.into().into());
        self
    }

    /// How many seconds must a user wait before sending another message.
    ///
    /// Bots, or users with the [`MANAGE_MESSAGES`] and/or [`MANAGE_CHANNELS`] permissions are
    /// exempt from this restriction.
    ///
    /// **Note**: Must be between 0 and 21600 seconds (360 minutes or 6 hours).
    ///
    /// [`MANAGE_MESSAGES`]: Permissions::MANAGE_MESSAGES
    /// [`MANAGE_CHANNELS`]: Permissions::MANAGE_CHANNELS
    #[doc(alias = "slowmode")]
    pub fn rate_limit_per_user(mut self, seconds: u16) -> Self {
        self.rate_limit_per_user = Some(seconds);
        self
    }

    /// A set of overwrites defining what a user or a user carrying a certain role can or can't do.
    ///
    /// # Example
    ///
    /// Inheriting permissions from an existing channel:
    ///
    /// ```rust,no_run
    /// # use serenity::builder::EditChannel;
    /// # use serenity::http::Http;
    /// # use serenity::model::channel::GuildChannel;
    /// # use std::sync::Arc;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Arc<Http> = unimplemented!();
    /// # let mut channel: GuildChannel = unimplemented!();
    /// use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
    /// use serenity::model::id::UserId;
    /// use serenity::model::permissions::Permissions;
    ///
    /// // Assuming a channel has already been bound.
    /// let permissions = vec![PermissionOverwrite {
    ///     allow: Permissions::VIEW_CHANNEL,
    ///     deny: Permissions::SEND_TTS_MESSAGES,
    ///     kind: PermissionOverwriteType::Member(UserId::new(1234)),
    /// }];
    ///
    /// let builder = EditChannel::new().name("my_edited_cool_channel").permissions(permissions);
    /// channel.edit(http, builder).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn permissions(mut self, perms: impl IntoIterator<Item = PermissionOverwrite>) -> Self {
        let overwrites = perms.into_iter().map(Into::into).collect::<Vec<_>>();

        self.permission_overwrites = Some(overwrites);
        self
    }

    /// If this is a forum channel, sets the tags that can be assigned to forum posts.
    pub fn available_tags(mut self, tags: impl IntoIterator<Item = CreateForumTag>) -> Self {
        self.available_tags = Some(tags.into_iter().collect());
        self
    }

    /// Sets the request's audit log reason.
    pub fn audit_log_reason(mut self, reason: &'a str) -> Self {
        self.audit_log_reason = Some(reason);
        self
    }

    /// The type of channel; only conversion between text and announcement is supported and only in
    /// guilds with the "NEWS" feature
    pub fn kind(mut self, kind: ChannelType) -> Self {
        self.kind = Some(kind);
        self
    }

    /// The default duration that the clients use (not the API) for newly created threads in the
    /// channel, in minutes, to automatically archive the thread after recent activity
    pub fn default_auto_archive_duration(
        mut self,
        default_auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        self.default_auto_archive_duration = Some(default_auto_archive_duration);
        self
    }

    /// Channel flags combined as a bitfield. Currently only [`ChannelFlags::REQUIRE_TAG`] is
    /// supported.
    pub fn flags(mut self, flags: ChannelFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// The emoji to show in the add reaction button on a thread in a forum channel
    pub fn default_reaction_emoji(mut self, default_reaction_emoji: Option<ForumEmoji>) -> Self {
        self.default_reaction_emoji = Some(default_reaction_emoji.into());
        self
    }

    /// The initial rate_limit_per_user to set on newly created threads in a channel. This field is
    /// copied to the thread at creation time and does not live update.
    pub fn default_thread_rate_limit_per_user(
        mut self,
        default_thread_rate_limit_per_user: u16,
    ) -> Self {
        self.default_thread_rate_limit_per_user = Some(default_thread_rate_limit_per_user);
        self
    }

    /// The default sort order type used to order posts in forum channels
    pub fn default_sort_order(mut self, default_sort_order: SortOrder) -> Self {
        self.default_sort_order = Some(default_sort_order);
        self
    }

    /// The default forum layout type used to display posts in forum channels
    pub fn default_forum_layout(mut self, default_forum_layout: ForumLayoutType) -> Self {
        self.default_forum_layout = Some(default_forum_layout);
        self
    }
}

#[cfg(feature = "http")]
#[async_trait::async_trait]
impl Builder for EditChannel<'_> {
    type Context<'ctx> = ChannelId;
    type Built = GuildChannel;

    /// Edits the channel's settings.
    ///
    /// **Note**: Requires the [Manage Channels] permission. Modifying permissions via
    /// [`Self::permissions`] also requires the [Manage Roles] permission.
    ///
    /// # Errors
    ///
    /// If the `cache` is enabled, returns a [`ModelError::InvalidPermissions`] if the current user
    /// lacks permission. Otherwise returns [`Error::Http`], as well as if invalid data is given.
    ///
    /// [Manage Channels]: Permissions::MANAGE_CHANNELS
    /// [Manage Roles]: Permissions::MANAGE_ROLES
    async fn execute(
        self,
        cache_http: impl CacheHttp,
        ctx: Self::Context<'_>,
    ) -> Result<Self::Built> {
        #[cfg(feature = "cache")]
        {
            if let Some(cache) = cache_http.cache() {
                crate::utils::user_has_perms_cache(cache, ctx, Permissions::MANAGE_CHANNELS)?;
                if self.permission_overwrites.is_some() {
                    crate::utils::user_has_perms_cache(cache, ctx, Permissions::MANAGE_ROLES)?;
                }
            }
        }

        if let Some(status) = &self.status {
            #[derive(Serialize)]
            struct EditVoiceStatusBody<'a> {
                status: &'a str,
            }

            cache_http
                .http()
                .edit_voice_status(
                    ctx,
                    &EditVoiceStatusBody { status: status.as_str() },
                    self.audit_log_reason,
                )
                .await?;
        }

        cache_http.http().edit_channel(ctx, &self, self.audit_log_reason).await
    }
}
