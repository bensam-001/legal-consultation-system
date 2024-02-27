#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LegalConsultation {
    id: u64,
    advisor_id: u64,
    details: String,
    created_at: u64,
    closed_at: Option<u64>,
    is_completed: bool,
}

impl Storable for LegalConsultation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LegalConsultation {
    const MAX_SIZE: u32 = 1024;  // Set an appropriate maximum size
    const IS_FIXED_SIZE: bool = false;  // Set to true if the size is fixed, otherwise false
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LegalAdvisor {
    id: u64,
    name: String,
    credentials: String,
    rating: f32,
}

impl Storable for LegalAdvisor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LegalAdvisor {
    const MAX_SIZE: u32 = 1024;  // Set an appropriate maximum size
    const IS_FIXED_SIZE: bool = false;  // Set to true if the size is fixed, otherwise false
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Feedback {
    client_id: u64,
    advisor_id: u64,
    rating: u8,
    comments: String,
}

impl Storable for Feedback {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Feedback {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static LEGAL_CONSULTATIONS: RefCell<StableBTreeMap<u64, LegalConsultation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static LEGAL_ADVISORS: RefCell<StableBTreeMap<u64, LegalAdvisor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static FEEDBACK: RefCell<StableBTreeMap<u64, Feedback, Memory>> =
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
));

}

#[ic_cdk::query]
fn get_legal_consultation(id: u64) -> Result<LegalConsultation, Error> {
    match _get_legal_consultation(&id) {
        Some(consultation) => Ok(consultation),
        None => Err(Error::NotFound {
            msg: format!("Legal consultation with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn initiate_legal_consultation(advisor_id: u64, details: String) -> Option<LegalConsultation> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let consultation = LegalConsultation {
        id,
        advisor_id,
        details,
        created_at: time(),
        closed_at: None,
        is_completed: false,
    };

    do_insert_legal_consultation(&consultation);
    Some(consultation)
}

#[ic_cdk::update]
fn update_legal_advisor(id: u64, name: String, credentials: String, rating: f32) -> Option<LegalAdvisor> {
    let advisor = LegalAdvisor {
        id,
        name,
        credentials,
        rating,
    };

    do_update_legal_advisor(&advisor);
    Some(advisor)
}

fn do_update_legal_advisor(advisor: &LegalAdvisor) {
    LEGAL_ADVISORS.with(|service| service.borrow_mut().insert(advisor.id, advisor.clone()));
}

fn do_insert_legal_consultation(consultation: &LegalConsultation) {
    LEGAL_CONSULTATIONS.with(|service| service.borrow_mut().insert(consultation.id, consultation.clone()));
}

fn _get_legal_consultation(id: &u64) -> Option<LegalConsultation> {
    LEGAL_CONSULTATIONS.with(|service| service.borrow().get(id))
}

#[ic_cdk::update]
fn delete_legal_consultation(id: u64) -> Result<(), Error> {
    if let Some(_) = _get_legal_consultation(&id) {
        LEGAL_CONSULTATIONS.with(|service| service.borrow_mut().remove(&id));
        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("Legal consultation with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn add_legal_advisor(name: String, credentials: String, rating: f32) -> Option<LegalAdvisor> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let advisor = LegalAdvisor {
        id,
        name,
        credentials,
        rating,
    };

    do_insert_legal_advisor(&advisor);
    Some(advisor)
}

#[ic_cdk::query]
fn get_legal_advisor(id: u64) -> Result<LegalAdvisor, Error> {
    match _get_legal_advisor(&id) {
        Some(advisor) => Ok(advisor),
        None => Err(Error::NotFound {
            msg: format!("Legal advisor with id={} not found", id),
        }),
    }
}

fn do_insert_legal_advisor(advisor: &LegalAdvisor) {
    LEGAL_ADVISORS.with(|service| service.borrow_mut().insert(advisor.id, advisor.clone()));
}

fn _get_legal_advisor(id: &u64) -> Option<LegalAdvisor> {
    LEGAL_ADVISORS.with(|service| service.borrow().get(id))
}

#[ic_cdk::update]
fn mark_consultation_as_completed(id: u64) -> Result<(), Error> {
    if let Some(consultation) = _get_legal_consultation(&id) {
        let mut updated_consultation = consultation.clone();
        updated_consultation.is_completed = true;
        LEGAL_CONSULTATIONS.with(|service| service.borrow_mut().insert(id, updated_consultation));
        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("Legal consultation with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn close_legal_consultation(id: u64, closed_at: u64) -> Result<(), Error> {
    if let Some(mut consultation) = _get_legal_consultation(&id) {
        consultation.closed_at = Some(closed_at);
        LEGAL_CONSULTATIONS.with(|service| service.borrow_mut().insert(id, consultation));
        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("Legal consultation with id={} not found", id),
        })
    }
}

#[ic_cdk::query]
fn list_all_legal_consultations() -> Vec<LegalConsultation> {
    LEGAL_CONSULTATIONS.with(|service| {
        let map_ref = service.borrow();
        map_ref.iter().map(|(_, v)| v.clone()).collect()
    })
}

#[ic_cdk::query]
fn list_all_legal_advisors() -> Vec<LegalAdvisor> {
    LEGAL_ADVISORS.with(|service| {
        let map_ref = service.borrow();
        map_ref.iter().map(|(_, v)| v.clone()).collect()
    })
}

#[ic_cdk::update]
fn update_legal_consultation(
    id: u64,
    advisor_id: Option<u64>,
    details: Option<String>,
    is_completed: Option<bool>,
) -> Result<(), Error> {
    if let Some(mut consultation) = _get_legal_consultation(&id) {
        // Update fields if provided
        if let Some(advisor_id) = advisor_id {
            consultation.advisor_id = advisor_id;
        }
        if let Some(details) = details {
            consultation.details = details;
        }
        if let Some(is_completed) = is_completed {
            consultation.is_completed = is_completed;
        }

        // Update the consultation in the map
        LEGAL_CONSULTATIONS.with(|service| service.borrow_mut().insert(id, consultation));
        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("Legal consultation with id={} not found", id),
        })
    }
}

#[ic_cdk::query]
fn search_legal_consultations(criteria: String) -> Vec<LegalConsultation> {
    LEGAL_CONSULTATIONS.with(|service| {
        let map_ref = service.borrow();
        map_ref
            .iter()
            .filter(|(_, consultation)| {
                consultation.details.contains(&criteria)
                // Add more search criteria as needed
            })
            .map(|(_, v)| v.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn search_legal_advisors(criteria: String) -> Vec<LegalAdvisor> {
    LEGAL_ADVISORS.with(|service| {
        let map_ref = service.borrow();
        map_ref
            .iter()
            .filter(|(_, advisor)| {
                advisor.name.contains(&criteria)
                // Add more search criteria as needed
            })
            .map(|(_, v)| v.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn sort_legal_consultations_by_date() -> Vec<LegalConsultation> {
    LEGAL_CONSULTATIONS.with(|service| {
        let map_ref = service.borrow();
        let mut consultations: Vec<LegalConsultation> = map_ref.iter().map(|(_, v)| v.clone()).collect();
        consultations.sort_by_key(|consultation| consultation.created_at);
        consultations
    })
}

#[ic_cdk::query]
fn sort_legal_advisors_by_rating() -> Vec<LegalAdvisor> {
    LEGAL_ADVISORS.with(|service| {
        let map_ref = service.borrow();
        let mut advisors: Vec<LegalAdvisor> = map_ref.iter().map(|(_, v)| v.clone()).collect();
        advisors.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
        advisors
    })
}

#[ic_cdk::query]
fn list_paginated_legal_consultations(page: usize, page_size: usize) -> Vec<LegalConsultation> {
    LEGAL_CONSULTATIONS.with(|service| {
        let map_ref = service.borrow();
        let consultations: Vec<LegalConsultation> = map_ref.iter().map(|(_, v)| v.clone()).collect();
        let start_index = page * page_size;
        consultations.iter().skip(start_index).take(page_size).cloned().collect()
    })
}

#[ic_cdk::query]
fn list_paginated_legal_advisors(page: usize, page_size: usize) -> Vec<LegalAdvisor> {
    LEGAL_ADVISORS.with(|service| {
        let map_ref = service.borrow();
        let advisors: Vec<LegalAdvisor> = map_ref.iter().map(|(_, v)| v.clone()).collect();
        let start_index = page * page_size;
        advisors.iter().skip(start_index).take(page_size).cloned().collect()
    })
}

#[ic_cdk::update]
fn provide_feedback(client_id: u64, advisor_id: u64, rating: u8, comments: String) -> Result<(), Error> {
    ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let feedback = Feedback {
        client_id,
        advisor_id,
        rating,
        comments,
    };

    do_insert_feedback(&feedback);
    Ok(())
}

fn do_insert_feedback(feedback: &Feedback) {
    let id = feedback.client_id; // Using client_id as the unique identifier

    FEEDBACK.with(|service| service.borrow_mut().insert(id, feedback.clone()));
}

#[ic_cdk::query]
fn get_feedback(id: u64) -> Result<Feedback, Error> {
    match _get_feedback(&id) {
        Some(feedback) => Ok(feedback),
        None => Err(Error::NotFound {
            msg: format!("Feedback with id={} not found", id),
        }),
    }
}

fn _get_feedback(id: &u64) -> Option<Feedback> {
    FEEDBACK.with(|service| service.borrow().get(id))
}

#[ic_cdk::update]
fn update_feedback(id: u64, new_rating: Option<u8>, new_comments: Option<String>) -> Result<(), Error> {
    if let Some(mut feedback) = _get_feedback(&id) {
        if let Some(rating) = new_rating {
            feedback.rating = rating;
        }
        if let Some(comments) = new_comments {
            feedback.comments = comments;
        }
        FEEDBACK.with(|service| service.borrow_mut().insert(id, feedback));
        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("Feedback with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn delete_feedback(id: u64) -> Result<(), Error> {
    if let Some(_) = _get_feedback(&id) {
        FEEDBACK.with(|service| service.borrow_mut().remove(&id));
        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("Feedback with id={} not found", id),
        })
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

ic_cdk::export_candid!();
