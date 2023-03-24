#[cfg(feature = "SiOxygen")]
use leptos::*;
#[cfg(feature = "SiOxygen")]
///This icon requires the feature `SiOxygen` to be enabled.
#[component]
pub fn Oxygen(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23.89 12c0-6.627-5.324-12-11.89-12S.109 5.373.109 12 5.433 24 12 24c2.014 0 3.91-.508 5.573-1.4.62.354 1.338.558 2.105.558 2.326 0 4.212-1.865 4.212-4.165 0-.946-.319-1.818-.857-2.517.552-1.383.857-2.894.857-4.476zm-21.402.005c0-5.448 4.269-9.864 9.535-9.864s9.535 4.416 9.535 9.864c0 1.07-.166 2.099-.471 3.063a4.23 4.23 0 0 0-1.408-.239c-2.326 0-4.212 1.865-4.212 4.165 0 .72.185 1.397.51 1.988a9.21 9.21 0 0 1-3.953.888c-5.267-.001-9.536-4.418-9.536-9.865zm17.191 9.864c-1.514.021-2.84-1.267-2.819-2.788 0-1.54 1.262-2.788 2.819-2.788 1.507-.025 2.843 1.27 2.819 2.788 0 1.54-1.263 2.788-2.819 2.788z"
        /> < title > { title } < / title > < / svg >
    }
}
