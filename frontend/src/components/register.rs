use leptos::*;
use leptos::ev::SubmitEvent;
use crate::api::{RegisterRequest, api_register, set_token, event_target_value};

#[component]
pub fn RegisterForm(
    set_page: WriteSignal<i32>,
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);

    let on_register = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(String::new());

        spawn_local(async move {
            let request = RegisterRequest {
                username: username.get(),
                email: email.get(),
                password: password.get(),
            };

            match api_register(&request).await {
                Ok(resp) => {
                    set_token(resp.token);
                    set_page.set(2);
                }
                Err(e) => {
                    set_error.set(e);
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="login-container">
            <div class="login-box">
                <h2>用户注册</h2>
                <p class="error-text">{error}</p>
                
                <form on:submit=on_register>
                    <div class="form-group">
                        <label for="username">用户名</label>
                        <input
                            type="text"
                            id="username"
                            placeholder="请输入用户名"
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            required
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="email">邮箱</label>
                        <input
                            type="email"
                            id="email"
                            placeholder="请输入邮箱"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            required
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="password">密码</label>
                        <input
                            type="password"
                            id="password"
                            placeholder="请输入密码"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            required
                        />
                    </div>
                    
                    <button type="submit" class="submit-btn" disabled=loading>
                        {move || if loading.get() { "注册中..." } else { "注册" }}
                    </button>
                </form>
                
                <div class="link-text">
                    <span>已有账号?</span>
                    <a href="#" on:click=move |_| set_page.set(0)>立即登录</a>
                </div>
            </div>
        </div>
    }
}