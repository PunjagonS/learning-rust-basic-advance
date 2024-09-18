/*  -----------------------------------------------------
    Longest Non-Stop Working
        Description
        - Given time slots numbers, we want to determine
          the longest consecutive time.

        Tools
        - Hashset
        - Vectors
        - Loops
    -----------------------------------------------------
*/

use std::collections::HashSet;

fn find_longest_busy_time(working_slots: Vec<Vec<u8>>) -> u8 {
    let mut employee_longest_non_stop_working = Vec::new();

    for i in working_slots {
        employee_longest_non_stop_working.push(find_longest_period(i));
    }

    // for i in 0..employee_longest_non_stop_working.len() {
    for (i, _) in employee_longest_non_stop_working.iter().enumerate() {
        println!(
            "Employee Number {} has worked nonstop for {} slots",
            i, employee_longest_non_stop_working[i]
        );
    }

    let max_value = employee_longest_non_stop_working.iter().max();
    employee_longest_non_stop_working
        .iter()
        .position(|x| *x == *max_value.unwrap())
        .unwrap() as u8
}

fn find_longest_period(working_slots: Vec<u8>) -> u8 {
    let mut longest_busy_period = 0;

    // Convert the working_slots to a HashSet to remove duplicates.
    let slot_set = working_slots.into_iter().collect::<HashSet<_>>();

    for slot in &slot_set {
        /*
            This if statement is to check if the current slot is the start of a consecutive sequence.
            If the current slot is not the start of a consecutive sequence,
            the while loop internal will not be executed.
        */
        // used reference `&(slot - 1)` because `slot` is a reference.
        if !slot_set.contains(&(slot - 1)) {
            /*
                `.to_owned()` is to create an owned version of a value from a reference
                or from borrowed type.
            */
            let mut current_slot = slot.to_owned();
            let mut current_consecutive_slot = 1;

            /*
                This while loop is to find the longest consecutive slots
                by checking if the next slot(current_slot + 1) is in the slot_set
                and keep adding 1 to the current_consecutive_slot.
                if current_consecutive_slot is greater than longest_busy_period,
                update longest_busy_period.
            */
            while slot_set.contains(&(current_slot + 1)) {
                current_slot += 1;
                current_consecutive_slot += 1;
            }
            if current_consecutive_slot > longest_busy_period {
                longest_busy_period = current_consecutive_slot;
            }
        }
    }
    longest_busy_period
}

fn main() {
    let schedule = vec![
        /*
            Number in arrays represent the working time slots.
            For example employee one has 4 busy periods:
                -> 1 - 2 (1,2)
                -> 4 - 6 (4,5,6)
                -> 8
                -> 10 - 11 (10,11)
            So longest non-stop working period of employee one is 3 slots(4,5,6).
        */
        vec![4, 1, 2, 5, 6, 8, 10, 11],  // Employee index 0
        vec![3, 1, 2, 5, 7, 10, 11, 14], // Employee index 1
        vec![3, 1, 15, 5, 13, 12, 10, 14, 15, 16, 17, 18, 8, 9], // Employee index 2
    ];

    println!(
        "Employee number {} has the highest number of non-stop working slots",
        find_longest_busy_time(schedule)
    );
}
