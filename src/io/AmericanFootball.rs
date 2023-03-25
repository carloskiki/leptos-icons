#[cfg(feature = "IoAmericanFootball")]
use leptos::*;
#[cfg(feature = "IoAmericanFootball")]
///This icon requires the feature `IoAmericanFootball` to be enabled.
#[component]
pub fn AmericanFootball(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M122.06,122.06c-44.37,44.37-66.71,100.61-78,145.28L244.66,467.9c44.67-11.25,100.91-33.59,145.28-78s66.71-100.61,78-145.28L267.34,44.1C222.67,55.35,166.43,77.69,122.06,122.06ZM378.79,378.78h0ZM300.65,189,323,166.71A15.78,15.78,0,0,1,345.29,189L323,211.35l11.16,11.17a15.78,15.78,0,0,1-22.32,22.32l-11.16-11.16L278.32,256l11.16,11.16a15.78,15.78,0,1,1-22.32,22.32L256,278.32l-22.32,22.33,11.16,11.16a15.78,15.78,0,1,1-22.32,22.32L211.35,323,189,345.29A15.78,15.78,0,0,1,166.71,323L189,300.65l-11.16-11.17a15.78,15.78,0,0,1,22.32-22.32l11.16,11.16L233.68,256l-11.16-11.16a15.78,15.78,0,1,1,22.32-22.32L256,233.68l22.32-22.33-11.16-11.16a15.78,15.78,0,0,1,22.32-22.32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M476.57,199.63c7.31-54.53,4-120.26-20-144.21s-89.68-27.3-144.21-20c-2.51.34-5.16.72-7.91,1.15l171,171C475.85,204.79,476.23,202.14,476.57,199.63Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M35.43,312.37c-7.31,54.53-4,120.26,20,144.21C72.17,473.33,109.34,480,148.84,480a387,387,0,0,0,50.79-3.43c2.51-.34,5.16-.72,7.91-1.15l-171-171C36.15,307.21,35.77,309.86,35.43,312.37Z"
        /> < title > { title } < / title > < / svg >
    }
}
