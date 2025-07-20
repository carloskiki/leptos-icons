use icondata as i;
use leptos::prelude::*;
use leptos_icons::Symbol;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div>
                <Symbol id="ai_carry_out_twotone" icon={i::AiCarryOutTwotone} />
                <Symbol id="bi_graphql" icon={i::BiGraphql} style="color: orange" />
                <Symbol id="bs1_circle" icon={i::Bs1Circle} style="color: red" />
                <Symbol id="fa_bars_solid" icon={i::FaBarsSolid} />
                <Symbol id="oc_alert_sm" icon={i::OcAlertSm} />
                <Symbol id="oc_alert_lg" icon={i::OcAlertLg} />

                <div>
                    "Reference the `id` assigned in "
                    <code>"<Symbol>"</code>
                    " in a " <code>"<use>"</code>
                    " element to display the icon."

                    <div style="color: #8f39d3;">
                        {(0..2).map(|_| view!{ <IconView /> }).collect::<Vec<_>>()}
                    </div>
                </div>
                <div>
                    "Use the " <code>"template!"</code> " macro 
                    (from "<code>"leptos"</code>") for even faster rendering."
                    <div style="color: #398fd3;">
                        {(0..10).map(|_| view!{ <IconTemplate /> }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        }
    });
}

#[component]
fn IconView() -> impl IntoView {
    view! {
        <div>
            <svg width="1em" height="1em">
                <use href="#ai_carry_out_twotone"/>
            </svg>
            <svg width="1em" height="1em">
                <use href="#bs1_circle" />
            </svg>
            <svg width="1em" height="1em">
                <use href="#fa_bars_solid"/>
            </svg>
            <svg width="64" height="1em">
                <use href="#bi_graphql"/>
                <use href="#oc_alert_sm" x="16"/>
            </svg>
            <svg width="2em" height="2em">
                <use href="#oc_alert_lg" />
            </svg>
        </div>
    }
}

#[component]
fn IconTemplate() -> impl IntoView {
    template! {
        <div>
            <svg width="1em" height="1em">
                <use href="#ai_carry_out_twotone"/>
            </svg>
            <svg width="1em" height="1em">
                <use href="#bs1_circle" />
            </svg>
            <svg width="1em" height="1em">
                <use href="#fa_bars_solid"/>
            </svg>
            <svg width="64" height="1em">
                <use href="#bi_graphql"/>
                <use href="#oc_alert_sm" x="16"/>
            </svg>
            <svg width="2em" height="2em">
                <use href="#oc_alert_lg" />
            </svg>
        </div>
    }
}