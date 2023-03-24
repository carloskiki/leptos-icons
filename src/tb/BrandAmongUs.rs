#[cfg(feature = "TbBrandAmongUs")]
use leptos::*;
#[cfg(feature = "TbBrandAmongUs")]
///This icon requires the feature `TbBrandAmongUs` to be enabled.
#[component]
pub fn BrandAmongUs(
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
        "icon icon-tabler icon-tabler-brand-among-us" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.646 12.774c-1.939 .396 -4.467 .317 -6.234 -.601c-2.454 -1.263 -1.537 -4.66 1.423 -4.982c2.254 -.224 3.814 -.354 5.65 .214c.835 .256 1.93 .569 1.355 3.281c-.191 1.067 -1.07 1.904 -2.194 2.088z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.84 7.132c.083 -.564 .214 -1.12 .392 -1.661c.456 -.936 1.095 -2.068 3.985 -2.456a22.464 22.464 0 0 1 2.867 .08c1.776 .14 2.643 1.234 3.287 3.368c.339 1.157 .46 2.342 .629 3.537v11l-12.704 -.019c-.552 -2.386 -.262 -5.894 .204 -8.481"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 10c.991 .163 2.105 .383 3.069 .67c.255 .13 .52 .275 .534 .505c.264 3.434 .57 7.448 .278 9.825h-3.881"
        /> < title > { title } < / title > < / svg >
    }
}
