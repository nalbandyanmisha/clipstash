// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug, Copy, Clone)]
struct LuggageId(usize);

#[derive(Debug)]
struct Luggage(LuggageId);

#[derive(Debug)]
struct CheckIn(LuggageId);

#[derive(Debug)]
struct OnLoading(LuggageId);

#[derive(Debug)]
struct OffLoading(LuggageId);

#[derive(Debug)]
struct AwaitingPickup(LuggageId);

#[derive(Debug)]
struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Self(id)
    }

    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn on_load(self) -> OnLoading {
        OnLoading(self.0)
    }
}

impl OnLoading {
    fn off_loading(self) -> OffLoading {
        OffLoading(self.0)
    }
}

impl OffLoading {
    fn awaiting_pickup(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn end_custody(self) -> EndCustody {
        EndCustody(self.0)
    }
}

fn main() {
    let luggage_id = LuggageId(432423);
    let luggage = Luggage::new(luggage_id);
    println!(
        "{:?}",
        luggage
            .check_in()
            .on_load()
            .off_loading()
            .awaiting_pickup()
            .end_custody()
    );
}
