#[cfg(feature = "SiExpensify")]
use leptos::*;
#[cfg(feature = "SiExpensify")]
///This icon requires the feature `SiExpensify` to be enabled.
#[component]
pub fn Expensify(
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
        "M8.16 17.52h7.68v-2.28h-4.872v-2.136h4.152v-2.328h-4.152v-2.04h4.872V6.48H8.16v11.04z M21.6 12a9.608 9.608 0 01-2.16 6.072l1.704 1.704A11.958 11.958 0 0024 12c0-2.928-1.056-5.616-2.784-7.68l-1.704 1.704A9.61 9.61 0 0121.6 12z M18.072 19.44a9.608 9.608 0 01-12.048.072L4.32 21.216A11.913 11.913 0 0012 24c2.976 0 5.688-1.08 7.776-2.856l-1.704-1.704z M4.632 18.168A9.613 9.613 0 012.4 12c0-5.304 4.296-9.6 9.6-9.6 2.352 0 4.488.84 6.168 2.232l1.704-1.704A12.02 12.02 0 0012 0C5.376 0 0 5.376 0 12c0 3 1.104 5.76 2.928 7.872l1.704-1.704z"
        /> < title > { title } < / title > < / svg >
    }
}
