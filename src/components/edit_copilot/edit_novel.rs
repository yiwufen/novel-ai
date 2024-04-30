use leptos::logging::log;
use leptos::*;
use crate::actix_end::ai_copilot::{c_write_api, re_write_api};
use crate::components::loading::Loading;
#[component]
pub fn EditCopilot() -> impl IntoView {
    let text = String::from("清晨的第一缕阳光透过高高的梧桐树，洒在古老城池的青石板上。一名身着蓝布长袍的少年，手持竹简，正从狭窄的巷子中匆匆走出。这是京城中著名的书院所在地，也是梦想开始的地方。少年名叫沈翼，今年十六岁，是一个普通官吏的儿子，但他有着不同寻常的梦想——在翰林院中出人头地，成为一名朝廷大官。");
    let (text, set_text) = create_signal(text);

    let action_c_write = create_action(|novel_text: &String| {
        log!("action_c_write: {}", novel_text);
        let novel_text = novel_text.clone();
        async move { c_write_api(novel_text).await }
    });
    let action_re_write = create_action(move |input: &String| {
      log!("action_re_write");
      let input = input.clone();
      async move {
        re_write_api(input).await
      }
    });
    create_effect(move |_| {
        if let Some(Ok(result)) = action_c_write.value().get() {
            log!("action_c_write result: {:?}", result);
            set_text.update(|t| *t = t.to_owned() + &result);
        }
        if let Some(Ok(res)) = action_re_write.value().get() {
          log!("action_re_write res: {:?}", res);
          set_text(res);
        }
    });

    let loading_view = move ||  if action_c_write.pending().get() || action_re_write.pending().get() {
        view! { <Loading /> }.into_view()
    } else {
        view! {}.into_view()
    };
    view! {
      <Suspense fallback=move || view! { <Loading /> }>
        <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p>}>
          { loading_view }
          <div class="flex h-full bg-gray-100 dark:bg-gray-600 dark:text-white p-10">
            // <!-- Left Side -->
            <div class="flex flex-col w-1/2 space-y-10 select-none">
              <div class="flex flex-col h-screen-no-bar">
                // <!-- Toolbar section -->
                <div class="flex-none">
                //   <!-- Replace with actual toolbar content -->
                //   <!-- Toolbar items can be inline-flex for a horizontal set -->
                  <div class="bg-gray-200 p-4 dark:text-white dark:bg-gray-600">
                    <ul class="flex space-x-4">
                      <li>
                        <button class="h-10 px-6 font-semibold rounded-md border border-slate-300 hover:border-slate-700 text-slate-900" type="button"
                          on:click={move |_| action_c_write.dispatch(text())}
                        >
                          "AI续写"
                        </button>
                      </li>
                      <li>
                        <button class="h-10 px-6 font-semibold rounded-md border border-slate-300 hover:border-slate-700 text-slate-900" type="button"
                          on:click={move |_| action_re_write.dispatch(text())}
                        >
                          "AI改写"
                        </button>
                      </li>
                  </ul>

                  </div>
                </div>

                // <!-- Text editor section -->
                <div class="flex-grow">
                //   <!-- Replace with actual text editor content -->
                  <textarea class="w-full h-full p-4 select-text"
                    on:input=move |e| set_text(event_target_value(&e))
                    prop:value=text()
                  >
                  </textarea>
                </div>
              </div>
            </div>
            // <!-- Right Side -->
            <div class="bg-blue-200 dark:bg-gray-800 flex-1 p-6">
              {move || text()}
            </div>
          </div>
        </ErrorBoundary>
    </Suspense>

    }
}
