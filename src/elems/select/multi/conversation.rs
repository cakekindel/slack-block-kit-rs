//! # Multi-Select Conversation List
//!
//! [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select)
//!
//! This select menu will populate its options with a list of public and private channels,
//! DMs, and MPIMs visible to the current user in the active workspace.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::{Confirm, ConversationFilter},
            elems::select::conversation::build,
            text,
            val_helpr::ValidationResult};

/// # Multi-Select Conversation List
///
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select)
///
/// This select menu will populate its options with a list of public and private channels,
/// DMs, and MPIMs visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Conversation<'a> {
  #[validate(custom = "crate::elems::select::validate::placeholder")]
  pub(in crate::elems::select) placeholder: text::Text,

  #[validate(length(max = 255))]
  pub(in crate::elems::select) action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  pub(in crate::elems::select) confirm: Option<Confirm>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(in crate::elems::select) initial_channels: Option<Cow<'a, [String]>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(in crate::elems::select) default_to_current_conversation: Option<bool>,

  #[validate]
  pub(in crate::elems::select) filter: Option<ConversationFilter>,

  #[validate(range(min = 1))]
  pub(in crate::elems::select) max_selected_items: Option<u32>,
}

impl<'a> Conversation<'a> {
  /// Build a new conversation multi-select element
  ///
  /// # Examples
  /// ```
  /// // TODO(#130)
  /// ```
  pub fn builder() -> build::MultiConversationBuilderInit<'a> {
    build::MultiConversationBuilderInit::new()
  }

  /// Validate that this conversation select agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_placeholder_and_action_id` was called with
  ///     `placeholder` longer than 150 chars
  /// - If `from_placeholder_and_action_id` was called with
  ///     `action_id` longer than 255 chars
  /// - If `with_confirm` was called with an invalid `Confirm` structure
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  ///
  /// let select = select::multi::Conversation::builder().placeholder(
  ///                           r#"Hey I really would appreciate it if you chose
  ///         a channel relatively soon, so that we can figure out
  ///         where we need to send this poll, ok? it's kind of
  ///         important that you specify where this poll should be
  ///         sent, in case we haven't made that super clear.
  ///         If you understand, could you pick a channel, already??"#,
  /// )
  ///              .action_id("ABC123")
  ///              .build();
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(&self)
  }
}
