use nostr_sdk::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async {
        let keys =
            Keys::from_sk_str("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
                .unwrap();
        let url = Url::parse("http://127.0.0.1:7773").unwrap();
        let client = Client::new(&keys, url);

        /* let contacts = client.get_contact_list_metadata().await.unwrap();
        console::log_1(&format!("{contacts:?}").into()); */

        client
            .publish_text_note("Hello from nostr-sdk WASM!", &[])
            .await
            .unwrap();

        let filter = Filter::new().author(keys.public_key());
        let events = client.get_events_of(vec![filter]).await.unwrap();
        console::log_1(&format!("{events:?}").into());
    });

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
