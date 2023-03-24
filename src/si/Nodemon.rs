#[cfg(feature = "SiNodemon")]
use leptos::*;
#[cfg(feature = "SiNodemon")]
///This icon requires the feature `SiNodemon` to be enabled.
#[component]
pub fn Nodemon(
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
        "M22.33 7.851l-.716-.398c1.101-1.569 1.758-3.927.934-7.453 0 0-1.857 5.029-5.59 4.863l-4.37-2.431a1.171 1.171 0 0 0-.536-.15h-.101a1.183 1.183 0 0 0-.538.15L7.042 4.863C3.309 5.03 1.452 0 1.452 0c-.825 3.526-.166 5.884.934 7.453l-.716.398a1.133 1.133 0 0 0-.589.988l.022 14.591c0 .203.109.392.294.491a.58.58 0 0 0 .584 0l5.79-3.204c.366-.211.589-.582.589-.987v-6.817c0-.406.223-.783.588-.984l2.465-1.372a1.19 1.19 0 0 1 .59-.154c.2 0 .407.05.585.154l2.465 1.372c.365.201.588.578.588.984v6.817c0 .405.226.779.59.987l5.788 3.204a.59.59 0 0 0 .589 0 .564.564 0 0 0 .292-.491l.019-14.591a1.129 1.129 0 0 0-.589-.988z"
        /> < title > { title } < / title > < / svg >
    }
}
