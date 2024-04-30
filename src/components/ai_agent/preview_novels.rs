use leptos::*;

use crate::{actix_end::novel_api::get_novel_list, app_models::novel::Novel};
/// https://play.tailwindcss.com/f05RWoJofd
#[component]
pub fn PreviewNovels() -> impl IntoView {

    let novels_resource = create_resource(
        ||(), 
        |_| async move {
            get_novel_list().await
        }
    );
    let card_views = move || -> Option<Result<View, _>> {
        novels_resource.and_then(|novels| {
            novels
                .into_iter()
                .map(|novel| {
                    view! {
                        <NovelPreviewCard novel={novel.clone()} />
                    }
                })
                .collect_view()
        })
    }; 
    view! {
        <div class="flex h-screen flex-col">
            // <!-- 链接部分 -->
            <div class="flex items-center justify-center bg-gray-100 p-4 dark:bg-gray-900">
              <a href="/edit_novel/" class="text-lg font-semibold text-blue-500 hover:text-blue-700">新建</a>
            </div>
            // <!-- 卡片容器 -->
            <div class="flex flex-wrap items-start justify-center gap-4 overflow-auto p-10">
                <Suspense fallback=move || view! { <p> "Loading" </p>}>
                    <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p>}>
                        {card_views}
                    </ErrorBoundary>
                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn NovelPreviewCard(novel: Novel) -> impl IntoView {
    view! {
        <a href={format!("/edit_novel/{}", novel.novel_name)}>
            <div class="w-64 shadow-lg transition duration-300 hover:scale-105">
                <img class="h-40 w-full object-cover" src={novel.image_url} alt="卡片图片" />
                <div class="flex flex-col bg-white p-4 dark:bg-gray-800">
                //   <!-- 小说名字 -->
                  <h2 class="mb-2 text-xl font-semibold text-gray-900 dark:text-white">{novel.novel_name}</h2>
                //   <!-- 小说描述 -->
                  <p class="text-gray-700 dark:text-gray-200">{novel.description}</p>
                </div>
              </div>
        </a>
    }
}