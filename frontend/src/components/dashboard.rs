use leptos::*;
use crate::api::{api_get_users, api_delete_user, clear_token};

#[component]
pub fn Dashboard(
    set_page: WriteSignal<i32>,
) -> impl IntoView {
    let (users, set_users) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(String::new());

    let load_users = move || {
        set_loading.set(true);
        set_error.set(String::new());
        spawn_local(async move {
            match api_get_users().await {
                Ok(data) => set_users.set(data),
                Err(e) => set_error.set(e),
            }
            set_loading.set(false);
        });
    };

    load_users();

    let delete_user = move |user_id: i64| {
        let load_users = load_users.clone();
        let set_error = set_error.clone();
        spawn_local(async move {
            match api_delete_user(user_id).await {
                Ok(_) => load_users(),
                Err(e) => set_error.set(e),
            }
        });
    };

    let logout = move |_| {
        clear_token();
        set_page.set(0);
    };

    view! {
        <div class="dashboard">
            <nav class="navbar">
                <div class="navbar-brand">
                    <h1>用户管理系统</h1>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <a class="navbar-item active" href="#">
                            <span class="icon">&#128101;</span>
                            <span>用户管理</span>
                        </a>
                        <a class="navbar-item" href="#">
                            <span class="icon">&#128202;</span>
                            <span>数据统计</span>
                        </a>
                        <a class="navbar-item" href="#" on:click=move |_| set_page.set(3)>
                            <span class="icon">&#9881;&#65039;</span>
                            <span>系统设置</span>
                        </a>
                    </div>
                    <div class="navbar-end">
                        <div class="navbar-item">
                            <span class="user-info">欢迎使用系统</span>
                        </div>
                        <div class="navbar-item">
                            <button class="logout-btn" on:click=logout>"退出登录"</button>
                        </div>
                    </div>
                </div>
            </nav>
            <div class="content">
                <div class="breadcrumb">
                    <span>首页</span>
                    <span class="separator">/</span>
                    <span class="current">用户管理</span>
                </div>
                <h2>用户列表</h2>
                <p class="error-text">{error}</p>

                <Show
                    when=move || loading.get()
                    fallback=|| view! {}
                >
                    <p class="loading-text">"加载中..."</p>
                </Show>

                <Show
                    when=move || !loading.get() && users.get().is_empty()
                    fallback=|| view! {}
                >
                    <p class="empty-text">"暂无用户"</p>
                </Show>

                <Show
                    when=move || !loading.get() && !users.get().is_empty()
                    fallback=|| view! {}
                >
                    <table class="user-table">
                        <thead>
                            <tr>
                                <th>ID</th>
                                <th>用户名</th>
                                <th>邮箱</th>
                                <th>操作</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || users.get().iter().map(|user| {
                                let user_id = user.id;
                                let delete_user = delete_user.clone();
                                view! {
                                    <tr key=user_id>
                                        <td>{user.id}</td>
                                        <td>{user.username.clone()}</td>
                                        <td>{user.email.clone()}</td>
                                        <td>
                                            <button class="delete-btn" on:click=move |_| delete_user(user_id)>"删除"</button>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </Show>
            </div>
        </div>
    }
}