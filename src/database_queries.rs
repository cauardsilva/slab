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

pub async fn get_channels_storage_usage(postgres_connection_pool: &Pool<Postgres>) -> Vec<String> {
    let rows = sqlx::query!(
        r#"
        SELECT
            c.ChannelName as channel_name,
            SUM(OCTET_LENGTH(att.Content)) AS storage_usage
        FROM Attachments att
            JOIN Messages m ON att.MessageId = m.MessageId
            JOIN Channels c ON m.ChannelId = c.ChannelId
            GROUP BY c.ChannelId
            ORDER BY storage_usage;
        "#,
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap();

    rows.iter()
        .map(|row| {
            format!(
                "{} - {}",
                row.channel_name,
                row.storage_usage.unwrap().to_string()
            )
            .to_string()
        })
        .collect()
}

pub async fn fetch_same_channel_users(
    postgres_connection_pool: &Pool<Postgres>,
    user_name: String,
) -> Vec<String> {
    let rows = sqlx::query!(
        r#"
        SELECT u1.UserName as user_name FROM Users u1
        WHERE NOT EXISTS (
            SELECT 1 FROM Channels c
            JOIN ChannelMemberships cm ON c.ChannelId = cm.ChannelId
            JOIN Users u2 ON u2.UserId = cm.UserId
            WHERE c.ChannelType = 'public'
                AND u2.UserName = $1
                AND NOT EXISTS (
                    SELECT 1 FROM ChannelMemberships cm1
                    WHERE cm.ChannelId = cm1.ChannelId
                        AND cm1.UserId = u1.UserId
                )
        );
        "#,
        user_name
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap();

    rows.iter()
        .map(|row| format!("{}", row.user_name).to_string())
        .collect()
}
