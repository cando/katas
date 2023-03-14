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

pub struct BirthDate {
    inner: NaiveDate,
}

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

pub struct SlackService();

trait Ops {
    type Repr<T>;
    fn send(msg: &Envelope) -> Self::Repr<Result<(), DispatchError>>;
}

impl Ops for SlackService {
    type Repr<T> = Box<dyn FnOnce() -> T>;

    fn send(_msg: &Envelope) -> Self::Repr<Result<(), DispatchError>> {
        Box::new(|| {
            // Do stuffs with envelope
            Ok(())
        })
    }
}

pub struct BirthdayService {
    employee_repository: Box<&'static dyn EmployeeRepository>,
}

impl BirthdayService {
    fn send_greetings<Sender>(self) -> Result<(), DispatchError>
    where
        Sender:
            Ops<Repr<Result<(), DispatchError>> = Box<dyn FnOnce() -> Result<(), DispatchError>>>,
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

                Self::send_op::<Sender>(&envelope)()
            })
            .collect::<Result<Vec<()>, DispatchError>>()?;
        Ok(())
    }

    fn send_op<E>(msg: &Envelope) -> E::Repr<Result<(), DispatchError>>
    where
        E: Ops,
    {
        E::send(msg)
    }
}
