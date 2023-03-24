#[cfg(feature = "TbBrandYoutubeKids")]
use leptos::*;
#[cfg(feature = "TbBrandYoutubeKids")]
///This icon requires the feature `TbBrandYoutubeKids` to be enabled.
#[component]
pub fn BrandYoutubeKids(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-brand-youtube-kids" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.782 17.03l-3.413 .235l-.023 0c-1.117 .09 -2.214 .335 -3.257 .725l-2.197 .794a3.597 3.597 0 0 1 -2.876 -.189a3.342 3.342 0 0 1 -1.732 -2.211l-1.204 -5.293a3.21 3.21 0 0 1 .469 -2.503a3.468 3.468 0 0 1 2.177 -1.452l9.843 -2.06c1.87 -.392 3.716 .744 4.124 2.537l1.227 5.392a3.217 3.217 0 0 1 -.61 2.7a3.506 3.506 0 0 1 -2.528 1.323z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 10l.972 4l4.028 -3z" /> <
        title > { title } < / title > < / svg >
    }
}
