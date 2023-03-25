#[cfg(feature = "SiVapor")]
use leptos::*;
#[cfg(feature = "SiVapor")]
///This icon requires the feature `SiVapor` to be enabled.
#[component]
pub fn Vapor(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M22.75 13.908v1.56L12 24 1.25 15.468v-1.56L12 22.44l10.75-8.532zM12 17.267L1.25 8.824 12 0l10.75 8.824L12 17.267zm.356-4.635a3.193 3.193 0 0 0 3.193-3.193 3.185 3.185 0 0 0-3.029-3.176l.001-.016-4.514-.427 1.205 4.102a3.184 3.184 0 0 0 3.144 2.71zM12 20.269L1.25 11.737v1.533L12 21.802l10.75-8.532v-1.533L12 20.269zm0-2.366L1.25 9.46v1.64L12 19.63l10.75-8.532V9.46L12 17.903z"
        /> < title > { title } < / title > < / svg >
    }
}
