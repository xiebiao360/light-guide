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
    rsx! {
        div {
            id: "left",
            // 工作区切换
            div {
                id: "workspace-switcher",
                select {
                    class: "workspace-select",
                    option { "工作区1" }
                    option { "工作区2" }
                    option { "工作区3" }
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
                        "博客"
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
