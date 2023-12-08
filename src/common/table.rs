use serde_json::Value;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Callback, Html, Properties};
use yew_hooks::use_clipboard;
use yew_notifications::{use_notification, Notification, NotificationType};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct TableViewProps {
    pub value: Value,
}

#[function_component(TableView)]
pub fn table_view(props: &TableViewProps) -> Html {
    let clipboard = use_clipboard();
    let copy_text = Callback::from(move |text: String| clipboard.write_text(text.clone()));
    let notifications = use_notification::<Notification>();
    let spawn_notification = Callback::from(move |notification| notifications.spawn(notification));

    html! {
        <div class="table-container">
            {build_cells(&props.value, copy_text, spawn_notification)}
        </div>
    }
}

fn format_json_value(value: &Value, copy_text: Callback<String>, spawn_notification: Callback<Notification>) -> VNode {
    let data = match value {
        Value::Null => String::from("Null"),
        Value::Bool(bool) => format!("{}", bool),
        Value::Number(number) => format!("{}", number),
        Value::String(string) => string.clone(),
        Value::Array(array) => serde_json::to_string(array).unwrap(),
        Value::Object(obj) => serde_json::to_string(obj).unwrap(),
    };
    let data_to_copy = data.clone();
    let onclick = Callback::from(move |_| {
        copy_text.emit(data_to_copy.clone());
        spawn_notification.emit(Notification::from_description_and_type(
            NotificationType::Info,
            "Copied!",
        ));
    });
    html! {
        <span class="table-cell" {onclick}>{data}</span>
    }
}

fn build_cells(value: &Value, copy_text: Callback<String>, spawn_notification: Callback<Notification>) -> Vec<VNode> {
    if let Some(obj) = value.as_object() {
        obj.iter()
            .flat_map(|(key, value)| {
                let key_copy_text = copy_text.clone();
                let key_notification = spawn_notification.clone();
                let key_value = key.to_owned();
                let on_key_click = Callback::from(move |_| {
                    key_copy_text.emit(key_value.clone());
                    key_notification.emit(Notification::from_description_and_type(
                        NotificationType::Info,
                        "Copied!",
                    ));
                });
                vec![
                    html! { <span class="table-cell" onclick={on_key_click}>{key}</span> },
                    format_json_value(value, copy_text.clone(), spawn_notification.clone()),
                ]
            })
            .collect()
    } else {
        vec![format_json_value(value, copy_text, spawn_notification)]
    }
}
