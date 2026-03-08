mod config;
mod containers;
mod utils;

use config::{AppConfig, ConfigManager};
use containers::{ContainerInfo, DockerClient};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Application, Background, Border, Color, Command, Element, Length, Padding, Settings, Subscription, Theme};

const BACKGROUND: Color = Color::from_rgb(0.08, 0.08, 0.10);
const CARD: Color = Color::from_rgb(0.15, 0.15, 0.18);
const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.95);
const TEXT_SECONDARY: Color = Color::from_rgb(0.65, 0.65, 0.70);
const RUNNING_GREEN: Color = Color::from_rgb(0.30, 0.75, 0.45);
const STOPPED_RED: Color = Color::from_rgb(0.80, 0.30, 0.30);

#[derive(Debug, Clone)]
pub enum Message {
    ContainersLoaded(Result<Vec<ContainerInfo>, String>),
    Refresh,
    StartContainer(String),
    StopContainer(String),
    RestartContainer(String),
    ContainerAction(Result<(), String>),
    SearchChanged(String),
    Tick,
    ViewLogs(String),
    LogsLoaded(Result<(String, String), String>),
    BackToList,
}

#[derive(Debug, Clone, Default)]
pub enum Screen {
    #[default]
    ContainerList,
    Logs {
        container_id: String,
        container_name: String,
        logs: String,
        loading: bool,
    },
}

pub struct ContainerTyrant {
    containers: Vec<ContainerInfo>,
    loading: bool,
    error: Option<String>,
    search_query: String,
    config: AppConfig,
    _config_manager: ConfigManager,
    screen: Screen,
}

impl Application for ContainerTyrant {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let config_manager = ConfigManager::new().unwrap_or_else(|_| {
            panic!("Failed to initialize config manager")
        });
        let config = config_manager.load().unwrap_or_default();

        (
            Self {
                containers: Vec::new(),
                loading: true,
                error: None,
                search_query: String::new(),
                config,
                _config_manager: config_manager,
                screen: Screen::default(),
            },
            Command::perform(
                async {
                    DockerClient::new().await
                        .map_err(|e| e.to_string())?
                        .list_containers(true).await
                        .map_err(|e| e.to_string())
                },
                Message::ContainersLoaded,
            ),
        )
    }

    fn title(&self) -> String {
        "ContainerTyrant".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ContainersLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(containers) => {
                        self.containers = containers;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                Command::none()
            }
            Message::Refresh | Message::Tick => {
                self.loading = true;
                Command::perform(
                    async {
                        DockerClient::new().await
                            .map_err(|e| e.to_string())?
                            .list_containers(true).await
                            .map_err(|e| e.to_string())
                    },
                    Message::ContainersLoaded,
                )
            }
            Message::StartContainer(id) => {
                let id_clone = id.clone();
                Command::perform(
                    async move {
                        DockerClient::new().await
                            .map_err(|e| e.to_string())?
                            .start_container(&id_clone).await
                            .map_err(|e| e.to_string())
                    },
                    Message::ContainerAction,
                )
            }
            Message::StopContainer(id) => {
                let id_clone = id.clone();
                Command::perform(
                    async move {
                        DockerClient::new().await
                            .map_err(|e| e.to_string())?
                            .stop_container(&id_clone).await
                            .map_err(|e| e.to_string())
                    },
                    Message::ContainerAction,
                )
            }
            Message::RestartContainer(id) => {
                let id_clone = id.clone();
                Command::perform(
                    async move {
                        DockerClient::new().await
                            .map_err(|e| e.to_string())?
                            .restart_container(&id_clone).await
                            .map_err(|e| e.to_string())
                    },
                    Message::ContainerAction,
                )
            }
            Message::ContainerAction(result) => {
                if let Err(e) = result {
                    self.error = Some(e);
                }
                Command::none()
            }
            Message::SearchChanged(query) => {
                self.search_query = query;
                Command::none()
            }
            Message::ViewLogs(id) => {
                let id_clone = id.clone();
                let name = self.containers.iter()
                    .find(|c| c.id == id)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| id.clone());
                
                self.screen = Screen::Logs {
                    container_id: id,
                    container_name: name.clone(),
                    logs: String::new(),
                    loading: true,
                };
                
                Command::perform(
                    async move {
                        let client = DockerClient::new().await
                            .map_err(|e| e.to_string())?;
                        let logs = client.get_container_logs(&id_clone, 500).await
                            .map_err(|e| e.to_string())?;
                        Ok((id_clone, logs))
                    },
                    Message::LogsLoaded,
                )
            }
            Message::LogsLoaded(result) => {
                match result {
                    Ok((id, logs)) => {
                        if let Screen::Logs { container_id, .. } = &mut self.screen {
                            if *container_id == id {
                                self.screen = Screen::Logs {
                                    container_id: id,
                                    container_name: String::new(),
                                    logs,
                                    loading: false,
                                };
                            }
                        }
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                Command::none()
            }
            Message::BackToList => {
                self.screen = Screen::ContainerList;
                self.error = None;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(self.config.refresh_interval_ms))
            .map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::ContainerList => self.view_container_list(),
            Screen::Logs { container_name, logs, loading, .. } => self.view_logs(container_name, logs, *loading),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl ContainerTyrant {
    fn view_container_list(&self) -> Element<Message> {
        let title = text("ContainerTyrant")
            .size(28)
            .style(Color::from_rgb(0.95, 0.95, 0.95));

        let subtitle = text("Rule your containers with an iron fist")
            .size(14)
            .style(TEXT_SECONDARY);

        let header = container(column![title, subtitle].spacing(4).align_items(Alignment::Start))
            .padding(Padding::new(20.0));

        let search_input = text_input("Search containers...", &self.search_query)
            .on_input(Message::SearchChanged)
            .padding(10)
            .width(Length::Fill);

        let refresh_btn = button(text("Refresh").size(14))
            .on_press(Message::Refresh)
            .padding([8, 16]);

        let toolbar = row![search_input, Space::with_width(10), refresh_btn]
            .align_items(Alignment::Center)
            .spacing(10);

        let content: Element<Message> = if self.loading {
            container(
                text("Loading containers...")
                    .size(18)
                    .style(TEXT_SECONDARY),
            )
            .padding(40)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else if let Some(e) = &self.error {
            container(
                column![
                    text("Error").size(20).style(STOPPED_RED),
                    text(e).size(14).style(TEXT_SECONDARY),
                    button("Retry").on_press(Message::Refresh)
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .padding(40)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            let filtered: Vec<_> = self
                .containers
                .iter()
                .filter(|c| {
                    self.search_query.is_empty()
                        || c.name.to_lowercase().contains(&self.search_query.to_lowercase())
                        || c.image.to_lowercase().contains(&self.search_query.to_lowercase())
                })
                .collect();

            if filtered.is_empty() {
                container(
                    text("No containers found")
                        .size(16)
                        .style(TEXT_SECONDARY),
                )
                .padding(30)
                .center_x()
                .width(Length::Fill)
                .into()
            } else {
                let mut items: Vec<Element<Message>> = Vec::new();
                for c in filtered {
                    items.push(container_card(c));
                }
                scrollable(column(items)).into()
            }
        };

        let main_content = column![header, toolbar, Space::with_height(10), content]
            .spacing(10)
            .padding([10, 20, 20, 20]);

        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(container_style)
            .into()
    }
    
    fn view_logs(&self, container_name: &str, logs: &str, loading: bool) -> Element<Message> {
        let back_btn = button(text("← Back").size(14))
            .on_press(Message::BackToList)
            .padding([8, 16]);
        
        let title = text(format!("Logs: {}", container_name))
            .size(22)
            .style(TEXT_PRIMARY);
        
        let header = row![back_btn, Space::with_width(20), title]
            .align_items(Alignment::Center);
        
        let content: Element<Message> = if loading {
            container(
                text("Loading logs...")
                    .size(16)
                    .style(TEXT_SECONDARY),
            )
            .padding(40)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else if logs.is_empty() {
            container(
                text("No logs available")
                    .size(16)
                    .style(TEXT_SECONDARY),
            )
            .padding(40)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            let log_text = text(logs)
                .size(12)
                .style(Color::from_rgb(0.85, 0.85, 0.85));
            
            container(scrollable(log_text))
                .padding(15)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(logs_style)
                .into()
        };
        
        let main_content = column![header, Space::with_height(15), content]
            .spacing(10)
            .padding([15, 20, 20, 20]);
        
        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(container_style)
            .into()
    }
}

fn container_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(BACKGROUND)),
        ..Default::default()
    }
}

fn card_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(CARD)),
        border: Border {
            color: Color::from_rgb(0.22, 0.22, 0.25),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

fn logs_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.10, 0.10, 0.12))),
        border: Border {
            color: Color::from_rgb(0.20, 0.20, 0.22),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

fn status_badge_style(running: bool) -> impl Fn(&Theme) -> container::Appearance {
    move |_: &Theme| container::Appearance {
        background: Some(Background::Color(if running { RUNNING_GREEN } else { STOPPED_RED })),
        text_color: Some(Color::WHITE),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn container_card(info: &ContainerInfo) -> Element<Message> {
    let status_badge = container(text(info.status.as_str()).size(11))
        .padding([4, 8])
        .style(status_badge_style(info.is_running));

    let name = text(&info.name)
        .size(15)
        .style(TEXT_PRIMARY);

    let image = text(&info.image)
        .size(12)
        .style(TEXT_SECONDARY);

    let id_text = text(&info.short_id)
        .size(11)
        .style(Color::from_rgb(0.5, 0.5, 0.55));

    let info_row = row![
        column![name, image].spacing(2),
        Space::with_width(Length::Fill),
        id_text,
        Space::with_width(8),
        status_badge
    ]
    .align_items(Alignment::Center);

    let start_btn = button(text("Start").size(12))
        .on_press(Message::StartContainer(info.id.clone()))
        .padding([6, 12]);

    let stop_btn = button(text("Stop").size(12))
        .on_press(Message::StopContainer(info.id.clone()))
        .padding([6, 12]);

    let restart_btn = button(text("Restart").size(12))
        .on_press(Message::RestartContainer(info.id.clone()))
        .padding([6, 12]);

    let logs_btn = button(text("Logs").size(12))
        .on_press(Message::ViewLogs(info.id.clone()))
        .padding([6, 12]);

    let actions = row![start_btn, stop_btn, restart_btn, logs_btn]
        .spacing(6)
        .align_items(Alignment::Center);

    container(column![info_row, Space::with_height(8), actions].spacing(4))
        .padding(14)
        .style(card_style)
        .into()
}

fn main() -> iced::Result {
    ContainerTyrant::run(Settings {
        window: iced::window::Settings {
            size: iced::Size { width: 900.0, height: 700.0 },
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
