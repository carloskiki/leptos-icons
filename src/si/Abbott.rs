#[cfg(feature = "SiAbbott")]
use leptos::*;
#[cfg(feature = "SiAbbott")]
///This icon requires the feature `SiAbbott` to be enabled.
#[component]
pub fn Abbott(
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
        "M20.812 2.4H0v3.197h19.773V5.6a1.03 1.03 0 0 1 1.032 1.031v10.742l-.004.007a1.034 1.034 0 0 1-1.034 1.025H4.23c-.569 0-1.033-.46-1.033-1.033v-4.34c0-.57.464-1.032 1.033-1.032H17.6V8.803H3.188A3.185 3.185 0 0 0 0 11.99v6.423A3.188 3.188 0 0 0 3.188 21.6h17.624A3.187 3.187 0 0 0 24 18.412V5.587A3.186 3.186 0 0 0 20.812 2.4"
        /> < title > { title } < / title > < / svg >
    }
}
