#[cfg(feature = "SiBeijingsubway")]
use leptos::*;
#[cfg(feature = "SiBeijingsubway")]
///This icon requires the feature `SiBeijingsubway` to be enabled.
#[component]
pub fn Beijingsubway(
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
        "M7.8857.7537C11.2092-.5 15.09-.1816 18.1547 1.6493c2.826 1.6916 5.055 4.7166 5.5526 7.5028-1.2538 0-1.5523.0398-2.7862.0398-.796-2.5872-3.7216-5.4729-6.1097-6.07-3.1246-.6567-5.6321-.199-7.8014 1.3335C5.418 5.55 3.9253 7.4604 3.368 9.3312c-.8159 2.408-.1592 5.8112 1.234 7.9208 1.0945 1.5722 3.005 3.0648 4.836 3.642 1.7911.5572 4.7166.0397 6.4878-.5971 2.4877-.9155 4.3186-3.1842 5.1147-5.6918l-3.4628.0398c-.1593 1.1543-.3384 1.5523-1.035 2.8857l-9.4929-.0199V6.4257h9.473c1.1345 1.4328 1.3136 3.6817 1.3335 5.5723 0 0 3.8011.02 6.1296 0-.0398 3.6022-1.5523 7.1645-4.4181 9.3935-2.0897 1.6518-4.7166 2.607-7.3834 2.607-3.7216.0598-7.4232-1.7314-9.6522-4.7166C1.0992 17.451.343 15.2021.0445 12.9334c-.0796-1.3931.02-2.0897.0398-2.1693C.343 9.1521.761 7.56 1.577 6.167 2.9501 3.6792 5.219 1.7289 7.8857.7537m1.811 8.1795v6.2291l4.5972.02c1.0349-1.1942.995-1.9703-.0398-3.0649 1.0349-1.0946 1.0946-2.2488.02-3.1842H9.6966Z"
        /> < title > { title } < / title > < / svg >
    }
}
