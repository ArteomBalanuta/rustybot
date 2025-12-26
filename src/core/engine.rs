use tokio::sync::mpsc;

use crate::{core::event_handler::EngineCommand, model::User};

use std::collections::HashMap;

pub trait Engine {
    async fn start(&mut self);
    fn Stop(&self);

    fn DispatchMessage(&self, message: &str);
    fn SendRawMessage(&self, message: &str);
    fn SendChatMessage(&self, author: &str, message: &str, isWhisper: bool) -> String;

    fn AddActiveUser(&self, joined: User);
    fn RemoveActiveUser(&self, left: &User);

    fn AddAfkUser(&self, u: &User, reason: &str);
    fn GetAfkUsers(&self) -> HashMap<User, String>;

    fn GetActiveUserByName(&self, name: &str) -> User;
    fn GetActiveUsers(&self) -> HashMap<User, String>;

    fn Kick(&self, name: &str, channel: &str);

    fn GetPrefix(&self) -> String;
    fn GetName(&self) -> String;
    fn GetChannel(&self) -> String;

    fn SetName(&self, name: &str);

    // fn Ban(name string)
    // fn Unban(hash string)
    // fn UnbanAll()
    // fn Lock()
    // fn Unlock()

    // fn RegisterCommand(c: Command);
    // fn GetEnabledCommands() *map[string]CommandMetadata

    // fn SetOnlineSetListener(l Listener)
    // fn WaitConnectionWgDone() // TODO: ugly refactor

    // fn LogMessage(trip, name, hash, message, channel string) (int64, error)
    // fn LogPresence(trip, name, hash, eventType, channel string) (int64, error)

    // fn SetLastKickedUser(name string)
    // fn SetLastKickedChannel(channel string)

    // fn IsUserAuthorized(u *model.User, r *model.Role) bool

    // fn NotifyAfkIfMentioned(m *model.ChatMessage)
    // fn RemoveIfAfk(u *model.User)
}
