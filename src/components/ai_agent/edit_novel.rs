use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use crate::{actix_end::novel_api::{delete_novel, get_novel, upsert_novel}, app_models::novel::Novel};

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditNovelParams {
    edit_novel_name: Option<String>,
}


/// https://play.tailwindcss.com/EkjuGwz8Nl
/// 
#[component]
pub fn EditNovel() -> impl IntoView {
    let params = use_params::<EditNovelParams>();
    
    let novel_resource = create_resource(
        move || params(),
        |params| async move {
            match params {
                Ok(EditNovelParams { edit_novel_name: Some(name)}) => get_novel(name).await,
                _ => Ok(Novel::default())
            }
        }
    );

    // let (novel_name, set_novel_name) = create_signal(String::from("微风与月"));
    // let (description, set_description) = create_signal(String::from("这里是小说的简介，可以详细描述小说的内容、主题或者其它吸引读者的信息。"));
    // let (cover, set_cover) = create_signal(String::from("https://th.bing.com/th/id/R.73ae5184f285dd1183060b792184f0e6?rik=3gA%2blabRY5DDfQ&riu=http%3a%2f%2fimg95.699pic.com%2fphoto%2f40101%2f4110.jpg_wh300.jpg!%2ffh%2f300%2fquality%2f90&ehk=SIwQPO2LzFXDQ3pJjPOlziW5dwfLDwSucFFl3yBfjSs%3d&risl=&pid=ImgRaw&r=0&sres=1&sresct=1"));
    
    let name_input_view = move || {
        match params() {
            Ok(EditNovelParams { edit_novel_name: Some(name)}) => {
                if name.is_empty() {
                    view! {
                        <input id="novelName" type="text" placeholder="请输入小说名字" class="rounded border border-gray-200 p-2" 
                                prop:value= move || novel_resource.get().and_then(|res| res.map(|novel| novel.novel_name).ok())
                                on:input=move |ev| novel_resource.update(|res| {
                                    if let Some(Ok(novel)) = res {
                                        novel.novel_name = event_target_value(&ev);
                                    }
                                })
                            />
                    }.into_view()
                }else {
                    view! {
                        <input id="novelName" type="text" placeholder="请输入小说名字" class="rounded border border-gray-200 p-2" readonly
                                prop:value= move || novel_resource.get().and_then(|res| res.map(|novel| novel.novel_name).ok())
                            />
                    }.into_view()
                }
            },
            _ => {
                view! {
                    <input id="novelName" type="text" placeholder="请输入小说名字" class="rounded border border-gray-200 p-2" 
                            prop:value= move || novel_resource.get().and_then(|res| res.map(|novel| novel.novel_name).ok())
                            on:input=move |ev| novel_resource.update(|res| {
                                if let Some(Ok(novel)) = res {
                                    novel.novel_name = event_target_value(&ev);
                                }
                            })
                        />
                }.into_view()
            }
        }
    };

    let action_upsert_novel = create_action(|novel: &Novel| {
        let novel = novel.clone();
        async move {
            upsert_novel(novel).await
        }
    });

    let action_delete_novel = create_action(|novel_name: &String| {
        let name = novel_name.clone();
        async move {
            delete_novel(name).await
        }
    });

    let chapters_url = create_memo(move |_| {
        if let Some(Ok(novel)) = novel_resource.get() {
            format!("/view_chapters/{}", novel.novel_name)
        }
        else {
            "".to_string()
        }
    });

    view! {
        <Suspense fallback=move || view! {<p>"Loading ..."</p> }>
            <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p> }>
                <div class="flex h-screen">
                    // <!-- Left Panel -->
                    <div class="m-auto flex max-h-[90%] w-1/2 flex-col space-y-8 rounded-lg bg-gray-50 p-10 shadow-lg">
                    //   <!-- Input Group for Novel Name -->
                      <div class="flex flex-col space-y-2">
                        <label for="novelName" class="font-semibold text-gray-800">小说名字</label>
                        {name_input_view}
                      </div>
                    //   <!-- Input Group for Description -->
                      <div class="flex flex-col space-y-2">
                        <label for="description" class="font-semibold text-gray-800">介绍</label>
                        <input id="description" type="text" placeholder="请输入小说介绍" class="rounded border border-gray-200 p-2" 
                            prop:value=move || novel_resource.get().and_then(|res| res.map(|novel| novel.description).ok())
                            on:input=move |ev| novel_resource.update(|res| {
                                if let Some(Ok(novel)) = res {
                                    novel.description = event_target_value(&ev);
                                }
                            })
                        />
                      </div>
                    //   <!-- Input Group for Cover Image -->
                      <div class="flex flex-col space-y-2">
                        <label for="cover" class="font-semibold text-gray-800">封面</label>
                        <input id="cover" type="text" placeholder="请输入封面图片链接" class="rounded border border-gray-200 p-2" 
                            prop:value=move || novel_resource.get().and_then(|res| res.map(|novel| novel.image_url).ok())
                            on:input=move |ev| novel_resource.update(|res| {
                                if let Some(Ok(novel)) = res {
                                    novel.image_url = event_target_value(&ev);
                                }
                            })
                        />
                      </div>
                      <div class="flex space-x-4">
                        // <!-- Submit Button with softer gradient and shadow -->
                        <button class="flex-1 rounded bg-gradient-to-r from-blue-400 to-blue-500 px-4 py-2 font-semibold text-white shadow hover:from-blue-500 hover:to-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
                            on:click= move |_| {
                                if let Some(Ok(novel)) =  novel_resource.get(){
                                    action_upsert_novel.dispatch(novel);
                                }
                            }
                        >提交</button>
                        // <!-- Delete Button with softer gradient and shadow -->
                        <button class="flex-1 rounded bg-gradient-to-r from-red-400 to-red-500 px-4 py-2 font-semibold text-white shadow hover:from-red-500 hover:to-red-600 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50"
                            on:click= move |_| {
                                if let Some(Ok(novel)) = novel_resource.get() {
                                    action_delete_novel.dispatch(novel.novel_name);
                                }
                            }
                        >删除</button>
                      </div>
                    </div>
                  
                    // <!-- Right Panel as a Card -->
                    <div class="flex w-1/2 items-center justify-center bg-gray-200 p-10">
                        <a href=chapters_url>
                            <div class="overflow-hidden rounded-lg bg-white shadow-lg transition-shadow duration-300 hover:shadow-xl">
                                <img src=move || novel_resource.get().and_then(|res| res.map(|novel| novel.image_url).ok()) alt="Novel Cover" class="h-64 w-full object-cover" />
                                <div class="p-5">
                                <h1 class="mb-3 text-3xl font-bold text-gray-800">{move || novel_resource.get().and_then(|res| res.map(|novel| novel.novel_name).ok())}</h1>
                                //   <h3 class="mb-2 text-xl font-semibold text-gray-800">{move || novel_resource.get().and_then(|res| res.map(|novel| novel.description).ok())}</h3>
                                <p class="text-gray-700">{move || novel_resource.get().and_then(|res| res.map(|novel| novel.description).ok())}</p>
                                </div>
                            </div>
                        </a>
                    </div>
                  </div>                  
            </ErrorBoundary>
        </Suspense>
    }
}