import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'Empty' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } };
export interface Feedback {
  'rating' : number,
  'comments' : string,
  'client_id' : bigint,
  'advisor_id' : bigint,
}
export interface LegalAdvisor {
  'id' : bigint,
  'name' : string,
  'credentials' : string,
  'rating' : number,
}
export interface LegalConsultation {
  'id' : bigint,
  'closed_at' : [] | [bigint],
  'created_at' : bigint,
  'is_completed' : boolean,
  'details' : string,
  'advisor_id' : bigint,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Feedback } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : LegalAdvisor } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : LegalConsultation } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_legal_advisor' : ActorMethod<
    [string, string, number],
    [] | [LegalAdvisor]
  >,
  'close_legal_consultation' : ActorMethod<[bigint, bigint], Result>,
  'delete_feedback' : ActorMethod<[bigint], Result>,
  'delete_legal_consultation' : ActorMethod<[bigint], Result>,
  'get_feedback' : ActorMethod<[bigint], Result_1>,
  'get_legal_advisor' : ActorMethod<[bigint], Result_2>,
  'get_legal_consultation' : ActorMethod<[bigint], Result_3>,
  'initiate_legal_consultation' : ActorMethod<
    [bigint, string],
    [] | [LegalConsultation]
  >,
  'list_all_legal_advisors' : ActorMethod<[], Array<LegalAdvisor>>,
  'list_all_legal_consultations' : ActorMethod<[], Array<LegalConsultation>>,
  'list_paginated_legal_advisors' : ActorMethod<
    [bigint, bigint],
    Array<LegalAdvisor>
  >,
  'list_paginated_legal_consultations' : ActorMethod<
    [bigint, bigint],
    Array<LegalConsultation>
  >,
  'mark_consultation_as_completed' : ActorMethod<[bigint], Result>,
  'provide_feedback' : ActorMethod<[bigint, bigint, number, string], Result>,
  'search_legal_advisors' : ActorMethod<[string], Array<LegalAdvisor>>,
  'search_legal_consultations' : ActorMethod<
    [string],
    Array<LegalConsultation>
  >,
  'sort_legal_advisors_by_rating' : ActorMethod<[], Array<LegalAdvisor>>,
  'sort_legal_consultations_by_date' : ActorMethod<
    [],
    Array<LegalConsultation>
  >,
  'update_feedback' : ActorMethod<
    [bigint, [] | [number], [] | [string]],
    Result
  >,
  'update_legal_advisor' : ActorMethod<
    [bigint, string, string, number],
    [] | [LegalAdvisor]
  >,
  'update_legal_consultation' : ActorMethod<
    [bigint, [] | [bigint], [] | [string], [] | [boolean]],
    Result
  >,
}
