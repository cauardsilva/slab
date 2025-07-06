use std::{env, sync::OnceLock};

use iced::{
    Element, Task,
    widget::{Column, button, column, pick_list, row, scrollable, text, text_input},
};
use slab::{
    Channel, User, UserMessage, Workspace, fetch_all_channel_user_messages,
    fetch_all_user_workspaces, fetch_all_workspace_channels, fetch_same_channel_users,
    fetch_user_by_login, get_channels_storage_usage, send_channel_user_message,
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

static POSTGRES_CONNECTION_POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to initialize dotenv");

    let postgres_connection_string =
        env::var("DATABASE_URL").expect("Failed to get postgres connection string from env");

    let postgres_connection_pool = PgPoolOptions::new()
        .connect(&postgres_connection_string)
        .await
        .expect("Failed to create postgres connection pool");

    POSTGRES_CONNECTION_POOL
        .set(postgres_connection_pool)
        .unwrap();

    iced::run("Slab", Slab::update, Slab::render).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateSelectedWorkspace(Workspace),
    UpdateSelectedChannel(Channel),
    TryFetchUserByLogin,
    FetchAllWorkspaces(Option<User>),
    FetchAllWorkspacesResult(Vec<Workspace>),
    UpdateSelectedWorkspaceResult(Vec<Channel>),
    FetchAllChannelUserMessages(()),
    FetchAllChannelUserMessagesResult(Vec<UserMessage>),
    UpdateMessage(String),
    SendMessage,
    Logout,

    // query testing page messages
    NavigateToQueryPage,
    FetchChannelStorageUsage,
    FetchSameChannels(String),
    SetQueryResult(String),
    SetQueryInput(String),
}

#[derive(Default)]
enum Page {
    #[default]
    LoginPage,
    HomePage,
    QueryTestingPage,
}

#[derive(Default)]
struct Slab {
    user_name: String,
    password: String,
    user_message_input: String,

    selected_user: Option<User>,
    workspaces: Vec<Workspace>,
    selected_workspace: Option<Workspace>,
    channels: Vec<Channel>,
    selected_channel: Option<Channel>,
    user_messages: Vec<UserMessage>,

    current_page: Page,
    query_result: Option<String>,
    query_input: String,
}

impl Slab {
    fn render(&self) -> Element<Message> {
        let screen = match self.current_page {
            Page::LoginPage => column![
                text("Login").size(24),
                text_input("Username", &self.user_name)
                    .on_input(Message::UpdateUsername)
                    .padding(10),
                text_input("Password", &self.password)
                    .on_input(Message::UpdatePassword)
                    .padding(10)
                    .secure(true),
                button("Login")
                    .on_press(Message::TryFetchUserByLogin)
                    .padding(10),
                button("Query Testing")
                    .on_press(Message::NavigateToQueryPage)
                    .padding(10),
            ]
            .spacing(15)
            .padding(30)
            .align_x(iced::Alignment::Center),

            Page::HomePage => {
                let selectors = row![
                    button("Logout").on_press(Message::Logout),
                    pick_list(
                        self.workspaces.clone(),
                        self.selected_workspace.clone(),
                        Message::UpdateSelectedWorkspace,
                    )
                    .placeholder("Workspace")
                    .width(iced::Length::Fill),
                    pick_list(
                        self.channels.clone(),
                        self.selected_channel.clone(),
                        Message::UpdateSelectedChannel,
                    )
                    .placeholder("Channel")
                    .width(iced::Fill),
                ]
                .spacing(10);

                let message_history = scrollable(
                    Column::with_children(
                        self.user_messages
                            .iter()
                            .map(|msg| text(format!("{}: {}", msg.user_name, msg.content)).into())
                            .collect::<Vec<_>>(),
                    )
                    .spacing(5),
                )
                .height(iced::Length::Fill);

                let message_input = row![
                    text_input("Type a message...", &self.user_message_input)
                        .on_input(Message::UpdateMessage)
                        .padding(10),
                    button(text("Send").size(16))
                        .on_press(Message::SendMessage)
                        .padding(10),
                ]
                .spacing(10)
                .align_y(iced::Center);

                column![selectors, message_history, message_input]
                    .spacing(15)
                    .padding(10)
            }
            Page::QueryTestingPage => {
                let query_result = match &self.query_result {
                    Some(result) => result.clone(),
                    None => "".to_string(),
                };

                column![
                    row![
                        button(text("tamanho dos anexos de cada canal").size(16))
                            .on_press(Message::FetchChannelStorageUsage)
                            .padding(10),
                        button(text("usuarios que estao no canal de @input").size(16))
                            .on_press(Message::FetchSameChannels(self.query_input.clone()))
                            .padding(10)
                    ]
                    .spacing(15),
                    row![
                        text_input("Enter query input", &self.query_input)
                            .on_input(Message::SetQueryInput)
                            .padding(10),
                    ],
                    text!("{}", query_result)
                ]
                .spacing(15)
                .padding(15)
            }
        };

        iced::widget::container(screen)
            .width(iced::Fill)
            .height(iced::Fill)
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateUsername(value) => {
                self.user_name = value;
                Task::none()
            }
            Message::UpdatePassword(value) => {
                self.password = value;
                Task::none()
            }
            Message::TryFetchUserByLogin => Task::perform(
                fetch_user_by_login(
                    POSTGRES_CONNECTION_POOL.get().unwrap(),
                    self.user_name.clone(),
                    self.password.clone(),
                ),
                Message::FetchAllWorkspaces,
            ),
            Message::FetchAllWorkspaces(user) => {
                if user.is_none() {
                    self.user_name = "Wrong credentials!".to_string();
                    self.password = "".to_string();
                    return Task::none();
                }

                self.selected_user = user;
                self.current_page = Page::HomePage;

                Task::perform(
                    fetch_all_user_workspaces(
                        POSTGRES_CONNECTION_POOL.get().unwrap(),
                        self.user_name.clone(),
                    ),
                    Message::FetchAllWorkspacesResult,
                )
            }
            Message::FetchAllWorkspacesResult(workspaces) => {
                self.workspaces = workspaces;
                self.channels = vec![];
                self.user_messages = vec![];
                Task::none()
            }
            Message::Logout => {
                self.selected_user = None;
                self.user_name = "".to_string();
                self.password = "".to_string();
                self.selected_channel = None;
                self.selected_workspace = None;
                self.current_page = Page::LoginPage;
                Task::none()
            }
            Message::UpdateSelectedWorkspace(value) => {
                self.selected_workspace = Some(value);
                self.selected_channel = None;
                self.channels = vec![];
                self.user_messages = vec![];
                self.user_message_input = "".to_string();

                Task::perform(
                    fetch_all_workspace_channels(
                        POSTGRES_CONNECTION_POOL.get().unwrap(),
                        self.selected_workspace.clone().unwrap().workspace_id,
                    ),
                    Message::UpdateSelectedWorkspaceResult,
                )
            }
            Message::UpdateSelectedWorkspaceResult(channels) => {
                self.channels = channels;
                Task::none()
            }
            Message::UpdateSelectedChannel(channel) => {
                self.selected_channel = Some(channel);
                self.user_messages = vec![];
                Task::done(Message::FetchAllChannelUserMessages(()))
            }
            Message::FetchAllChannelUserMessages(()) => {
                self.user_message_input = "".to_string();
                Task::perform(
                    fetch_all_channel_user_messages(
                        POSTGRES_CONNECTION_POOL.get().unwrap(),
                        self.selected_channel.clone().unwrap().channel_id,
                    ),
                    Message::FetchAllChannelUserMessagesResult,
                )
            }
            Message::FetchAllChannelUserMessagesResult(user_messages) => {
                self.user_messages = user_messages;
                Task::none()
            }
            Message::UpdateMessage(value) => {
                self.user_message_input = value;
                Task::none()
            }
            Message::SendMessage => {
                if self.selected_user.is_none() || self.selected_channel.is_none() {
                    return Task::none();
                }

                Task::perform(
                    send_channel_user_message(
                        POSTGRES_CONNECTION_POOL.get().unwrap(),
                        self.selected_user.clone().unwrap().user_id,
                        self.selected_channel.clone().unwrap().channel_id,
                        self.user_message_input.clone(),
                    ),
                    Message::FetchAllChannelUserMessages,
                )
            }
            Message::NavigateToQueryPage => {
                self.current_page = Page::QueryTestingPage;
                Task::none()
            }
            Message::FetchChannelStorageUsage => Task::perform(
                get_channels_storage_usage(POSTGRES_CONNECTION_POOL.get().unwrap()),
                |storage_usage| Message::SetQueryResult(storage_usage.join("\n")),
            ),
            Message::SetQueryResult(result) => {
                self.query_result = Some(result);
                Task::none()
            }
            Message::FetchSameChannels(user_id) => Task::perform(
                fetch_same_channel_users(POSTGRES_CONNECTION_POOL.get().unwrap(), user_id),
                |output| Message::SetQueryResult(output.join(", ")),
            ),
            Message::SetQueryInput(input) => {
                self.query_input = input;
                Task::none()
            }
        }
    }
}
