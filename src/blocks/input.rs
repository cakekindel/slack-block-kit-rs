use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::block_elements::select;
use crate::compose;
use crate::val_helpr::ValidationResult;

/// # Input Block
///
/// [slack api docs 🔗]
///
/// A block that collects information from users -
///
/// Read [slack's guide to using modals 🔗]
/// to learn how input blocks pass information to your app.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#input
/// [slack's guide to using modals 🔗]: https://api.slack.com/surfaces/modals/using#gathering_input
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    #[validate(custom = "validation::text_max_len_2k")]
    label: compose::Text,

    element: InputElement,

    #[validate(length(max = 255))]
    block_id: Option<String>,

    #[validate(custom = "validation::text_max_len_2k")]
    hint: Option<compose::Text>,

    optional: Option<bool>,
}

impl Contents {
    /// Create an Input Block from a text Label and interactive element.
    ///
    /// # Arguments
    ///
    /// - `label` - A label that appears above an input element in the form of
    ///     a [text object 🔗] that must have type of `plain_text`.
    ///     Maximum length for the text in this field is 2000 characters.
    ///
    /// - `element` - An interactive `block_element` that will be used to gather
    ///     the input for this block.
    ///     For the kinds of Elements supported by
    ///     Input blocks, see the `InputElement` enum.
    ///     For info about Block Elements in general,
    ///     see the `block_elements` module.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let label = compose::Text::plain("On a scale from 1 - 5, how angsty are you?");
    /// let input = select::Static {};
    ///
    /// let block = blocks::input::Contents::from_label_and_element(label, input);
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_label_and_element<Label: Into<compose::Text>, El: Into<InputElement>>(
        label: Label,
        element: El,
    ) -> Self {
        Contents {
            label: label.into(),
            element: element.into(),
            block_id: None,
            hint: None,
            optional: None,
        }
    }

    /// Set a unique `block_id` to identify this instance of an Input Block.
    ///
    /// # Arguments
    ///
    /// - `block_id` - A string acting as a unique identifier for a block.
    ///     You can use this `block_id` when you receive an interaction
    ///     payload to [identify the source of the action 🔗].
    ///     If not specified, one will be generated.
    ///     Maximum length for this field is 255 characters.
    ///     `block_id` should be unique for each message and each iteration of a message.
    ///     If a message is updated, use a new `block_id`.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let label = compose::Text::plain("On a scale from 1 - 5, how angsty are you?");
    /// let input = select::Static {};
    ///
    /// let block = blocks::input
    ///     ::Contents
    ///     ::from_label_and_element(label, input)
    ///     .with_block_id("angst_rating_12345");
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_block_id<StrIsh: AsRef<str>>(mut self, block_id: StrIsh) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }

    /// Set the `hint` on this Input Block that appears below
    /// an input element in a lighter grey.
    ///
    /// # Arguments
    ///
    /// - `hint` - An optional hint that appears below an input element
    ///     in a lighter grey.
    ///     It must be a a [text object 🔗] with a `type` of `plain_text`.
    ///     Maximum length for the `text` in this field is 2000 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let label = compose::Text::plain("On a scale from 1 - 5, how angsty are you?");
    /// let input = select::Static {};
    ///
    /// let block = blocks::input
    ///     ::Contents
    ///     ::from_label_and_element(label, input)
    ///     .with_hint(compose::Text::plain("PSST hey! Don't let them know how angsty you are!"));
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_hint<IntoText: Into<compose::Text>>(mut self, hint: IntoText) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Set whether or not this input is Optional.
    ///
    /// # Arguments
    /// - `optionality` - A boolean that indicates whether the input
    ///     element may be empty when a user submits the modal.
    ///     Defaults to false.
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let label = compose::Text::plain("On a scale from 1 - 5, how angsty are you?");
    /// let input = select::Static {};
    ///
    /// let block = blocks::input
    ///     ::Contents
    ///     ::from_label_and_element(label, input)
    ///     .with_hint(compose::Text::plain("PSST hey! Don't even answer that!"))
    ///     .with_optional(true);
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_optional(mut self, optionality: bool) -> Self {
        self.optional = Some(optionality);
        self
    }

    /// Validate that this Input block agrees with Slack's model requirements
    ///
    /// # Errors
    /// - If `from_label_and_element` was passed a Text object longer
    ///     than 2000 chars
    /// - If `with_hint` was called with a block id longer
    ///     than 2000 chars
    /// - If `with_block_id` was called with a block id longer
    ///     than 256 chars
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let label = compose::Text::plain("On a scale from 1 - 5, how angsty are you?");
    /// let input = select::Static {};
    /// let long_string = std::iter::repeat(' ').take(2001).collect::<String>();
    ///
    /// let block = blocks::input
    ///     ::Contents
    ///     ::from_label_and_element(label, input)
    ///     .with_block_id(long_string);
    ///
    /// assert_eq!(true, matches!(block.validate(), Err(_)));
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

/// Enum representing the [`BlockElement` 🔗] types
/// supported by InputElement.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum InputElement {
    Checkboxes,
    DatePicker,
    MultiSelect,
    Select(select::Contents),
    PlainInput,
    RadioButtons,
}

impl<T> From<T> for InputElement
where
    T: Into<select::Contents>,
{
    fn from(contents: T) -> Self {
        InputElement::Select(contents.into())
    }
}

mod validation {
    use crate::compose;
    use crate::val_helpr::ValidatorResult;

    pub fn text_max_len_2k(text: &compose::Text) -> ValidatorResult {
        compose::validation::text_max_len(text, 2000)
    }
}
