#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 4.2: References and Borrowing
///
/// Borrowing allows you to grant access to data without transferring ownership.
/// This file explores immutable references (`&T`), mutable references (`&mut T`),
/// the fundamental borrowing rules (aliasing vs. mutation), Non-Lexical Lifetimes (NLL),
/// and how Rust prevents dangling references at compile time.
fn main() {
    immutable_references();
    mutable_references();
    borrowing_rules_and_aliasing();
    non_lexical_lifetimes();
    dangling_references_prevention();
}

/// # Immutable References (`&T`)
/// - Creating a reference is called **borrowing**.
/// - An immutable reference grants read-only access to the data.
/// - The owner retains ownership, so the data is NOT dropped when the borrowing function ends.
/// - You can create multiple immutable references to the same data simultaneously.
fn immutable_references() {
    println!("\n{:=>80}", "");
    println!("immutable_references()\n");

    let network_config = String::from("SSID=LabNet;Security=WPA3;IP=192.168.1.50");

    // We pass `&network_config` (a reference) instead of moving `network_config`.
    let ssid = extract_ssid(&network_config);

    // Because `network_config` was only borrowed, it is still valid and owned by this scope!
    println!("Original config remains accessible: {network_config}");
    println!("Extracted SSID: {ssid}");

    // You cannot modify data through an immutable reference.
    // try_modify_config(&network_config); // See function definition below.
}

/// # Mutable References (`&mut T`)
/// - A mutable reference grants both read and write access to borrowed data.
/// - The underlying variable MUST be declared as mutable (`let mut`).
/// - To pass a mutable reference, use `&mut variable_name`.
fn mutable_references() {
    println!("\n{:=>80}", "");
    println!("mutable_references()\n");

    // Variable must be mutable to allow mutable borrowing later.
    let mut system_log = String::from("10:00:01 [INFO] Service started");

    println!("Log before mutation: {system_log}");

    // We pass a mutable reference `&mut system_log` to allow the function to edit the String.
    append_timestamp(&mut system_log, "10:05:22 [WARN] High memory usage");

    // The original variable reflects the changes made during the borrow.
    println!("Log after mutation:  {system_log}");
}

/// # Borrowing Rules and Aliasing
/// The Golden Rule of References:
/// At any given time, you can have EITHER:
/// - Any number of immutable references (`&T`), OR
/// - Exactly ONE mutable reference (`&mut T`).
///
/// This rule prevents **data races** at compile time! A data race occurs when:
/// 1. Two or more pointers access the same memory location simultaneously.
/// 2. At least one of the pointers is being used to write to the memory.
/// 3. There is no synchronization mechanism used to control access.
fn borrowing_rules_and_aliasing() {
    println!("\n{:=>80}", "");
    println!("borrowing_rules_and_aliasing()\n");

    let mut database_uri = String::from("postgres://admin:secret@localhost:5432/production");

    // --- Case 1: Multiple Immutable References (ALLOWED) ---
    let reader1 = &database_uri;
    let reader2 = &database_uri;
    println!("Reader 1: {reader1}");
    println!("Reader 2: {reader2}");
    // Multiple readers can safely access data because none of them can alter it.

    // --- Case 2: Combining Immutable and Mutable References (DISALLOWED) ---
    let ref_read = &database_uri;
    // let ref_write = &mut database_uri; // Uncomment to see compile error.

    // If `ref_write` could mutate the String while `ref_read` is holding a reference,
    // reallocation on the heap could invalidate `ref_read`'s pointer.
    // println!("Read: {ref_read}, Write: {ref_write}");

    // --- Case 3: Multiple Mutable References in the Same Scope (DISALLOWED) ---
    let mut_writer1 = &mut database_uri;
    // let mut_writer2 = &mut database_uri; // Uncomment to see compile error.

    // println!("{mut_writer1}, {mut_writer2}");
}

/// # Non-Lexical Lifetimes (NLL)
/// - Historically (in older Rust), a reference's scope lasted until the closing `}` brace.
/// - Modern Rust compiler uses Non-Lexical Lifetimes (NLL): a reference's scope ends at
///   its **last point of use**, rather than at the end of the enclosing block.
/// - This makes the borrow checker much more intuitive and flexible.
/// - Lifetimes will be discussed further later on the course.
fn non_lexical_lifetimes() {
    println!("\n{:=>80}", "");
    println!("non_lexical_lifetimes()\n");

    let mut queue_status = String::from("Status: Pending");

    // `reader` borrows `queue_status` immutably.
    let reader = &queue_status;
    println!("Current status: {reader}");
    // <--- `reader` is used for the LAST time on the line above!
    // Under NLL, the immutable borrow ends HERE, not at the end of the function block.

    // Because `reader` is never used again, we can now take a mutable reference!
    let writer = &mut queue_status;
    writer.push_str(" -> Processing");
    println!("Updated status: {queue_status}");
}

/// # Dangling References Prevention
/// - A dangling reference is a pointer that references a location in memory that
///   may have been given to someone else (or freed).
/// - In languages like C/C++, returning a pointer to local stack memory causes undefined behavior.
/// - Rust guarantees that references will never dangle: the compiler ensures the data
///   will not go out of scope before the reference does.
fn dangling_references_prevention() {
    println!("\n{:=>80}", "");
    println!("dangling_references_prevention()\n");

    // We call a function that correctly transfers ownership instead of returning a reference.
    let valid_string = create_valid_string();
    println!("Safely received owned string: {valid_string}");

    // Attempting to create and return a reference to local memory fails at compile time:
    // let dangling_ref = create_dangling_reference(); // See commented helper below.
}

// ========================================================================= //
// NOTE! Helper functions demonstrating reference rules and signatures.
// ========================================================================= //

/// Reads from a borrowed `String` reference without taking ownership.
fn extract_ssid(config: &String) -> &str {
    // Slices are covered in section 4.3, but notice we can read `config` freely.
    if let Some(start) = config.find("SSID=") {
        let rest = &config[start + 5..];
        if let Some(end) = rest.find(';') {
            return &rest[..end];
        }
    }
    "UNKNOWN"
}

/// Helper function showing that modifying an immutable reference is rejected by compiler.
fn try_modify_config(config: &String) {
    // config.push_str(";Debug=true"); // Uncomment to see error: cannot borrow immutable as mutable!
}

/// Takes a mutable reference (`&mut String`) and modifies the referenced heap object directly.
fn append_timestamp(log: &mut String, new_entry: &str) {
    log.push_str("\n");
    log.push_str(new_entry);
}

/// Safe alternative: Instead of returning a reference to data created inside the function,
/// we return the `String` itself, moving ownership out to the caller.
fn create_valid_string() -> String {
    let s = String::from("Locally allocated string");
    s // Ownership moved out.
}

// Attempting to return a reference to data owned by the current function will fail to compile.
// Un-commenting this will trigger: "returns a value referencing data owned by the current function".
// fn create_dangling_reference() -> &String {
//     let s = String::from("I am short-lived");
//     &s
// } // `s` goes out of scope and is dropped HERE! Returning `&s` would point to deallocated memory.
