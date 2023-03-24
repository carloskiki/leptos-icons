#[cfg(feature = "SiGumroad")]
use leptos::*;
#[cfg(feature = "SiGumroad")]
///This icon requires the feature `SiGumroad` to be enabled.
#[component]
pub fn Gumroad(
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
        "M15.826 15.069a1.018 1.018 0 1 1-.001 2.036 1.018 1.018 0 0 1 0-2.036zM21.327 1.11a1.018 1.018 0 1 1 .001 2.036 1.018 1.018 0 0 1 0-2.036zM3.322 3.107h16.116a2.13 2.13 0 0 0 1.89 1.151c1.174 0 2.129-.955 2.129-2.13A2.131 2.131 0 0 0 21.327 0c-.89 0-1.654.55-1.97 1.329H3.321C1.764 1.329.543 2.51.543 4.019v17.156C.543 22.706 1.816 24 3.322 24h17.155c1.51 0 2.738-1.267 2.738-2.825V10.998c0-1.532-1.228-2.78-2.738-2.78H10.3c-1.553 0-2.866 1.274-2.866 2.78v3.198c0 1.484 1.286 2.691 2.866 2.691h3.554a2.132 2.132 0 0 0 1.972 1.329c1.174 0 2.129-.956 2.129-2.13s-.955-2.129-2.13-2.129a2.13 2.13 0 0 0-1.889 1.152H10.3c-.523 0-1.088-.349-1.088-.913v-3.198c0-.524.519-1 1.088-1h10.177c.538 0 .96.439.96 1v10.177c0 .567-.44 1.047-.96 1.047H3.322c-.533 0-1-.49-1-1.047V4.02c0-.52.43-.912 1-.912"
        /> < title > { title } < / title > < / svg >
    }
}
