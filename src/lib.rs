use std::{fmt::Display, ops::Deref};
use chrono::{DateTime, Utc};

#[derive(Clone, Copy)]
pub enum OnlineStatus {
    Online,
    Offline
}
impl Display for OnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            OnlineStatus::Offline => "offline",
            OnlineStatus::Online => "online"
        })
    }
}

/// Used to store a unique id for each user in Vortex
#[derive(Clone, Copy)]
pub struct UserId(u64);
impl Deref for UserId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<u64> for UserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl Into<u64> for UserId {
    fn into(self) -> u64 {
        self.0
    }
}
impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// struct used to store a user's information, such as friend count, bio, username, is_deleted, and more.
#[derive(Clone)]
pub struct VortexUser {
    pub id: UserId,
    pub username: String,
    pub bio: String,
    pub friend_count: usize,
    pub follower_count: usize,
    pub following_count: usize,
    pub visits: u64,

    pub online_status: OnlineStatus,
    pub created_at: DateTime<Utc>,

    pub is_deleted: bool,
    pub is_staff: bool,
    pub is_moderator: bool,
    pub is_booster: bool,
    pub is_content_creator: bool,
    
    pub shirt_id: u64,
    pub last_seen: DateTime<Utc>,
}
impl Display for VortexUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "id: {},\nusername: {},\nbio: {},\nfriend_count: {},\nfollower_count: {},\nfollowing_count: {},\nvisits: {},\nonline_status: {},\ncreated_at: {}\n",
            self.id, self.username, self.bio, self.friend_count, self.follower_count, self.following_count, self.visits, self.online_status, self.created_at
        )
    }
}

/// Used to preview a user's friend.
/// 
/// If you want to get every information of the user, use the `view` method
#[derive(Clone)]
pub struct VortexUserPreview {
    pub id: UserId,
    pub username: String,
    pub online_status: OnlineStatus
}
impl VortexUserPreview {
    pub async fn view(&self) -> Result<VortexUser, String> {
        get_user_from_id(self.id).await
    }
}
impl Display for VortexUserPreview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {},\nusername: {},\nonline_status: {}", self.id, self.username, self.online_status)
    }
}

/// Gets the user information from an UserId.
pub async fn get_user_from_id(user_id: impl Into<UserId>) -> Result<VortexUser, String> {
    let user_id = user_id.into();

    let mut response: serde_json::Value = reqwest::get(format!("https://playvortex.io/api/users/{}", user_id.0)).await.or(Err("Failed to get response"))?
        .json().await.or(Err("Failed to cast response into a json type"))?;

    if let Some(error) = response.get("detail") {
        return Err(error.as_str().unwrap().to_string());
    }

    Ok(VortexUser {
        id: user_id,
        username: response.get_mut("username").ok_or("Error getting username")?.take().as_str().unwrap().to_string(),
        bio: response.get_mut("bio").ok_or("Error getting bio")?.take().as_str().unwrap().to_string(),
        friend_count: response.get_mut("friend_count").ok_or("Error getting friend_count")?.take().as_u64().unwrap() as usize,
        follower_count: response.get_mut("follower_count").ok_or("Error getting followed_count")?.take().as_u64().unwrap() as usize,
        following_count: response.get_mut("following_count").ok_or("Error getting following_count")?.take().as_u64().unwrap() as usize,
        visits: response.get_mut("visits").ok_or("Error getting visits")?.take().as_u64().unwrap(),

        online_status: match response.get_mut("online_status").ok_or("Error getting online_status")?.take().as_str().unwrap() {
            "offline" => OnlineStatus::Offline,
            "online" => OnlineStatus::Online,
            _ => OnlineStatus::Offline
        },
        created_at: DateTime::parse_from_rfc3339(response.get_mut("created_at").ok_or("Error getting created_at")?.take().as_str().unwrap()).unwrap().with_timezone(&Utc),

        is_deleted: response.get_mut("is_deleted").ok_or("Error getting is_deleted")?.take().as_bool().unwrap(),
        is_staff: response.get_mut("is_staff").ok_or("Error getting is_staff")?.take().as_bool().unwrap(),
        is_moderator: response.get_mut("is_moderator").ok_or("Error getting is_moderator")?.take().as_bool().unwrap(),
        is_booster: response.get_mut("is_booster").ok_or("Error getting is_booster")?.take().as_bool().unwrap(),
        is_content_creator: response.get_mut("is_content_creator").ok_or("Error getting is_content_creator")?.take().as_bool().unwrap(),
        
        shirt_id: response.get_mut("shirt_id").ok_or("Error getting shirt_id")?.take().as_u64().unwrap(),
        last_seen: DateTime::parse_from_rfc3339(response.get_mut("last_seen").ok_or("Error getting last_seen")?.take().as_str().unwrap()).unwrap().with_timezone(&Utc)
    })
}

/// Gets the friends list of an user.
/// 
/// Note: this returns a preview of the friend's information. If you want to get every information, use the `view` method of `VortexUserPreview`
pub async fn get_friends_from_id(user_id: impl Into<UserId>) -> Result<Vec<VortexUserPreview>, String> {
    let user_id = user_id.into();

    let mut response: serde_json::Value = reqwest::get(format!("https://playvortex.io/api/friends/{}", user_id.0)).await.or(Err("Failed to get response"))?
        .json().await.or(Err("Failed to cast response into a json type"))?;

    Ok(response.as_array_mut().unwrap().into_iter().map(|data| {
        let mut data = data.take();

        let friend_id = data.get("id").unwrap().as_u64().unwrap().into();
        let username = data.get_mut("username").take().unwrap().as_str().unwrap().to_string();
        let online_status = &data.get_mut("online_status").take().unwrap().as_str().unwrap().to_string();
        let online_status = match online_status.as_str() {
            "online" => OnlineStatus::Online,
            _ => OnlineStatus::Offline,
        };

        VortexUserPreview {
            id: friend_id,
            username,
            online_status
        }
    }).collect::<Vec<_>>())
}