#[cfg(feature = "SiSourcetree")]
use leptos::*;
#[cfg(feature = "SiSourcetree")]
///This icon requires the feature `SiSourcetree` to be enabled.
#[component]
pub fn Sourcetree(
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
        "M11.999 0C6.756 0 2.474 4.245 2.474 9.525c0 4.21 2.769 7.792 6.572 9.047v4.764c0 .37.295.664.664.664h4.506a.661.661 0 0 0 .664-.664v-4.764c.025-.008.049-.019.074-.027v.064c3.694-1.22 6.412-4.634 6.565-8.687.005-.124.007-.25.007-.375v-.022c0-.152-.006-.304-.013-.455C21.275 4.037 17.125 0 11.999 0Zm0 6.352a3.214 3.214 0 0 1 2.664 5.005v.002A3.218 3.218 0 0 1 12 12.775a3.212 3.212 0 0 1 0-6.424z"
        /> < title > { title } < / title > < / svg >
    }
}
