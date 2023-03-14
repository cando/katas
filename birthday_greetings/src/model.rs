#![allow(dead_code)]

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

pub struct Envelope<'a> {
    pub to: &'a Address,
    pub message: Message,
}

pub struct Message {
    pub subject: NonEmptyString,
    pub body: NonEmptyString,
}

trait EnvelopeDispatcher {
    type Repr<T>;
    fn send(msg: &Envelope) -> Self::Repr<Result<(), DispatchError>>;
}

pub struct SlackService();
impl EnvelopeDispatcher for SlackService {
    type Repr<T> = Box<dyn FnOnce() -> T>;

    fn send(_msg: &Envelope) -> Self::Repr<Result<(), DispatchError>> {
        Box::new(|| {
            // Do stuffs with slack
            Ok(())
        })
    }
}

pub struct EmailService();
impl EnvelopeDispatcher for EmailService {
    type Repr<T> = Box<dyn FnOnce() -> T>;

    fn send(_msg: &Envelope) -> Self::Repr<Result<(), DispatchError>> {
        Box::new(|| {
            // Do stuffs via email
            Ok(())
        })
    }
}

pub struct BirthdayService<'a> {
    employee_repository: Box<&'a dyn EmployeeRepository>,
}

impl<'a> BirthdayService<'a> {
    fn send_greetings<Sender>(self) -> Result<(), DispatchError>
    where
        Sender: EnvelopeDispatcher<
            Repr<Result<(), DispatchError>> = Box<dyn FnOnce() -> Result<(), DispatchError>>,
        >,
    {
        let employees = self
            .employee_repository
            .get_employees()
            .map_err(|e| DispatchError::GenericError(e))?;

        employees
            .iter()
            .map(|e| {
                let envelope = Envelope {
                    to: &e.address,
                    message: Message {
                        subject: NonEmptyString::new("ciao".to_owned()).unwrap(),
                        body: NonEmptyString::new("ciao".to_owned()).unwrap(),
                    },
                };

                Self::do_send::<Sender>(&envelope)()
            })
            .collect::<Result<Vec<()>, DispatchError>>()?;
        Ok(())
    }

    fn do_send<E>(msg: &Envelope) -> E::Repr<Result<(), DispatchError>>
    where
        E: EnvelopeDispatcher,
    {
        E::send(&msg)
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
            type Repr<T> = Box<dyn FnOnce() -> T>;
            fn send<'a>(msg: &Envelope<'a>) -> <tests::MockSlackService as EnvelopeDispatcher>::Repr<Result<(), DispatchError>>;
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

        let send_ctx = MockSlackService::send_context();
        send_ctx
            .expect()
            .times(1)
            .returning(|_| Box::new(|| Ok(())));

        let birthday_service = BirthdayService {
            employee_repository: Box::new(&employee_repository_mock),
        };

        assert!(birthday_service
            .send_greetings::<MockSlackService>()
            .is_ok())
    }
}
