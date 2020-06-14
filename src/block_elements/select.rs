use serde::{Deserialize, Serialize};

/// # [Select Menu Element 🔗]
///
/// A select menu, just as with a standard HTML `<select>` tag,
/// creates a drop down menu with a list of options for a user to choose.
///
/// The select menu also includes type-ahead functionality, where a user can type
/// a part or all of an option string to filter the list.
///
/// To use interactive components, you will need to make some changes to prepare your app.
/// Read our [guide to enabling interactivity 🔗].
///
/// [Select Menu Element 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
/// [guide to enabling interactivity 🔗]: https://api.slack.com/interactivity/handling
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Contents {
    Static(StaticSelect),
    External(ExternalSelect),
    User(UserSelect),
    Conversation(ConversationSelect),
    Channel(ChannelSelect),
}

/// ## [Static Select 🔗]
///
/// This is the simplest form of select menu,
/// with a static list of options passed in when defining the element.
///
/// [Static Select 🔗]: https://api.slack.com/reference/block-kit/block-elements#static_select
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct StaticSelect {}

/// ## [Select menu with external data source 🔗]
///
/// This select menu will load its options from an external data source,
/// allowing for a dynamic list of options.
///
/// ### Setup
/// For a guide to set up your app to use this element type, go to the Slack
/// API section for [Select menu with external data source 🔗].
///
/// [Select menu with external data source 🔗]: https://api.slack.com/reference/block-kit/block-elements#external_select
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ExternalSelect {}

/// ## [Select menu with user list 🔗]
///
/// This select menu will populate its options with a list of
/// Slack users visible to the current user in the active workspace.
///
/// [Select menu with user list 🔗]: https://api.slack.com/reference/block-kit/block-elements#users_select
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct UserSelect {}

/// ## [Select menu with conversations list 🔗]
///
/// This select menu will populate its options with a list of public and private channels,
/// DMs, and MPIMs visible to the current user in the active workspace.
///
/// [Select menu with conversations list 🔗]: https://api.slack.com/reference/block-kit/block-elements#conversation_select
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ConversationSelect {}

/// ## [Select menu with channels list 🔗]
///
/// This select menu will populate its options with a list of
/// public channels visible to the current user in the active workspace.
///
/// [Select menu with channels list 🔗]: https://api.slack.com/reference/block-kit/block-elements#channel_select
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ChannelSelect {}

