#[cfg(feature = "BiRegularCheese")]
use leptos::*;
#[cfg(feature = "BiRegularCheese")]
///This icon requires the feature `BiRegularCheese` to be enabled.
#[component]
pub fn Cheese(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.16 2a1 1 0 0 0-.66.13l-12 7a.64.64 0 0 0-.13.1l-.1.08a1.17 1.17 0 0 0-.17.26.84.84 0 0 0-.1.43v10a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1V10a8.08 8.08 0 0 0-6.84-8zm0 2.05A6.07 6.07 0 0 1 19.93 9H6.7zM20 19H4v-8h16z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "6.5" cy = "16.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "11.5" cy = "13.5" r = "1.5"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "17" cy = "16" r = "2" /> <
        title > { title } < / title > < / svg >
    }
}
