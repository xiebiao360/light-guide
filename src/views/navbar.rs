use crate::components::dialog::Dialog;
use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Left {}
            Content {}
        }
    }
}

#[component]
pub fn Left() -> Element {
    let show_dialog = use_signal(|| false);
    let workspace_type = use_signal(|| "");
    let workspace_name = use_signal(|| String::new());
    let workspaces = use_signal(|| {
        vec![
            "工作区1".to_string(),
            "工作区2".to_string(),
            "工作区3".to_string(),
        ]
    });

    rsx! {
        div {
            id: "left",
            // 工作区切换
            div {
                id: "workspace-switcher",
                select {
                    class: "workspace-select",
                    onchange: move |e| {
                        if e.value() == "new" {
                            to_owned![show_dialog];
                            show_dialog.set(true);
                        }
                    },
                    for workspace in workspaces.read().iter() {
                        option { key: "{workspace}", value: "{workspace}", "{workspace}" }
                    }
                    option { value: "new", "新建工作区" }
                }
            }

            // 新建工作区对话框
            Dialog {
                show: show_dialog(),
                on_close: move |_| {
                    to_owned![show_dialog];
                    show_dialog.set(false)
                },
                div {
                    class: "dialog-content",
                    h2 { class: "dialog-title", "新建工作区" }

                    div {
                        class: "dialog-options",
                        div {
                            class: "dialog-option",
                            onclick: move |_| {
                                to_owned![workspace_type];
                                workspace_type.set("local")
                            },
                            "从本地环境创建"
                        }
                        div {
                            class: "dialog-option",
                            onclick: move |_| {
                                to_owned![workspace_type];
                                workspace_type.set("docker")
                            },
                            "从Docker环境创建"
                        }
                        div {
                            class: "dialog-option",
                            onclick: move |_| {
                                to_owned![workspace_type];
                                workspace_type.set("kubernetes")
                            },
                            "从Kubernetes环境创建"
                        }
                    }

                    input {
                        class: "dialog-input",
                        placeholder: "输入工作区名称",
                        oninput: move |e| {
                            to_owned![workspace_name];
                            workspace_name.set(e.value());
                        },
                    }

                    div {
                        class: "dialog-buttons",
                        button {
                            class: "dialog-button secondary",
                            onclick: move |_| {
                                to_owned![show_dialog];
                                show_dialog.set(false)
                            },
                            "取消"
                        }
                        button {
                            class: "dialog-button primary",
                            onclick: move |_| {
                                if !workspace_name().is_empty() {
                                    to_owned![show_dialog, workspace_name, workspaces, workspace_type];
                                    let name = workspace_name().to_string();
                                    workspaces.write().push(name);
                                    show_dialog.set(false);
                                    workspace_name.set(String::new());
                                    workspace_type.set("");
                                }
                            },
                            "确认"
                        }
                    }
                }
            }

            // 主菜单
            nav {
                id: "main-menu",
                ul {
                    class: "menu-list",
                    li {
                        class: "menu-item active",
                        "首页"
                    }
                    li {
                        class: "menu-item",
                        "应用"
                    }
                    li {
                        class: "menu-item",
                        "日志"
                    }
                    li {
                        class: "menu-item",
                        "设置"
                    }
                }
            }

            // 用户信息
            div {
                id: "user-profile",
                div {
                    class: "avatar",
                    "头像"
                }
                div {
                    class: "username",
                    "用户名"
                }
            }
        }
    }
}

#[component]
pub fn Content() -> Element {
    rsx! {
        div {
            id: "content",
            Header {}
            MainContent {}
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            // 面包屑导航
            div {
                class: "breadcrumbs",
                span { "首页" }
                span { ">" }
                span { "控制台" }
            }

            // 操作按钮区
            div {
                class: "action-buttons",
                button {
                    class: "btn primary",
                    "新建"
                }
                button {
                    class: "btn",
                    "刷新"
                }
            }
        }
    }
}

#[component]
pub fn MainContent() -> Element {
    rsx! {
        div {
            id: "main-content",
            // The Outlet component will render the current route's component
            Outlet::<Route> {}
        }
    }
}
