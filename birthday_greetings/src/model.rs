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
    fn get_employees(self) -> Result<Vec<Employee>, ()>;
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

// pub trait Dispatchable {
//    fn send(envelop: Envelope) -> Result<(), DispatchError>;
// }

// impl Dispatchable for EmailService {
//     fn send(envelop: Envelop) -> Result<(), DispatchError> {

//     }
// }

// pub struct Envelope{
//     pub to: Box<dyn Dispatchable>,
//     pub message: Message
// }

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

pub trait DispatcherService {
    fn send(&self, envelope: &Envelope) -> Result<(), String>;
    fn can_send(&self, envelope: &Envelope) -> bool;
}
pub struct EmailService {}

impl DispatcherService for EmailService {
    fn send(&self, envelope: &Envelope) -> Result<(), String> {
        Ok(())
    }

    fn can_send(&self, envelope: &Envelope) -> bool {
        matches!(envelope.to, Address::Email(_))
    }
}

pub struct SlackService {}

impl DispatcherService for SlackService {
    fn send(&self, envelope: &Envelope) -> Result<(), String> {
        Ok(())
    }

    fn can_send(&self, envelope: &Envelope) -> bool {
        matches!(envelope.to, Address::Slack(_))
    }
}

pub struct BirthdayService {
    employee_repository: Box<dyn EmployeeRepository>,
    dispatchers: Vec<Box<dyn DispatcherService>>,
}

impl BirthdayService {
    fn send_greetings(self) -> Result<(), ()> {
        let employees = self.employee_repository.get_employees()?;

        employees.iter().for_each(|e| {
            let envelope = Envelope {
                to: e.address,
                message: Message {
                    subject: NonEmptyString::new("ciao".to_owned()).unwrap(),
                    body: NonEmptyString::new("ciao".to_owned()).unwrap(),
                },
            };
            
            let a = self.dispatchers.into_iter()
            .filter(|d| d.can_send(&envelope))
            .collect::<Vec<Box<dyn DispatcherService>>>().get(0).map(|dispatcher|dispatcher.send(&envelope));
        });
        Ok(())
    }
}
