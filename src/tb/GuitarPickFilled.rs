#[cfg(feature = "TbGuitarPickFilled")]
use leptos::*;
#[cfg(feature = "TbGuitarPickFilled")]
///This icon requires the feature `TbGuitarPickFilled` to be enabled.
#[component]
pub fn GuitarPickFilled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-guitar-pick-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2c-1.613 0 -2.882 .104 -3.825 .323l-.23 .057c-3.019 .708 -4.945 2.503 -4.945 5.62c0 3.367 1.939 8.274 4.22 11.125c.32 .4 .664 .786 1.03 1.158l.367 .36a4.904 4.904 0 0 0 6.752 .011a15.04 15.04 0 0 0 1.41 -1.528c2.491 -3.113 4.221 -7.294 4.221 -11.126c0 -3.025 -1.813 -4.806 -4.71 -5.562l-.266 -.066c-.936 -.25 -2.281 -.372 -4.024 -.372z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
