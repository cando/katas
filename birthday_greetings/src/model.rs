#![allow(dead_code)]

use std::{convert::Infallible, ops::FromResidual};

use chrono::{NaiveDate, Utc};

pub struct NonEmptyString {
    inner: String,
}

#[derive(Debug)]
pub enum NonEmptyStringValidationError {
    EmptyName(String),
}

impl NonEmptyString {
    pub fn new(input: String) -> Result<NonEmptyString, NonEmptyStringValidationError> {
        match input.len() {
            0 => Err(NonEmptyStringValidationError::EmptyName(
                "specified string is invalid".to_owned(),
            )),
            _ => Ok(NonEmptyString { inner: input }),
        }
    }
}

type Name = NonEmptyString;

pub struct FullName {
    pub first_name: Name,
    pub last_name: Name,
}

pub struct Employee {
    pub name: FullName,
    pub address: Address,
    pub birth_date: BirthDate,
}

pub trait EmployeeRepository {
    fn get_employees(&self) -> Result<Vec<Employee>, String>;
}

#[derive(Clone)]
pub struct Email {
    inner: String,
}

pub enum EmailValidationError {
    InvalidFormat(String),
}

impl Email {
    pub fn new(input: String) -> Result<Email, EmailValidationError> {
        match input.len() {
            0 => Err(EmailValidationError::InvalidFormat(
                "specified email is invalid".to_owned(),
            )),
            _ => Ok(Email { inner: input }),
        }
    }
}

#[derive(Debug)]
pub struct BirthDate {
    inner: NaiveDate,
}

#[derive(Debug)]
pub enum BirthDateValidationError {
    InvalidFormat(String),
}

impl BirthDate {
    pub fn new(input: NaiveDate) -> Result<BirthDate, BirthDateValidationError> {
        if input > Utc::now().naive_utc().date() {
            Err(BirthDateValidationError::InvalidFormat(
                "date cannot be in the future".to_owned(),
            ))
        } else {
            Ok(BirthDate { inner: input })
        }
    }
}

pub enum DispatchError {
    GenericError(String),
}

#[derive(Clone)]
pub enum Address {
    Email(Email),
    Slack(String),
}

pub struct Envelope {
    pub to: Address,
    pub message: Message,
}

pub struct Message {
    pub subject: NonEmptyString,
    pub body: NonEmptyString,
}

trait EnvelopeDispatcher {
    type Repr<T>;
    fn prepare(employee: &Employee) -> Self::Repr<Envelope>;
    fn send(msg: Self::Repr<Envelope>) -> Self::Repr<()>;
}

pub struct SlackService();
impl EnvelopeDispatcher for SlackService {
    type Repr<T> = Result<T, String>;

    fn send(msg: Result<Envelope, String>) -> Result<(), String> {
        // Do stuffs with slack
        let _msg = msg?;
        Ok(())
    }

    fn prepare(e: &Employee) -> Result<Envelope, String> {
        let addr = e.address.clone();
        Ok(Envelope {
            to: addr,
            message: Message {
                subject: NonEmptyString::new("Happy birthday".to_owned()).unwrap(),
                body: NonEmptyString::new("Happy birthday".to_owned()).unwrap(),
            },
        })
    }
}

pub struct EmailService();
impl EnvelopeDispatcher for EmailService {
    type Repr<T> = Option<T>;

    fn send(msg: Option<Envelope>) -> Option<()> {
        // Do stuffs with slack
        let _msg = msg?;
        Some(())
    }

    fn prepare(e: &Employee) -> Option<Envelope> {
        let addr = e.address.clone();
        Some(Envelope {
            to: addr,
            message: Message {
                subject: NonEmptyString::new("Happy birthday".to_owned()).unwrap(),
                body: NonEmptyString::new("Happy birthday".to_owned()).unwrap(),
            },
        })
    }
}

pub struct BirthdayService<'a> {
    employee_repository: Box<&'a dyn EmployeeRepository>,
}

impl<'a> BirthdayService<'a> {
    fn send_greetings<E, R>(self) -> R
    where
        E: EnvelopeDispatcher,
        R: FromIterator<E::Repr<()>> + FromResidual<Result<Infallible, String>>,
    {
        self.employee_repository
            .get_employees()?
            .iter()
            .map(|e| E::send(E::prepare(e)))
            .collect::<R>()
    }
}

#[cfg(test)]
mod tests {
    use chrono::*;

    use mockall::mock;

    use crate::model::*;

    mock! {
        EmployeeRepository{}
        impl EmployeeRepository for EmployeeRepository {
            fn get_employees(&self) -> Result<Vec<Employee>, String>;
        }
    }

    mock! {
        SlackService{}
        impl EnvelopeDispatcher for SlackService {
            type Repr<T> = Result<T, String>;
            fn prepare(e: &Employee) -> <tests::MockSlackService as EnvelopeDispatcher>::Repr<Envelope>;
            fn send(msg: <tests::MockSlackService as EnvelopeDispatcher>::Repr<Envelope>) -> <tests::MockSlackService as EnvelopeDispatcher>::Repr<()>;
        }
    }

    mock! {
        EmailService{}
        impl EnvelopeDispatcher for EmailService {
            type Repr<T> = Option<T>;
            fn prepare(e: &Employee) -> <tests::MockEmailService as EnvelopeDispatcher>::Repr<Envelope>;
            fn send(msg: <tests::MockEmailService as EnvelopeDispatcher>::Repr<Envelope>) -> <tests::MockEmailService as EnvelopeDispatcher>::Repr<()>;
        }
    }

    #[test]
    fn end_to_end_should_send_greetings_with_dependency_injection() {
        let mut employee_repository_mock = MockEmployeeRepository::new();

        employee_repository_mock
            .expect_get_employees()
            .times(1)
            .returning(|| {
                Ok(vec![Employee {
                    address: Address::Slack("pippo".into()),
                    birth_date: BirthDate::new(NaiveDate::from_ymd_opt(2014, 7, 8).unwrap())
                        .unwrap(),
                    name: FullName {
                        first_name: Name::new("a".into()).unwrap(),
                        last_name: Name::new("b".into()).unwrap(),
                    },
                }])
            });

        let birthday_service = BirthdayService {
            employee_repository: Box::new(&employee_repository_mock),
        };

        let prepare_ctx = MockSlackService::prepare_context();
        prepare_ctx.expect().times(1).returning(|_| {
            Ok(Envelope {
                to: Address::Slack("pippo".into()),
                message: Message {
                    subject: NonEmptyString::new("ciao".to_owned()).unwrap(),
                    body: NonEmptyString::new("ciao".to_owned()).unwrap(),
                },
            })
        });

        let send_ctx = MockSlackService::send_context();
        send_ctx.expect().times(1).returning(|_| Ok(()));

        //
        // Open for extensions, closed for modification! We define target dispatcher and effect only at call site! Everything else does not change!
        //
        assert!(birthday_service
            .send_greetings::<MockSlackService, Result<Vec<()>, String>>() // <----------- MAGIC IS HERE!
            .is_ok());

        // --------------------------------------------------
        // OR if we inject a different service, it works!
        // let prepare_ctx = MockEmailService::prepare_context();
        // prepare_ctx.expect().times(1).returning(|_| {
        //     Some(Envelope {
        //         to: Address::Slack("pippo".into()),
        //         message: Message {
        //             subject: NonEmptyString::new("ciao".to_owned()).unwrap(),
        //             body: NonEmptyString::new("ciao".to_owned()).unwrap(),
        //         },
        //     })
        // });

        // let send_ctx = MockEmailService::send_context();
        // send_ctx.expect().times(1).returning(|_| Some(()));

        // assert!(birthday_service
        //     .send_greetings::<MockEmailService, Option<Vec<()>>>() // <----------- MAGIC IS HERE!
        //     .is_some())
    }
}
