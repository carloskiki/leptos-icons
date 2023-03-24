#[cfg(feature = "IoBusiness")]
use leptos::*;
#[cfg(feature = "IoBusiness")]
///This icon requires the feature `IoBusiness` to be enabled.
#[component]
pub fn Business(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M432,176H320V64a48,48,0,0,0-48-48H80A48,48,0,0,0,32,64V480a16,16,0,0,0,16,16H152a8,8,0,0,0,8-8V416.45c0-8.61,6.62-16,15.23-16.43A16,16,0,0,1,192,416v72a8,8,0,0,0,8,8H464a16,16,0,0,0,16-16V224A48,48,0,0,0,432,176ZM98.08,431.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,431.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,351.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,271.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,191.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,111.87Zm80,240a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,351.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,271.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,191.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,111.87Zm80,320a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,431.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,351.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,271.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,191.87Zm0-80a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,111.87ZM444,464H320V208H432a16,16,0,0,1,16,16V460A4,4,0,0,1,444,464Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,400a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M400,320a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,240a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M336,400a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,320a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M336,240a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z"
        /> < title > { title } < / title > < / svg >
    }
}
