#[cfg(feature = "AiFilledCalculator")]
use leptos::*;
#[cfg(feature = "AiFilledCalculator")]
///This icon requires the feature `AiFilledCalculator` to be enabled.
#[component]
pub fn Calculator(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM440.2 765h-50.8c-2.2 0-4.5-1.1-5.9-2.9L348 718.6l-35.5 43.5a7.38 7.38 0 0 1-5.9 2.9h-50.8c-6.6 0-10.2-7.9-5.8-13.1l62.7-76.8-61.2-74.9c-4.3-5.2-.7-13.1 5.9-13.1h50.9c2.2 0 4.5 1.1 5.9 2.9l34 41.6 34-41.6c1.5-1.9 3.6-2.9 5.9-2.9h50.8c6.6 0 10.2 7.9 5.9 13.1L383.5 675l62.7 76.8c4.2 5.3.6 13.2-6 13.2zm7.8-382c0 2.2-1.4 4-3.2 4H376v68.7c0 1.9-1.8 3.3-4 3.3h-48c-2.2 0-4-1.4-4-3.2V387h-68.8c-1.8 0-3.2-1.8-3.2-4v-48c0-2.2 1.4-4 3.2-4H320v-68.8c0-1.8 1.8-3.2 4-3.2h48c2.2 0 4 1.4 4 3.2V331h68.7c1.9 0 3.3 1.8 3.3 4v48zm328 369c0 2.2-1.4 4-3.2 4H579.2c-1.8 0-3.2-1.8-3.2-4v-48c0-2.2 1.4-4 3.2-4h193.5c1.9 0 3.3 1.8 3.3 4v48zm0-104c0 2.2-1.4 4-3.2 4H579.2c-1.8 0-3.2-1.8-3.2-4v-48c0-2.2 1.4-4 3.2-4h193.5c1.9 0 3.3 1.8 3.3 4v48zm0-265c0 2.2-1.4 4-3.2 4H579.2c-1.8 0-3.2-1.8-3.2-4v-48c0-2.2 1.4-4 3.2-4h193.5c1.9 0 3.3 1.8 3.3 4v48z"
        /> < title > { title } < / title > < / svg >
    }
}
