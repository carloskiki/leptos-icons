#[cfg(feature = "SiIpfs")]
use leptos::*;
#[cfg(feature = "SiIpfs")]
///This icon requires the feature `SiIpfs` to be enabled.
#[component]
pub fn Ipfs(
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
        "M12 0L1.608 6v12L12 24l10.392-6V6zm-1.073 1.445h.001a1.8 1.8 0 002.138 0l7.534 4.35a1.794 1.794 0 000 .403l-7.535 4.35a1.8 1.8 0 00-2.137 0l-7.536-4.35a1.795 1.795 0 000-.402zM21.324 7.4c.109.08.226.147.349.201v8.7a1.8 1.8 0 00-1.069 1.852l-7.535 4.35a1.8 1.8 0 00-.349-.2l-.009-8.653a1.8 1.8 0 001.07-1.851zm-18.648.048l7.535 4.35a1.8 1.8 0 001.069 1.852v8.7c-.124.054-.24.122-.349.202l-7.535-4.35a1.8 1.8 0 00-1.069-1.852v-8.7c.124-.054.24-.122.35-.202z"
        /> < title > { title } < / title > < / svg >
    }
}
