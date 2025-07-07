use sqlx::{Error, Pool, Postgres};
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

pub async fn create_workspace(
    postgres_connection_pool: &Pool<Postgres>,
    workspace_name: String,
    owner_user_id: Uuid,
) -> Result<(), String> {
    let workspace_id = Uuid::new_v4();

    if let Err(Error::Database(_)) = sqlx::query!(
        r#"
        INSERT INTO postgres.public.Workspaces
        VALUES ($1, $2, $3, NOW())"#,
        workspace_id,
        workspace_name,
        owner_user_id
    )
    .execute(postgres_connection_pool)
    .await
    {
        return Err("Exceeded max created workspaces limit".to_string());
    }

    sqlx::query!(
        r#"
        INSERT INTO postgres.public.WorkspaceMemberships
        VALUES ($1, $2)"#,
        workspace_id,
        owner_user_id
    )
    .execute(postgres_connection_pool)
    .await
    .unwrap();

    Ok(())
}

pub async fn create_channel(
    postgres_connection_pool: &Pool<Postgres>,
    channel_name: String,
    workspace_id: Uuid,
    is_public: bool,
    owner_user_id: Uuid,
) {
    let channel_id = Uuid::new_v4();
    let channel_type = match is_public {
        true => "public",
        false => "private",
    };

    sqlx::query!(
        r#"
        INSERT INTO postgres.public.Channels
        VALUES ($1, $2, $3, $4, NOW())"#,
        channel_id,
        workspace_id,
        channel_type,
        channel_name,
    )
    .execute(postgres_connection_pool)
    .await
    .unwrap();

    sqlx::query!(
        r#"
        INSERT INTO postgres.public.ChannelMemberships
        VALUES ($1, $2)"#,
        channel_id,
        owner_user_id
    )
    .execute(postgres_connection_pool)
    .await
    .unwrap();
}

pub async fn fetch_all_workspaces_revenue_by_subscription_type(
    postgres_connection_pool: &Pool<Postgres>,
) -> String {
    let revenues = sqlx::query!(
        r#"
        SELECT
            st.SubscriptionTypeName as subscription_type_name,
            WorkspaceName AS workspace_name,
            SUM(st.Price) AS subscription_type_revenue,
            st.Duration AS subscription_duration
        FROM Users u
            JOIN WorkspaceMemberships wkm ON u.UserId = wkm.UserId
            JOIN SubscriptionTypes st ON u.SubscriptionTypeId = st.SubscriptionTypeId
            JOIN Workspaces USING (WorkspaceId)
            WHERE st.Valid = true
            GROUP BY st.SubscriptionTypeId, WorkspaceName
            ORDER BY workspace_name ASC, subscription_type_name ASC, subscription_type_revenue DESC, st.Duration ASC"#
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap();

    revenues
        .iter()
        .map(|value| {
            format!(
                "{}: R${:.2} every {} days with {} subscription",
                value.workspace_name,
                value.subscription_type_revenue.clone().unwrap_or_default(),
                value.subscription_duration.months * 30 + value.subscription_duration.days,
                value.subscription_type_name
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub async fn fetch_users_in_channel_with_at_least_one_message(
    postgres_connection_pool: &Pool<Postgres>,
    channel_name: String,
) -> String {
    sqlx::query!(
        r#"
        SELECT c.ChannelName AS channel_name, u.UserName AS user_name FROM Users u
        JOIN Messages m on m.SenderUserId = u.UserId
        JOIN Channels c on c.ChannelId = m.ChannelId
        WHERE c.ChannelName = $1
        GROUP BY u.UserId, c.ChannelId HAVING COUNT(distinct m.MessageId) > 0"#,
        channel_name
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|value| value.user_name)
    .collect::<Vec<String>>()
    .join("\n")
}

pub async fn fetch_users_with_subscription_not_in_any_workspace(
    postgres_connection_pool: &Pool<Postgres>,
) -> String {
    sqlx::query!(
        r#"
        SELECT u.UserName AS user_name, st.SubscriptionTypeName AS subscription_type_name
        FROM Users u
            JOIN SubscriptionTypes st ON u.SubscriptionTypeId = st.SubscriptionTypeId
            WHERE st.Price > 0
                AND NOT EXISTS (
                    SELECT 1
                    FROM WorkspaceMemberships wkm
                        WHERE wkm.UserId = u.UserId
                )
            ORDER BY u.UserName"#
    )
    .fetch_all(postgres_connection_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|value| {
        format!(
            "{} with {} subscription",
            value.user_name, value.subscription_type_name
        )
    })
    .collect::<Vec<String>>()
    .join("\n")
}
