use leptos::{expect_context, RwSignal, SignalUpdate, Trigger};

use crate::canister::backend::CarDetails;

#[derive(Clone, Default)]
pub struct CheckoutState {
    pub selected_car: RwSignal<Option<CarDetails>>,
    pub start_time: RwSignal<Option<u64>>,
    pub end_time: RwSignal<Option<u64>>,
}

#[derive(Clone, Default)]
pub struct CheckoutUser {
    pub user: RwSignal<UserDetials>,
    pub on_form_reset: Trigger,
}

#[derive(Clone, Default)]
pub struct UserDetials {
    pub name: Option<String>,
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub mobile_number: Option<String>,
    pub age: Option<u8>,
    pub pan: Option<String>, 
    pub aadhar: Option<String,>
}

fn check_is_some_and_not_empty(val: &Option<String> ) -> bool {
    val.is_some() && val.as_ref().unwrap().len() > 0
}

impl UserDetials {
    pub fn check_ready(&self) -> bool {
        check_is_some_and_not_empty(&self.name)  &&  check_is_some_and_not_empty(&self.email) && check_is_some_and_not_empty(&self.country_code) && self.age.is_some() && check_is_some_and_not_empty(&self.pan) && check_is_some_and_not_empty(&self.aadhar)
    }

    
}

impl CheckoutState {
    pub fn get() -> Self {
        let this: Self = expect_context();
        this
    }

    pub fn set(car: CarDetails) {
        let this: Self = expect_context();

        this.selected_car.update(|f| *f = Some(car));
        //2024-10-01 08:35:05
        this.start_time.update(|f| *f = Some(1727771705));
        //2024-10-03 12:35:05
        this.end_time.update(|f| *f = Some(1727958905));
    }

    pub fn clear() {
        let this: Self = expect_context();
        this.selected_car.update(|f| *f = None);
    }
}
