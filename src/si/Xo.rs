#[cfg(feature = "SiXo")]
use leptos::*;
#[cfg(feature = "SiXo")]
///This icon requires the feature `SiXo` to be enabled.
#[component]
pub fn Xo(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "m1.629 5.698 4.275 5.367 4.274-5.367h1.613l-5.089 6.384 4.958 6.219h-1.613L5.903 13.1l-4.142 5.201H.131l4.957-6.219L0 5.698h1.629Zm16.48-.082C21.423 5.616 24 8.632 24 12c0 3.425-2.613 6.331-5.883 6.383-3.301-.1-5.804-2.878-5.911-6.164L12.202 12c0-3.436 2.637-6.384 5.907-6.384Zm0 1.268c-2.59 0-4.639 2.4-4.639 5.116.078 2.736 1.983 4.996 4.444 5.111l.195.004c2.583 0 4.623-2.406 4.623-5.115 0-2.752-2.086-5.116-4.623-5.116Zm.944 3.71c.507 0 1.1.662.702 1.473-.297.605-1.373 1.192-1.609 1.315l-.045.024s-1.32-.658-1.655-1.339c-.397-.811.196-1.473.703-1.473.56 0 .952.535.952.535s.391-.535.952-.535Z"
        /> < title > { title } < / title > < / svg >
    }
}
