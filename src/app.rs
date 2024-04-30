use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::ai_agent::edit_chapter::EditChapter;
use crate::components::ai_agent::edit_novel::EditNovel;
use crate::components::edit_copilot::edit_novel::EditCopilot;
use crate::components::ai_agent::chapters::Chapters;
use crate::components::login::Login;
use crate::components::register::Register;
use crate::components::ai_agent::preview_novels::PreviewNovels;

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <div class="dark:bg-gray-800 dark:text-white  p-4 h-bar">
            <div class="container mx-auto flex justify-between items-center">
                <a href="/" class="text-2xl font-hold"> AI小说写作 </a>
                <nav>
                    <ul class="flex space-x-4">
                        <li><a href="/" class="hover:text-blue-400"> 助手写作 </a></li>
                        <li><a href="/view_novels" class="hover:text-blue-400"> AI-Agent </a> </li>
                        // <li><a href="/login" class="hover:text-blue-400"> 登录 </a> </li>
                    </ul>
                </nav>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/novel-ai.css"/>
        
        <Navbar/>

        // content for this welcome page
        <Router>
            <main class="h-screen-no-bar">
                <Routes>
                    <Route path="" view=EditCopilot/>
                    // <Route path="/agent" view=Chapters/>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/edit_chapter/:novel_name/:chapter_title?" view=EditChapter/>
                    <Route path="/edit_novel/:edit_novel_name?" view=EditNovel/>
                    <Route path="/view_novels" view=PreviewNovels/>
                    <Route path="/view_chapters/:novel_name" view=Chapters/>
                </Routes>
            </main>
        </Router>
       
    }
}

