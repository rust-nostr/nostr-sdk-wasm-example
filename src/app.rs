use std::time::Duration;

use nostr_sdk::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async {
        let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
        let keys = Keys::new(secret_key);
        let client = Client::new(&keys);

        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.add_relay("wss://relay.rip").await.unwrap();

        client.connect().await;

        // Publish text note
        let event_id = client
            .publish_text_note("Testing nostr-sdk WASM", &[])
            .await
            .unwrap();
        console::log_1(&format!("Event id: {event_id}").into());

        let timeout = Some(Duration::from_secs(10));

        // Get contact list
        let contacts = client.get_contact_list_metadata(timeout).await.unwrap();
        console::log_1(&format!("Contacts: {contacts:?}").into());

        // Subscribe to filters
        let subscription = Filter::new()
            .pubkey(keys.public_key())
            .since(Timestamp::now());
        client.subscribe(vec![subscription]).await;
        let mut notifications = client.notifications();
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event(_url, event) = notification {
                if event.kind == Kind::EncryptedDirectMessage {
                    if let Ok(msg) =
                        decrypt(&keys.secret_key().unwrap(), &event.pubkey, &event.content)
                    {
                        console::log_1(&format!("New DM: {msg}").into());
                    } else {
                        console::log_1(&"Impossible to decrypt direct message".into());
                    }
                } else {
                    console::log_1(&format!("{event:?}").into());
                }
            }
        }
    });

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
