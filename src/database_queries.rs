use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{Channel, User, UserMessage, Workspace};

pub async fn fetch_user_by_login(
    postgres_connection_pool: &Pool<Postgres>,
    user_name: String,
    unhashed_password: String,
) -> Option<User> {
    sqlx::query_as!(
        User,
        r#"
            SELECT UserId AS user_id, UserName AS user_name
            FROM postgres.public.Users
            WHERE UserName    = $1 AND
                  SecretValue = $2 AND
                  SecretType  = 'UnhashedPassword'"#,
        user_name,
        unhashed_password
    )
    .fetch_optional(postgres_connection_pool)
    .await
    .unwrap()
}

// async fn fetch_all_users() -> Vec<User> {
//     sqlx::query_as!(
//         User,
//         r#"
//         SELECT UserName AS user_name
//         FROM postgres.public.Users"#
//     )
//     .fetch_all(POSTGRES_CONNECTION_POOL.get().unwrap())
//     .await
//     .unwrap_or_default()
// }

pub async fn fetch_all_user_workspaces(
    postgres_connection_pool: &Pool<Postgres>,
    user_name: String,
) -> Vec<Workspace> {
    sqlx::query_as!(
        Workspace,
        r#"
            SELECT WorkspaceId AS workspace_id, WorkspaceName AS workspace_name
            FROM postgres.public.Workspaces
            JOIN postgres.public.WorkspaceMemberships USING (WorkspaceId)
            JOIN postgres.public.Users USING (UserId)
            WHERE UserName = $1"#,
        user_name
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap_or_default()
}

pub async fn fetch_all_workspace_channels(
    postgres_connection_pool: &Pool<Postgres>,
    workspace_id: Uuid,
) -> Vec<Channel> {
    sqlx::query_as!(
        Channel,
        r#"
            SELECT ChannelId AS channel_id, ChannelName AS channel_name
            FROM postgres.public.Channels
            WHERE WorkspaceId = $1"#,
        workspace_id
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap_or_default()
}

pub async fn fetch_all_channel_user_messages(
    postgres_connection_pool: &Pool<Postgres>,
    channel_id: Uuid,
) -> Vec<UserMessage> {
    sqlx::query_as!(
        UserMessage,
        r#"
            SELECT u.UserId AS user_id, u.UserName AS user_name, m.Content AS content
            FROM postgres.public.Messages m
            JOIN postgres.public.Users u ON m.SenderUserId = u.UserId
            WHERE m.ChannelId = $1
            ORDER BY m.CreatedAt ASC"#,
        channel_id
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap_or_default()
}

pub async fn send_channel_user_message(
    postgres_connection_pool: &Pool<Postgres>,
    user_id: Uuid,
    channel_id: Uuid,
    content: String,
) {
    sqlx::query!(
        r#"
            INSERT INTO postgres.public.Messages
            VALUES ($1, $2, $3, $4, NOW(), NULL)"#,
        Uuid::new_v4(),
        channel_id,
        user_id,
        content,
    )
    .execute(postgres_connection_pool)
    .await
    .unwrap();
}
