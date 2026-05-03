use leptos::*;
use crate::api::clear_token;

#[component]
pub fn SettingsPage(
    set_page: WriteSignal<i32>,
) -> impl IntoView {
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
                        <a class="navbar-item" href="#" on:click=move |_| set_page.set(2)>
                            <span class="icon">&#128101;</span>
                            <span>用户管理</span>
                        </a>
                        <a class="navbar-item" href="#">
                            <span class="icon">&#128202;</span>
                            <span>数据统计</span>
                        </a>
                        <a class="navbar-item active" href="#">
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
                    <span class="current">系统设置</span>
                </div>
                <h2>系统设置</h2>
                
                <div class="settings-container">
                    <div class="settings-section">
                        <h3>外部API接口</h3>
                        <div class="api-list">
                            <div class="api-item">
                                <h4>足球赛事API</h4>
                                <p>获取实时足球比赛信息</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/football/matches?league=premier-league</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>球队信息API</h4>
                                <p>获取球队详细信息和排名</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/football/teams?team=manchester-united</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>球员数据API</h4>
                                <p>获取球员统计数据和表现</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/football/players?player=messi</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>联赛排名API</h4>
                                <p>获取各大联赛积分榜</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/football/standings?season=2024</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>天气API</h4>
                                <p>获取实时天气信息</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/weather?city=北京</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>汇率API</h4>
                                <p>获取实时汇率信息</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/exchange?from=USD&to=CNY</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>IP查询API</h4>
                                <p>查询IP地址地理位置</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/ip-lookup?ip=8.8.8.8</code>
                                </div>
                            </div>
                            
                            <div class="api-item">
                                <h4>新闻API</h4>
                                <p>获取最新新闻资讯</p>
                                <div class="api-details">
                                    <span class="api-method get">GET</span>
                                    <code class="api-url">/api/news?category=technology</code>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="settings-section">
                        <h3>系统配置</h3>
                        <div class="config-list">
                            <div class="config-item">
                                <label>语言设置</label>
                                <select>
                                    <option value="zh">中文</option>
                                    <option value="en">English</option>
                                </select>
                            </div>
                            
                            <div class="config-item">
                                <label>主题设置</label>
                                <select>
                                    <option value="light">浅色主题</option>
                                    <option value="dark">深色主题</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}