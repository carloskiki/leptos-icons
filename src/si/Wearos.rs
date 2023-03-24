#[cfg(feature = "SiWearos")]
use leptos::*;
#[cfg(feature = "SiWearos")]
///This icon requires the feature `SiWearos` to be enabled.
#[component]
pub fn Wearos(
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
        "M8.416 21.1346c-.9687 0-1.8938-.56-2.3135-1.5015L.2193 6.4198C-.3488 5.1432.2248 3.6472 1.5014 3.079c1.2767-.5681 2.7727.0055 3.3408 1.2821l5.8832 13.2133c.5681 1.2767-.0055 2.7727-1.2821 3.3408a2.5254 2.5254 0 01-1.0273.2194zm7.1952.0368c-.891 0-1.7412-.515-2.1268-1.3816L7.39 6.1024C6.867 4.9279 7.3955 3.5532 8.5686 3.03c1.173-.5218 2.5492.0054 3.0724 1.1785l6.0943 13.6888c.5232 1.1745-.0054 2.5492-1.1785 3.0724a2.3111 2.3111 0 01-.9456.2017zM24 5.195a2.3271 2.3271 0 01-2.3271 2.327 2.3271 2.3271 0 01-2.3271-2.327 2.3271 2.3271 0 012.327-2.3271A2.3271 2.3271 0 0124 5.1949zm-2.6119 5.116a2.4892 2.4892 0 01-2.4892 2.4893 2.4892 2.4892 0 01-2.4893-2.4892 2.4892 2.4892 0 012.4893-2.4893 2.4892 2.4892 0 012.4892 2.4893Z"
        /> < title > { title } < / title > < / svg >
    }
}
