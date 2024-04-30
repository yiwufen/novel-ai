use leptos::{logging::log, *};

use crate::{actix_end::chapter_api::get_chapter_edit_api, components::loading::Loading};

#[component]
pub fn PreviewChapter(novel_name: String, chapter_title: ReadSignal<String>) -> impl IntoView {
    let (novel_name, _) = create_signal(novel_name);
    create_effect(move |_| {
        log!("novel: {} selected_chapter: {}", novel_name() ,chapter_title());
        let uri = format!("/edit_chapter/{}/{}", novel_name(), chapter_title());
        log!("uri: {}", uri);
    });
    let chapter_edit_resouce = create_resource(
        move || (novel_name(), chapter_title()), 
        |(novel_name, chapter_title)| async move { get_chapter_edit_api(novel_name, chapter_title).await });
    
    let edit_url = create_memo(move |_| format!{"/edit_chapter/{}/{}", novel_name(), chapter_title()});

    view! {
        <Suspense fallback=move || view! {<Loading /> }>
        <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p> }>
            <div class="flex h-full flex-col flex-wrap space-y-3">
                <div class="flex flex-row w-full p-3">
                    <div class="w-1/3 bg-gray-100">
                        // <img class="w-48 h-48" src="http://43.139.238.38:3000/assets/icon.png" />
                    </div>
                    <div class="mb-4 w-1/3">
                        <h1 class="p-2 text-center text-lg"> {move || novel_name} </h1>
                        <h2 class="p-1 text-base"> <input class="bg-gray-100 text-center focus:border-none focus:outline-none" readonly
                            prop:value = {move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| chapter.chapter_title).ok())} /> 
                        </h2>
                    </div>
                    <div class="flex w-1/3 flex-row items-start justify-end space-x-4">
                        <a href={edit_url} class="p-2 hover:text-blue-400">编辑</a>
                        // <a href={format!("edit_chapter/{}/{}", novel_name(), chapter_title())} class="p-2 hover:text-blue-400">编辑2</a>
                        // <input type="button" class="p-2 hover:text-blue-400" value="编辑" 
                        //     on:click=click_chapter_edit />
                    </div>
                </div>
                <div>
                    <textarea class="w-full h-96 p-2 focus:outline-none bg-gray-100" readonly
                        prop:value = {move || chapter_edit_resouce.get().and_then(|res| res.map(|chapter| format!{"{}\n{}",chapter.chapter_start, chapter.content}).ok())} />
                </div>
            </div>
        </ErrorBoundary>
    </Suspense>
    }
}