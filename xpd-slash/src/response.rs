use twilight_model::{
    application::command::CommandOptionChoice,
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::{attachment::Attachment, interaction::InteractionResponseData},
};

#[derive(Debug, Default, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct XpdSlashResponse {
    pub allowed_mentions: Option<AllowedMentions>,
    pub attachments: Option<Vec<Attachment>>,
    pub choices: Option<Vec<CommandOptionChoice>>,
    pub components: Option<Vec<Component>>,
    pub content: Option<String>,
    pub custom_id: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub flags: Option<MessageFlags>,
    pub title: Option<String>,
    pub tts: Option<bool>,
}

impl XpdSlashResponse {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn allowed_mentions_o(self, allowed_mentions: Option<AllowedMentions>) -> Self {
        Self {
            allowed_mentions,
            ..self
        }
    }
    pub fn attachments_o(self, attachments: Option<impl Into<Vec<Attachment>>>) -> Self {
        Self {
            attachments: attachments.map(std::convert::Into::into),
            ..self
        }
    }
    pub fn choices_o(self, choices: Option<impl Into<Vec<CommandOptionChoice>>>) -> Self {
        Self {
            choices: choices.map(|v| v.into()),
            ..self
        }
    }
    pub fn components_o(self, components: Option<impl Into<Vec<Component>>>) -> Self {
        Self {
            components: components.map(|v| v.into()),
            ..self
        }
    }
    pub fn content_o(self, content: Option<String>) -> Self {
        Self { content, ..self }
    }
    pub fn custom_id_o(self, custom_id: Option<String>) -> Self {
        Self { custom_id, ..self }
    }
    pub fn embeds_o(self, embeds: Option<impl Into<Vec<Embed>>>) -> Self {
        Self {
            embeds: embeds.map(|v| v.into()),
            ..self
        }
    }
    pub fn flags_o(self, flags: Option<MessageFlags>) -> Self {
        Self { flags, ..self }
    }
    pub fn title_o(self, title: Option<String>) -> Self {
        Self { title, ..self }
    }
    pub fn tts_o(self, tts: Option<bool>) -> Self {
        Self { tts, ..self }
    }
    pub fn allowed_mentions(self, allowed_mentions: AllowedMentions) -> Self {
        self.allowed_mentions_o(Some(allowed_mentions))
    }
    pub fn attachments(self, attachments: impl Into<Vec<Attachment>>) -> Self {
        self.attachments_o(Some(attachments))
    }
    pub fn choices(self, choices: impl Into<Vec<CommandOptionChoice>>) -> Self {
        self.choices_o(Some(choices))
    }
    pub fn components(self, components: impl Into<Vec<Component>>) -> Self {
        self.components_o(Some(components))
    }
    pub fn content(self, content: String) -> Self {
        self.content_o(Some(content))
    }
    pub fn custom_id(self, custom_id: String) -> Self {
        self.custom_id_o(Some(custom_id))
    }
    pub fn embeds(self, embeds: impl Into<Vec<Embed>>) -> Self {
        self.embeds_o(Some(embeds))
    }
    pub fn flags(self, flags: MessageFlags) -> Self {
        self.flags_o(Some(flags))
    }
    pub fn title(self, title: String) -> Self {
        self.title_o(Some(title))
    }
    pub fn tts(self, tts: bool) -> Self {
        self.tts_o(Some(tts))
    }
}

impl From<XpdSlashResponse> for InteractionResponseData {
    fn from(value: XpdSlashResponse) -> Self {
        Self {
            allowed_mentions: value.allowed_mentions,
            attachments: value.attachments,
            choices: value.choices,
            components: value.components,
            content: value.content,
            custom_id: value.custom_id,
            embeds: value.embeds,
            flags: value.flags,
            title: value.title,
            tts: value.tts,
        }
    }
}

impl From<InteractionResponseData> for XpdSlashResponse {
    fn from(value: InteractionResponseData) -> Self {
        Self {
            allowed_mentions: value.allowed_mentions,
            attachments: value.attachments,
            choices: value.choices,
            components: value.components,
            content: value.content,
            custom_id: value.custom_id,
            embeds: value.embeds,
            flags: value.flags,
            title: value.title,
            tts: value.tts,
        }
    }
}