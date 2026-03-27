#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec, Address, Map};

#[contract]
pub struct EventRSVP;

#[contractimpl]
impl EventRSVP {

    // Create an event
    pub fn create_event(env: Env, event_id: Symbol, creator: Address) {
        creator.require_auth();

        let mut events: Map<Symbol, Address> = env.storage().instance().get(&symbol_short!("EVENTS")).unwrap_or(Map::new(&env));

        events.set(event_id.clone(), creator);
        env.storage().instance().set(&symbol_short!("EVENTS"), &events);

        // Initialize RSVP list
        let rsvp_list: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&event_id, &rsvp_list);
    }

    // RSVP to an event
    pub fn rsvp(env: Env, event_id: Symbol, user: Address) {
        user.require_auth();

        let mut rsvp_list: Vec<Address> = env.storage().instance().get(&event_id).unwrap();

        // Prevent duplicate RSVP
        if !rsvp_list.contains(&user) {
            rsvp_list.push_back(user);
        }

        env.storage().instance().set(&event_id, &rsvp_list);
    }

    // Get RSVPs
    pub fn get_rsvps(env: Env, event_id: Symbol) -> Vec<Address> {
        env.storage().instance().get(&event_id).unwrap_or(Vec::new(&env))
    }
}