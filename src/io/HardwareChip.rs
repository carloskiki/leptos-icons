#[cfg(feature = "IoHardwareChip")]
use leptos::*;
#[cfg(feature = "IoHardwareChip")]
///This icon requires the feature `IoHardwareChip` to be enabled.
#[component]
pub fn HardwareChip(
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
        "M352,128H160a32,32,0,0,0-32,32V352a32,32,0,0,0,32,32H352a32,32,0,0,0,32-32V160A32,32,0,0,0,352,128Zm0,216a8,8,0,0,1-8,8H168a8,8,0,0,1-8-8V168a8,8,0,0,1,8-8H344a8,8,0,0,1,8,8Z"
        style = "fill:none" />< rect xmlns = "http://www.w3.org/2000/svg" x = "160" y =
        "160" width = "192" height = "192" rx = "8" ry = "8" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M464,192a16,16,0,0,0,0-32H448V128a64.07,64.07,0,0,0-64-64H352V48a16,16,0,0,0-32,0V64H272V48a16,16,0,0,0-32,0V64H192V48a16,16,0,0,0-32,0V64H128a64.07,64.07,0,0,0-64,64v32H48a16,16,0,0,0,0,32H64v48H48a16,16,0,0,0,0,32H64v48H48a16,16,0,0,0,0,32H64v32a64.07,64.07,0,0,0,64,64h32v16a16,16,0,0,0,32,0V448h48v16a16,16,0,0,0,32,0V448h48v16a16,16,0,0,0,32,0V448h32a64.07,64.07,0,0,0,64-64V352h16a16,16,0,0,0,0-32H448V272h16a16,16,0,0,0,0-32H448V192ZM384,352a32,32,0,0,1-32,32H160a32,32,0,0,1-32-32V160a32,32,0,0,1,32-32H352a32,32,0,0,1,32,32Z"
        /> < title > { title } < / title > < / svg >
    }
}
