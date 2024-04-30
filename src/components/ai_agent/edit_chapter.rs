use crate::actix_end::chapter_api::{
    c_write_agent_api, delete_chapter_edit_api, get_chapter_edit_api, upsert_chapter_edit_api,
};
use crate::app_models::novel::ChapterEdit;
use crate::components::loading::Loading;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditChapterParams {
    novel_name: String,
    chapter_title: Option<String>,
}

#[component]
pub fn EditChapter() -> impl IntoView {
    let params = use_params::<EditChapterParams>();

    let chapter_edit_resouce = create_resource(
        move || params(),
        |params| async move {
            match params {
                Ok(params) => {
                    if let Some(title) = params.chapter_title {
                        get_chapter_edit_api(params.novel_name, title).await
                    } else {
                        get_chapter_edit_api(params.novel_name, "".into()).await
                    }
                }
                Err(_) => Err(ServerFnError::ServerError("params error".to_string())),
            }
        },
    );

    let upsert_chapter_edit = create_action(|input: &ChapterEdit| {
        let input = input.clone();
        async move { upsert_chapter_edit_api(input).await }
    });
    let save_chapter_edit = move |_ev| {
        log!("save_chapter_edit");
        if let Some(Ok(chapter)) = chapter_edit_resouce.get() {
            if !chapter.chapter_title.is_empty() {
                upsert_chapter_edit.dispatch(chapter.clone());
                let navigate = use_navigate();
                navigate(&format!("/edit_chapter/{}/{}", chapter.novel_name, chapter.chapter_title), Default::default());
            }
        }
    };
    let c_write_action = create_action(|chapter: &ChapterEdit| {
        let chapter = chapter.clone();
        async move { c_write_agent_api(chapter).await }
    });
    let c_write_click = move |_| {
        chapter_edit_resouce.and_then(|chapter| c_write_action.dispatch(chapter.clone()));
    };

    let delete_chapter_edit = create_action(|info: &(String, String)| {
        log!("delete_chapter_edit");
        let (name, title) = info.clone();
        async move { delete_chapter_edit_api(name, title).await }
    });
    let delete_click = move |_| {
        if let Some(Ok(chapter)) = chapter_edit_resouce.get() {
            delete_chapter_edit
                .dispatch((chapter.novel_name.clone(), chapter.chapter_title.clone()));
            let navigate = use_navigate();
            navigate(&format!("/view_chapters/{}", chapter.novel_name), Default::default());
        }
    };
    create_effect(move |_| {
        if let Some(Ok(s)) = c_write_action.value().get() {
            log!("c_write_action: {:?}", s);
            chapter_edit_resouce.update(|res| {
                if let Some(Ok(chapter)) = res {
                    chapter.content = s;
                }
            });
        }
    });
    let loading_view = move || {
        if c_write_action.pending().get() {
            view! { <Loading /> }.into_view()
        } else {
            view! {}.into_view()
        }
    };

    view! {
        <Suspense fallback=move || view! {<Loading /> }>
            <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p> }>
                {loading_view}
                <div class="flex w-screen flex-col flex-wrap space-y-3 bg-gray-100">
                    <div class="flex flex-row w-full p-3">
                        <div class="w-1/3 bg-gray-100">
                            // <img class="w-48 h-24" src="http://43.139.238.38:3000/assets/icon.png" />
                        </div>
                        <div class="mb-4 w-1/3 bg-gray-100">
                            <h1 class="p-2 text-center text-lg">{ move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.novel_name).ok())}</h1>
                            <h2 class="p-1 text-base"> <input class="bg-gray-100 text-center focus:border-none focus:outline-none"
                                on:input={move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.chapter_title = event_target_value(&ev);
                                    }
                                })}
                                prop:value = {move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.chapter_title).ok())} />
                            </h2>
                        </div>
                        <div class="flex w-1/3 flex-row items-start justify-end space-x-4">
                            <input type="button" class="p-2 hover:text-blue-400" value="保存"
                                on:click=save_chapter_edit />
                            <input type="button" class="p-2 hover:text-blue-400" value="删除"
                                on:click=delete_click
                            />
                        </div>
                    </div>
                    <div class="flex flex-row w-full p-3 space-x-2">
                        // <!-- 当前情节输入 -->
                        <div class="w-1/4 flex flex-col p-2">
                            <h3 class="text-lg">当前情节</h3>
                            <textarea placeholder="当前情节" class="mr-2 h-24  rounded border border-gray-300 p-2"
                                on:input={move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.current_plot = event_target_value(&ev);
                                    }
                                }) }
                                prop:value={move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.current_plot).ok())}
                            ></textarea>
                        </div>
                        // <!-- 情节发展输入 -->
                        <div class="w-1/4 flex flex-col p-2" >
                            <h3 class="text-lg">情节发展</h3>
                            <textarea placeholder="情节发展" class="mr-2 h-24 rounded border border-gray-300 p-2"
                                on:input={move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.plot_development = event_target_value(&ev);
                                    }
                                })}
                                prop:value={move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.plot_development).ok())}
                            ></textarea>
                        </div>
                        // <!-- 写作风格输入 -->
                        <div class="w-1/4 flex flex-col p-2">
                            <h3 class="text-lg">写作风格</h3>
                            <textarea placeholder="写作风格" class="mr-2 h-24 rounded border border-gray-300 p-2"
                                on:input={move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.writing_style = event_target_value(&ev);
                                    }
                                }) }
                                prop:value={move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.writing_style).ok())}
                            ></textarea>
                        </div>
                        <div class="w-1/6 flex flex-col p-2">
                            <h3 class="text-lg"> 关键人物 </h3>
                            <textarea placeholder="关键人物" class="h-24 rounded border border-gray-300 p-2"
                                on:input={move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.key_role = event_target_value(&ev);
                                    }
                                })}
                                prop:value={move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.key_role).ok())}
                            > </textarea>
                        </div>
                    </div>

                    <div class="flex flex-row w-full p-3 space-x-2">

                        <div class="w-1/6 flex flex-col p-2">
                            <h3 class="text-lg"> 小说背景 </h3>
                            <textarea class="h-32 mr-2 p-2" placeholder="小说背景"
                                on:input={move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.bg_novel = event_target_value(&ev);
                                    }
                                })}
                                prop:value={move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.bg_novel).ok())}
                            > </textarea>
                        </div>
                        // <!-- 章节开始内容 -->
                        <div class="w-1/3 flex flex-col p-2">
                            <h3 class="text-lg"> 章节开始内容 </h3>
                            <textarea placeholder="章节开始内容" class="mr-2 h-96 rounded border border-gray-300 p-2"
                                on:input={move |ev| chapter_edit_resouce.update(|res| if let Some(Ok(chapter)) = res {chapter.chapter_start = event_target_value(&ev);})}
                                prop:value={move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.chapter_start).ok())}
                            ></textarea>
                        </div>

                        // <!-- 操作按钮 -->
                        <div class="mr-2 flex w-14 flex-col justify-between items-center p-2">
                            <input class="rounded hover:text-blue-400 p-2 text-black" type="button"
                                on:click=c_write_click
                                value="续写"
                            />
                        </div>
                        // <!-- 章节续写内容 -->
                        <div class="w-1/3 flex flex-col p-2">
                            <h3 class="text-lg"> 章节续写内容 </h3>
                            <textarea placeholder="章节续写内容" class="h-96 rounded border border-gray-300 p-2"
                                on:input=move |ev| chapter_edit_resouce.update(|res| {
                                    if let Some(Ok(chapter)) = res {
                                        chapter.content = event_target_value(&ev);
                                    }
                                })
                                prop:value=move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.content).ok())
                            ></textarea>
                        </div>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>

    }
}
