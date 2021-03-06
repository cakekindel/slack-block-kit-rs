//! # Checkbox Group
//!
//! A checkbox group that allows a user to choose multiple items from a list of possible options.
//!
//! [slack api docs 🔗]
//!
//! Works in [blocks 🔗]: Section, Actions, Input
//! Works in [app surfaces 🔗]: Home tabs, Modals, Messages
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#checkboxes
//! [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
//! [app surfaces 🔗]: https://api.slack.com/surfaces

use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
#[cfg(feature = "validation")]
use validator::Validate;

#[cfg(feature = "validation")]
use crate::val_helpr::*;
use crate::{compose::{opt::{AnyText, NoUrl},
                      Confirm,
                      Opt},
            text};

type MyOpt<'a> = Opt<'a, AnyText, NoUrl>;

#[cfg(feature = "validation")]
fn validate_options<'a>(o: &Cow<'a, [MyOpt<'a>]>) -> ValidatorResult {
  below_len("options", 10, o.as_ref())
}

#[cfg(feature = "validation")]
fn validate_initial_options<'a>(o: &Cow<'a, [MyOpt<'a>]>) -> ValidatorResult {
  below_len("initial_options", 10, o.as_ref())
}

/// # Checkbox Group
///
/// A checkbox group that allows a user to choose multiple items from a list of possible options.
///
/// [slack api docs 🔗]
///
/// Works in [blocks 🔗]: Section, Actions, Input
/// Works in [app surfaces 🔗]: Home tabs, Modals, Messages
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#checkboxes
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
/// [app surfaces 🔗]: https://api.slack.com/surfaces
#[derive(Clone, Debug, Hash, PartialEq, Ser, De)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct Checkboxes<'a> {
  #[cfg_attr(feature = "validation", validate(length(max = 255)))]
  action_id: Cow<'a, str>,

  #[cfg_attr(feature = "validation", validate(custom = "validate_options"))]
  options: Cow<'a, [MyOpt<'a>]>,

  #[cfg_attr(feature = "validation",
             validate(custom = "validate_initial_options"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  initial_options: Option<Cow<'a, [MyOpt<'a>]>>,

  #[cfg_attr(feature = "validation", validate)]
  #[serde(skip_serializing_if = "Option::is_none")]
  confirm: Option<Confirm>,
}

impl<'a> Checkboxes<'a> {
  /// Build a new checkboxes element.
  ///
  /// # Example
  /// see example for `build::CheckboxesBuilder`.
  pub fn builder() -> build::CheckboxesBuilderInit<'a> {
    build::CheckboxesBuilderInit::new()
  }

  /// Validate that this element agrees with Slack's model requirements.
  ///
  /// # Errors
  /// - length of `action_id` greater than 255
  /// - length of `options` greater than 10
  /// - length of `initial_options` greater than 10
  /// - one or more of `options` is invalid // TODO
  /// - one or more of `initial_options` is invalid // TODO
  /// - `initial_option` is set and an invalid `Opt`
  /// - `confirm` is set and an invalid `Confirm`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{compose::Opt, elems::Checkboxes};
  ///
  /// fn repeat<T: Copy>(el: T, n: usize) -> impl Iterator<Item = T> {
  ///   std::iter::repeat(el).take(n)
  /// }
  ///
  /// let long_string: String = repeat('a', 256).collect();
  /// let opt = Opt::builder().text_md("foo").value("bar").build();
  ///
  /// let opts = repeat(&opt, 11).map(|o| o.clone()).collect::<Vec<_>>();
  ///
  /// let input = Checkboxes::builder().action_id(long_string)
  ///                                  .options(opts)
  ///                                  .build();
  ///
  /// assert!(matches!(input.validate(), Err(_)))
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Checkbox group builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// CheckboxesBuilder.action_id
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
    /// CheckboxesBuilder.options
    #[derive(Copy, Clone, Debug)]
    pub struct options;
  }

  /// Initial state for Checkbox builder
  pub type CheckboxesBuilderInit<'a> =
    CheckboxesBuilder<'a,
                      RequiredMethodNotCalled<method::action_id>,
                      RequiredMethodNotCalled<method::options>>;

  /// Checkbox group builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `CheckboxesBuilder::build()` is only available if these methods have been called:
  ///  - `action_id`
  ///  - `options`
  ///
  /// # Example
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    elems::{BlockElement, Checkboxes}};
  ///
  /// mod usa {
  ///   pub struct State {
  ///     pub name: String,
  ///     pub abbrev: String,
  ///   }
  ///
  ///   pub fn arizona() -> State {
  ///     State { name: String::from("Arizona"),
  ///             abbrev: String::from("AZ") }
  ///   }
  ///
  ///   pub fn get_states() -> Vec<State> {
  ///     // ...
  ///     # vec![]
  ///   }
  /// }
  ///
  /// let state_opt = |state: usa::State| {
  ///   Opt::builder().text_plain(state.name)
  ///                 .value(state.abbrev)
  ///                 .build()
  /// };
  ///
  /// let states: Vec<Opt<_, _>> =
  ///   usa::get_states().into_iter().map(state_opt).collect();
  ///
  /// let boxes =
  ///   Checkboxes::builder().action_id("state_picker")
  ///                        .options(states)
  ///                        .initial_options(vec![state_opt(usa::arizona())])
  ///                        .build();
  ///
  /// let block: Block = Actions::builder().element(boxes).build().into();
  ///
  /// // <send block to slack API>
  /// ```
  #[derive(Debug)]
  pub struct CheckboxesBuilder<'a, A, O> {
    action_id: Option<Cow<'a, str>>,
    options: Option<Vec<MyOpt<'a>>>,
    initial_options: Option<Vec<MyOpt<'a>>>,
    confirm: Option<Confirm>,
    state: PhantomData<(A, O)>,
  }

  impl<'a, A, O> CheckboxesBuilder<'a, A, O> {
    /// Create a new builder
    pub fn new() -> Self {
      Self { action_id: None,
             options: None,
             initial_options: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Set `action_id` (Optional)
    ///
    /// An identifier for the action triggered when the checkbox group is changed.
    ///
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///
    /// Should be unique among all other `action_id`s in the containing block.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn action_id<S>(self,
                        action_id: S)
                        -> CheckboxesBuilder<'a, Set<method::action_id>, O>
      where S: Into<Cow<'a, str>>
    {
      CheckboxesBuilder { action_id: Some(action_id.into()),
                          options: self.options,
                          initial_options: self.initial_options,
                          confirm: self.confirm,
                          state: PhantomData::<_> }
    }

    /// Append an `option` to `options`
    #[cfg(feature = "blox")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
    pub fn child<T: Into<text::Text>>(
      self,
      option: Opt<'a, T, NoUrl>)
      -> CheckboxesBuilder<'a, A, Set<method::options>> {
      self.option(option)
    }

    /// Set `options` (**Required**)
    ///
    /// An array of [option objects 🔗].
    ///
    /// A maximum of 10 options are allowed.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option
    pub fn options<T: Into<text::Text>>(
      self,
      options: Vec<Opt<'a, T, NoUrl>>)
      -> CheckboxesBuilder<'a, A, Set<method::options>> {
      CheckboxesBuilder { action_id: self.action_id,
                          options: Some(options.into_iter()
                                               .map(|o| o.into())
                                               .collect()),
                          initial_options: self.initial_options,
                          confirm: self.confirm,
                          state: PhantomData::<_> }
    }

    /// Append an `option` to `options`
    pub fn option<T: Into<text::Text>>(
      self,
      option: Opt<'a, T, NoUrl>)
      -> CheckboxesBuilder<'a, A, Set<method::options>> {
      let options = match self.options {
        | Some(mut options) => {
          options.push(option.into());
          options
        },
        | None => vec![option.into()],
      };

      CheckboxesBuilder { action_id: self.action_id,
                          options: Some(options),
                          initial_options: self.initial_options,
                          confirm: self.confirm,
                          state: PhantomData::<_> }
    }

    /// Set `initial_options` (Optional)
    ///
    /// An array of [option objects 🔗] that exactly matches one or more
    /// of the options within `options`.
    ///
    /// These options will be selected when the checkbox group initially loads.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#option
    pub fn initial_options<T: Into<text::Text>>(mut self,
                                                options: Vec<Opt<'a,
                                                        T,
                                                        NoUrl>>)
                                                -> Self {
      self.initial_options =
        Some(options.into_iter().map(|o| o.into()).collect());
      self
    }

    /// Set `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an optional confirmation dialog
    /// that appears after clicking one of the checkboxes in this element.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }
  }

  impl<'a> CheckboxesBuilder<'a, Set<method::action_id>, Set<method::options>> {
    /// All done building, now give me a darn checkbox group!
    ///
    /// > `no method name 'build' found for struct 'CheckboxesBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `CheckboxesBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::Checkboxes;
    ///
    /// let foo = Checkboxes::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{compose::Opt, elems::Checkboxes};
    ///
    /// let foo = Checkboxes::builder().action_id("foo")
    ///                                .options(vec![Opt::builder().text_plain("foo")
    ///                                                            .value("bar")
    ///                                                            .build()])
    ///                                .build();
    /// ```
    pub fn build(self) -> Checkboxes<'a> {
      Checkboxes { action_id: self.action_id.unwrap(),
                   options: self.options.unwrap().into(),
                   initial_options: self.initial_options.map(|os| os.into()),
                   confirm: self.confirm }
    }
  }
}
