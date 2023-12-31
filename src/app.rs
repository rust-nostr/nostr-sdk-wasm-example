use nostr_sdk::prelude::*;
use nostr_sdk::nips::nip07::Nip07Signer;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async {
        let signer = ClientSigner::NIP07(Nip07Signer::new().unwrap());
        let client = Client::new(signer);

        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.add_relay("wss://relay.rip").await.unwrap();

        client.connect().await;

        // Publish text note
        let event_id = client
            .publish_text_note("Testing `nostr-sdk` WASM with NIP07 signer!", [])
            .await
            .unwrap();
        console::log_1(&format!("Event id: {event_id}").into());
    });

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
