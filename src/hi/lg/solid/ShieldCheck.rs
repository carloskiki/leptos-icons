#[cfg(feature = "HiLgSolidShieldCheck")]
use leptos::*;
#[cfg(feature = "HiLgSolidShieldCheck")]
///This icon requires the feature `HiLgSolidShieldCheck` to be enabled.
#[component]
pub fn ShieldCheck(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.5157 2.1698C12.2265 1.89587 11.7735 1.89587 11.4843 2.1698C9.46752 4.07977 6.74624 5.25011 3.75 5.25011C3.70233 5.25011 3.65473 5.24981 3.60721 5.24922C3.27984 5.24515 2.98767 5.4539 2.88541 5.76491C2.47287 7.01968 2.25 8.35963 2.25 9.75015C2.25 15.6922 6.31406 20.6831 11.8131 22.0984C11.9357 22.13 12.0643 22.13 12.1869 22.0984C17.6859 20.6831 21.75 15.6922 21.75 9.75015C21.75 8.35963 21.5271 7.01968 21.1146 5.76491C21.0123 5.4539 20.7202 5.24515 20.3928 5.24922C20.3453 5.24981 20.2977 5.25011 20.25 5.25011C17.2538 5.25011 14.5325 4.07977 12.5157 2.1698ZM15.6103 10.1859C15.8511 9.84887 15.773 9.38046 15.4359 9.1397C15.0989 8.89894 14.6305 8.97701 14.3897 9.31407L11.1543 13.8436L9.53033 12.2197C9.23744 11.9268 8.76256 11.9268 8.46967 12.2197C8.17678 12.5126 8.17678 12.9874 8.46967 13.2803L10.7197 15.5303C10.8756 15.6862 11.0921 15.7656 11.3119 15.7474C11.5316 15.7293 11.7322 15.6153 11.8603 15.4359L15.6103 10.1859Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
