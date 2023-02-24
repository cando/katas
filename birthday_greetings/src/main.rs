/*
== Problem: write a program that
Loads a set of employee records from a flat file
Sends a greetings email to all employees whose birthday is today
The flat file is a sequence of records, separated by newlines; this are the first few lines:

    last_name, first_name, date_of_birth, email
    Doe, John, 1982/10/08, john.doe@foobar.com
    Ann, Mary, 1975/09/11, mary.ann@foobar.com
    The greetings email contains the following text:

    Subject: Happy birthday!

    Happy birthday, dear John!

with the first name of the employee substituted for “John”

The program should be invoked by a main program like this one:

public static void main(String[] args) {
    ...
    BirthdayService birthdayService = new BirthdayService(
        employeeRepository, emailService);
    birthdayService.sendGreetings(today());
}
Note that the collaborators of the birthdayService objects are injected in it.
Ideally domain code should never use the new operator.
The new operator is called from outside the domain code, to set up an aggregate of objects that collaborate together.

== Goals
The goal of this exercise is to come up with a solution that is

* Testable; we should be able to test the internal application logic with no need to ever send a real email.
* Flexible: we anticipate that the data source in the future could change from a flat file to a relational database, or perhaps a web service. We also anticipate that the email service could soon be replaced with a service that sends greetings through Facebook or some other social network.
* Well-designed: separate clearly the business logic from the infrastructure.
*/

fn main() {
    println!("Hello, world!");
}
