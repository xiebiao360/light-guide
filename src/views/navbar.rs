use crate::{components::Dialog, Route};
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
    let mut show_dialog = use_signal(|| false);
    let mut workspace_type = use_signal(|| String::new());
    let mut workspace_name = use_signal(|| String::new());
    let mut workspaces = use_signal(|| {
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
                div {
                    display: "flex",
                    justify_content: "space-between",
                    select {
                        class: "workspace-select",
                        for workspace in workspaces.read().iter() {
                            option { key: "{workspace}", value: "{workspace}", "{workspace}" }
                        }
                    }
                    button {
                        class: "btn",
                        width: "72px",
                        margin_left: "10px",
                        onclick: move |_| {
                            show_dialog.set(true);
                        },
                        "新建"
                    }
                }
            }

            Dialog {
                show: show_dialog(),
                title: "新建工作区",
                onclose: move |_| {
                    show_dialog.set(false);
                },
                onconfirm: move |_| {
                    // Handle confirm action
                    let new_workspace = format!("{} - {}", workspace_type(), workspace_name());
                    workspaces.set({
                        let mut ws = workspaces.read().clone();
                        ws.push(new_workspace);
                        ws
                    });
                },
                validate: move |_| {
                    let mut errors = Vec::new();
                    if workspace_name().is_empty() {
                        errors.push("工作区名称不能为空".to_string());
                    }
                    if workspace_type().is_empty() {
                        errors.push("工作区类型不能为空".to_string());
                    }
                    if errors.is_empty() {
                        None
                    } else {
                        Some(errors)
                    }
                },
                div {
                    div {
                        class: "form-group",
                        label { "工作区类型" }
                        select {
                            class: "form-select",
                            onchange: move |e| {
                                workspace_type.set(e.value());
                            },
                            option { value: "type1", "类型1" }
                            option { value: "type2", "类型2" }
                        }
                    }
                    div {
                        class: "form-group",
                        label { "工作区名称" }
                        input {
                            class: "form-input",
                            placeholder: "请输入工作区名称",
                            oninput: move |e| {
                                workspace_name.set(e.value());
                            },
                        }
                    }
                }
                // div {
                //     class: "dialog-footer",
                //     button {
                //         class: "btn primary",
                //         onclick: move |_| {
                //             // 处理新建工作区逻辑
                //             let new_workspace = format!("{} - {}", workspace_type(), workspace_name());
                //             workspaces.set({
                //                 let mut ws = workspaces.read().clone();
                //                 ws.push(new_workspace);
                //                 ws
                //             });
                //             show_dialog.set(false);
                //         },
                //         "确定"
                //     }
                //     button {
                //         class: "btn",
                //         onclick: move |_| {
                //             show_dialog.set(false);
                //         },
                //         "取消"
                //     }
                // }
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
