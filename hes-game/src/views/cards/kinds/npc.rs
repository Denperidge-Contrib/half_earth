use super::super::card::*;
use crate::{
    consts,
    icons,
    t,
    views::{tip, HasTip},
    util::{get_element_if_exists, event_related_target},
};
use hes_engine::NPC;
use leptos::*;
use web_sys::FocusEvent;
use leptos::wasm_bindgen::JsCast;

#[component]
pub fn NPCCard(
    #[prop(into)] npc: Signal<NPC>,
) -> impl IntoView {
    let hearts = move || {
        with!(|npc| {
            (0..consts::MAX_RELATIONSHIP)
                .map(|i| {
                    let icon = if i as f32 <= npc.relationship {
                        icons::RELATIONSHIP
                    } else {
                        icons::RELATIONSHIP_EMPTY
                    };
                    view! { <img src=icon/> }
                })
                .collect::<Vec<_>>()
        })
    };

    let rel_tip = move || {
        with!(|npc| {
            tip(
                icons::RELATIONSHIP,
                t!("Your relationship with {name}. Increase it by implementing projects they like. At 5 hearts or more they will join your coalition.", name: t!(&npc.name)),
            )
        })
    };
    let portrait = move || {
        with!(|npc| {
            format!("/assets/characters/{}.webp", npc.name)
        })
    };
    let rel_icon = move || {
        with!(|npc| match npc.relationship_name() {
            "Ally" => icons::ALLY,
            "Friendly" => icons::FRIENDLY,
            "Nemesis" => icons::NEMESIS,
            "Neutral" => icons::NEUTRAL,
            _ => unreachable!(),
        })
    };
    let name = move || with!(|npc| t!(&npc.name));
    let rel_name =
        move || with!(|npc| t!(&npc.relationship_name()));
    let effects = move || {
        with!(|npc| {
            let effects = t!(&npc.flavor.effects);
            if npc.is_ally() {
                view! { <p class="npc-effect active" inner_html=effects></p> }.into_view()
            } else {
                let tip = tip(
                    icons::RELATIONSHIP,
                    t!("Improve your relationship with {name} to activate this ability.", name: t!(&npc.name)),
                );
                view! {
                    <HasTip tip>
                        <p class="npc-effect inactive" inner_html=effects></p>
                    </HasTip>
                }.into_view()
            }
        })
    };
    let description =
        move || with!(|npc| t!(&npc.flavor.description));
    let likes = move || with!(|npc| t!(&npc.flavor.likes));
    let dislikes =
        move || with!(|npc| t!(&npc.flavor.dislikes));
    
    let collapse = move |ev: FocusEvent| {
        event_related_target(ev).map(|target| {
            if target.class_name().contains("minicard") {
                get_element_if_exists(".minicard--expanded")
                    .map(|background| background.click());
            }
        });

    };

    view! {
        <Card class="npc" background="#724680" on:blur=collapse>
            <Header slot>
                <div>{t!("Parliament")}</div>
                <HasTip tip=rel_tip.into_signal()>
                    <span>{hearts}</span>
                </HasTip>
            </Header>
            <Figure slot>
                <img src=portrait/>
            </Figure>
            <Name slot>
                <span class="npc-tag">
                    <img src=rel_icon/>
                    {rel_name}
                </span>
                {name}
            </Name>
            <Body slot>{effects}</Body>
            <TopBack slot>
                <img src=portrait/>
                <p class="card-desc npc-desc">{description}</p>
            </TopBack>
            <BottomBack slot>
                <span class="likes-dislikes">
                    <span>
                        <h3>{t!("Likes")}</h3>
                        <p>{likes}</p>
                    </span>
                    <span>
                        <h3>{t!("Dislikes")}</h3>
                        <p>{dislikes}</p>
                    </span>
                </span>
            </BottomBack>
        </Card>
    }
}
