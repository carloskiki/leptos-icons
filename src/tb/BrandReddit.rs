#[cfg(feature = "TbBrandReddit")]
use leptos::*;
#[cfg(feature = "TbBrandReddit")]
///This icon requires the feature `TbBrandReddit` to be enabled.
#[component]
pub fn BrandReddit(
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
        "icon icon-tabler icon-tabler-brand-reddit" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 8c2.648 0 5.028 .826 6.675 2.14a2.5 2.5 0 0 1 2.326 4.36c0 3.59 -4.03 6.5 -9 6.5c-4.875 0 -8.845 -2.8 -9 -6.294l-1 -.206a2.5 2.5 0 0 1 2.326 -4.36c1.646 -1.313 4.026 -2.14 6.674 -2.14z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 8l1 -5l6 1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M19 4m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "9" cy = "13" r = ".5" fill =
        "currentColor" />< circle xmlns = "http://www.w3.org/2000/svg" cx = "15" cy =
        "13" r = ".5" fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M10 17c.667 .333 1.333 .5 2 .5s1.333 -.167 2 -.5" /> < title > { title } < /
        title > < / svg >
    }
}
