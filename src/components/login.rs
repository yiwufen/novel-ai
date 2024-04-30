use gloo_storage::Storage;
use leptos::{ev::MouseEvent, logging::log, *};

use crate::actix_end::auth::get_token_api;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let login_action = create_action(|info: &(String, String)| {
        log!("login_action");
        let (email, password) = info.clone();
        async move { get_token_api(email, password).await }
    });
    let login_click = move |ev: MouseEvent| {
        ev.prevent_default();
        log!("login_click");
        login_action.dispatch((email(), password()));
    };
    create_effect(move |_| {
        if let Some(Ok(s)) = login_action.value().get() {
            log!("local token: {:?}", s);
            gloo_storage::LocalStorage::set("auth_token", s).expect("failed to set auth_token");
        }
    });
    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
              <img class="mx-auto h-10 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company"/>
              <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">Sign in to your account</h2>
            </div>

            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
              <form class="space-y-6">
                <div class="text-left">
                  <label class="block text-sm font-medium leading-6 text-gray-900">Email address</label>
                  <div class="mt-2">
                    <input id="email" name="email" type="email" autocomplete="email" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        on:input=move |ev| set_email(event_target_value(&ev))
                        prop:value=move || email()
                    />
                  </div>
                </div>

                <div>
                  <div class="flex items-center justify-between">
                    <label class="block text-sm font-medium leading-6 text-gray-900">Password</label>
                    <div class="text-sm">
                      <a href="#" class="font-semibold text-indigo-600 hover:text-indigo-500">Forgot password?</a>
                    </div>
                  </div>
                  <div class="mt-2">
                    <input id="password" name="password" type="password" autocomplete="current-password" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        on:input=move |ev| set_password(event_target_value(&ev))
                        prop:value=move || password()
                    />
                  </div>
                </div>

                <div>
                  <button type="submit" class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                        on:click=login_click
                  >Sign in</button>
                </div>
              </form>

              <p class="mt-10 text-center text-sm text-gray-500">
                Not a member?
                <a href="/register" class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">注册</a>
              </p>
            </div>
          </div>
    }
}
