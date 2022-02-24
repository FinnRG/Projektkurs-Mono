use crate::schema::{
    comments, likes, notifications, playlist_to_video, playlists, subscriptions, tags, users,
    videos,
};
use serde::Serialize;

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User)]
pub struct Video {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[table_name = "videos"]
pub struct VideoUpdate<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "videos"]
pub struct NewVideo<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct VideoSuggestion {
    id: String,
    title: String,
    description: String,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User)]
#[belongs_to(Video)]
pub struct Comment {
    pub id: i32,
    pub user_id: String,
    pub video_id: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub user_id: &'a str,
    pub video_id: &'a str,
    pub content: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User)]
#[belongs_to(Video)]
#[primary_key(user_id, video_id)]
pub struct Like {
    pub value: bool,
    pub user_id: String,
    pub video_id: String,
}

#[derive(Insertable)]
#[table_name = "likes"]
pub struct NewLike<'a> {
    pub value: bool,
    pub user_id: &'a str,
    pub video_id: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User, foreign_key = "author")]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub author: String,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub author: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct TagUpdate<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User, foreign_key = "author")]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name = "playlists"]
pub struct NewPlaylist<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub author: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "playlists"]
pub struct PlaylistUpdate<'a> {
    pub title: Option<&'a str>,
}

#[derive(Queryable, Insertable, Debug, Serialize)]
#[table_name = "playlist_to_video"]
pub struct PlaylistToVideo {
    pub id: i32,
    pub video_id: String,
    pub playlist_id: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct PlaylistEntry {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub video_id: String,
}

#[derive(Queryable, Debug, Serialize, Associations)]
#[belongs_to(User)]
#[belongs_to(Tag)]
pub struct Subscription {
    pub user_id: String,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name = "subscriptions"]
pub struct NewSubscription<'a> {
    pub user_id: &'a str,
    pub tag_id: i32,
}

#[derive(Queryable, Debug, Serialize, Associations)]
#[belongs_to(User)]
#[belongs_to(Tag)]
#[belongs_to(Video)]
pub struct Notification {
    pub user_id: String,
    pub tag_id: i32,
    pub video_id: String,
}

#[derive(Insertable)]
#[table_name = "notifications"]
pub struct NewNotification<'a> {
    pub user_id: &'a str,
    pub tag_id: i32,
    pub video_id: &'a str,
}
