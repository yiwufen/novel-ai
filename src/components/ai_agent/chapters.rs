use crate::actix_end::chapter_api::get_chapters_api;
use crate::components::ai_agent::preview_chapter::PreviewChapter;
use crate::components::loading::Loading;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditChaptersParams {
    novel_name: String,
}

#[component]
pub fn Chapters() -> impl IntoView {
    let params = use_params::<EditChaptersParams>();
    // let (novel_name, set_novel_name) = create_signal(String::from("云边有个小卖部"));
    let chapter_edit_resouce = create_resource(
        move || params(), 
        |params| async move {
            match params {
                Ok(EditChaptersParams { novel_name }) => get_chapters_api(novel_name).await,
                _ => Ok(vec![])
            }
        }
    );

    let novel_name = create_memo(move |_| {
        if let Ok(tmp) = params() {
            tmp.novel_name
        } else {
            "".into()
        }
    });

    let (selected_chapter, set_selected_chapter) = create_signal(String::new()); 

    let chapters_view = move || {
        chapter_edit_resouce.get().and_then(|res| res.map(|chapters| {
            chapters.iter().map(|chapter| {
                view! {
                    <li class="border-b border-blue-300 p-2" ><input type="button" class="hover:text-blue-400" value=chapter.chapter_title.clone() on:click={move |ev| set_selected_chapter.set(event_target_value(&ev)) } /></li>
                }
            }).collect_view()
        }).ok())
    };
    // create_effect(move |_| {
    //     log!("selected_chapter: {:?}", selected_chapter.get());
    // });
    view! {
        <Suspense fallback=move || view! {<Loading /> }>
            <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p> }>
            <div class="mx-auto mb-4 flex gap-4 bg-gray-100 p-5">
                // <!-- 侧边栏章节列表 -->
                <div class="w-1/6">
                    <div class="felx flex-row">
                        <input type="button" readonly class="focus:border-none focus:outline-none text-center text-black bg-gray-100" 
                            prop:value=novel_name()/>
                    </div>
                    //   <!-- 章节列表 -->
                    <ul class="overflow-auto" style="height: 500px;">
                        // <!-- 章节项 -->
                        {chapters_view}
                        // <!-- 新建章节... -->
                        <li class="border-b p-2">
                            // <input type="button" class="hover:text-blue-400" value="新建章节..." on:click=move |_ev| set_selected_chapter("".into())/>
                            <a href=format!{"/edit_chapter/{}/{}", novel_name(), ""} class="hover:text-blue-400">新建章节</a>
                        </li>
                    </ul>
                </div>
                // <!-- 内容编辑区域 -->
                <div class="flex h-full w-5/6 flex-col flex-wrap space-y-3">
                    <PreviewChapter novel_name=novel_name() chapter_title=selected_chapter />
                </div>
            </div>
            </ErrorBoundary>
        </Suspense>
    }
}
