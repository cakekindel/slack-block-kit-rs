//! # Multi-Select User List
//!
//! [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_multi_select)
//!
//! This multi-select menu will populate its options with
//! a list of Slack users visible to the current user in the active workspace.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
#[cfg(feature = "validation")]
use validator::Validate;

#[cfg(feature = "validation")]
use crate::val_helpr::ValidationResult;
use crate::{compose::Confirm, elems::select::user::build, text};

/// # Multi-Select User List
///
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_multi_select)
///
/// This multi-select menu will populate its options with
/// a list of Slack users visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct User<'a> {
  #[cfg_attr(feature = "validation",
             validate(custom = "crate::elems::select::validate::placeholder"))]
  pub(in crate::elems::select) placeholder: text::Text,

  #[cfg_attr(feature = "validation", validate(length(max = 255)))]
  pub(in crate::elems::select) action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate)]
  pub(in crate::elems::select) confirm: Option<Confirm>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(in crate::elems::select) initial_users: Option<Vec<Cow<'a, str>>>,

  #[cfg_attr(feature = "validation", validate(range(min = 1)))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(in crate::elems::select) max_selected_items: Option<u32>,
}

impl<'a> User<'a> {
  /// Build a new user multi-select element
  ///
  /// # Examples
  /// ```
  /// // TODO(#130)
  /// ```
  pub fn builder() -> build::MultiUserBuilderInit<'a> {
    build::MultiUserBuilderInit::new()
  }

  /// Validate that this user select agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_placeholder_and_action_id` was called with
  ///     `placeholder` longer than 150 chars
  /// - If `from_placeholder_and_action_id` was called with
  ///     `action_id` longer than 255 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  ///
  /// let select = select::multi::User::builder().placeholder(
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
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(&self)
  }
}
