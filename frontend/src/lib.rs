mod api;
mod components;

use api::get_token;
use leptos::*;
use wasm_bindgen::prelude::*;
use components::{login::LoginForm, register::RegisterForm, dashboard::Dashboard, settings::SettingsPage};

// 主应用组件
#[component]
fn App() -> impl IntoView {
    let (page, set_page) = create_signal(if get_token().is_some() { 2 } else { 0 });

    view! {
        <div id="app">
            <Show
                when=move || page.get() == 0
                fallback=|| view! {}
            >
                <LoginForm set_page=set_page />
            </Show>

            <Show
                when=move || page.get() == 1
                fallback=|| view! {}
            >
                <RegisterForm set_page=set_page />
            </Show>

            <Show
                when=move || page.get() == 2
                fallback=|| view! {}
            >
                <Dashboard set_page=set_page />
            </Show>

            <Show
                when=move || page.get() == 3
                fallback=|| view! {}
            >
                <SettingsPage set_page=set_page />
            </Show>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}