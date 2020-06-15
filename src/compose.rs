use serde::{Deserialize, Serialize};

use crate::impl_from_contents;

pub mod validation {
    use crate::val_helpr::error;
    use validator::ValidationError;
    type ValidationResult = Result<(), ValidationError>;

    pub fn text_is_plain(text: &super::Text) -> ValidationResult {
        match text {
            super::Text::Markdown { .. } => {
                Err(error("text_is_plain", "expected plain, got markdown"))
            }
            super::Text::Plain { .. } => Ok(()),
        }
    }

    pub fn text_max_len(text: &super::Text, max_len: usize) -> ValidationResult {
        let len = text.text().chars().count();

        if len > max_len {
            let message = format!(
                "Section#text has max len of {}, but got text of len {}.",
                max_len, len
            );

            Err(error("text_max_len", message))
        } else {
            Ok(())
        }
    }
}

/// # Composition Objects
///
/// Composition objects can be used inside of [block elements 🔗] and certain message payload fields.
///
/// They are simply common JSON object patterns that you'll encounter frequently
/// when building blocks or composing messages.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Compose {
    Text(Text),
}

impl_from_contents!(Compose, Text, Text);

/// # Text Object
/// [_slack api docs 🔗_](https://api.slack.com/reference/block-kit/composition-objects#text)
///
/// An object containing some text,
/// formatted either as `plain_text`
/// or using [`mrkdwn` 🔗](https://api.slack.com/reference/surfaces/formatting),
/// our proprietary textual markup that's just different enough
/// from Markdown to frustrate you.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Text {
    /// ## Markdown text
    /// [_for more info, check out the slack api docs 🔗_](https://api.slack.com/reference/surfaces/formatting)
    ///
    /// ### Reserved Characters
    ///
    /// Slack uses the following special characters,
    /// and recommends you HTML escape them like so:
    ///
    /// <details>
    /// <summary><b>➤ Click to expand</b></summary>
    ///
    /// |character|how to escape|
    /// |---      |---          |
    /// |`&`      |`&amp;`      |
    /// |`<`      |`&lt;`       |
    /// |`>`      |`&gt;`       |
    /// </details>
    ///
    /// ### Basic Formatting
    ///
    /// NOTE: This is **not** an exhaustive list
    ///
    /// This should, however, capture most basic
    /// use cases without requiring that you check with
    /// the Slack documentation.
    ///
    /// For more info, please visit
    /// [Slack's docs for markdown formatting 🔗](https://api.slack.com/reference/surfaces/formatting)
    ///
    /// <details>
    /// <summary><b>➤ Click to expand</b></summary>
    ///
    /// <!-- wow - markdown tables strike again! -->
    /// |slack markdown    |formatted result     |
    /// |---               |---                  |
    /// |`_italic_`        |_italic_             |
    /// |`*bold*`          |**bold**             |
    /// |`~strike~`        |<del>strike</del>    |
    /// |`\n`              |line break           |
    /// |`> a block quote` | <blockquote> a block quote </blockquote> |
    /// |`` `some code!` ``| `some code!`        |
    /// |`` ```multiline code\n2 lines!``` `` | <code>multiline code<br> 2 lines!</code> |
    /// |` - li \n - li `  | <ul><li>li</li><li>li</li></ul> |
    /// |<code>&lt;http://www.foo.com&#124;link name&gt;</code>| [link name](http://www.foo.com) |
    /// |`:joy:` (list from [iamcal/emoji-data 🔗](https://github.com/iamcal/emoji-data)) | 😂 |
    /// | link to #channel: `<#Cxxxxxx>` | [#channel](https://work.slack.com/some-public-channel) |
    /// | link to @user: `<@Uxxxxxx>` | [@user](https://work.slack.com/some-user) |
    /// | link to @user_group: `<!subteam^xxxxxx>` | [@user_group](https://work.slack.com/some-user-group) |
    /// </details>
    #[serde(rename = "mrkdwn")]
    Markdown {
        /// The text for the block.
        ///
        /// This field accepts any of the [standard text formatting markup](#markdown-text)
        text: String,
        /// When set to false (as is default)
        /// URLs will be auto-converted into links,
        /// conversation names will be link-ified,
        /// and certain mentions will be automatically parsed.
        ///
        /// Using a value of true will skip any preprocessing
        /// of this nature, although you can
        /// still include manual parsing strings.
        verbatim: Option<bool>,
    },
    #[serde(rename = "plain_text")]
    Plain {
        /// The text for the block
        text: String,
        /// Indicates whether emojis in a text field
        /// should be escaped into the colon emoji format
        emoji: Option<bool>,
    },
}

impl Default for Text {
    fn default() -> Self {
        Text::Markdown {
            text: String::new(),
            verbatim: None,
        }
    }
}

impl Text {
    pub fn plain<StrIsh: AsRef<str>>(text: StrIsh) -> Text {
        Text::Plain {
            text: text.as_ref().to_string(),
            emoji: None,
        }
    }

    pub fn markdown<StrIsh: AsRef<str>>(text: StrIsh) -> Text {
        Text::Markdown {
            text: text.as_ref().to_string(),
            verbatim: None,
        }
    }

    pub fn text(&self) -> &str {
        use Text::*;

        match self {
            Plain { text, .. } => text,
            Markdown { text, .. } => text,
        }
    }
}
