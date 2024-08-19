// --------------------------------------------
//      Sized and Optionally Sized Trait
// --------------------------------------------

/*
    The sized trait is served as both:

    - Auto Trait (Send, Sync etc)
        special category of traits that the compiler automatically implements 
        for types based on certain criteria. These traits do not require the user 
        to explicitly implement them like Copy, Clone, Default, etc.,
        as the compiler derives their implementation by default 
        when the type satisfies specific conditions. 

        All auto traits are also marker traits.

    - Marker Trait
        special type of trait that does not define any methods or associated items. 
        Instead, it serves as a "marker" or "flag" to indicate some property or capability 
        of a type. Marker traits are often used by the compiler or other traits to enforce 
        certain behaviors or optimizations at compile time.
*/

use negative_impl::negative_impl;

/*
    ABC cannot be transferred across threads (Send) and 
    cannot be accessed from multiple threads simultaneously (Sync).
*/
struct ABC;
#[negative_impl]
impl !Send for ABC {}               // Opt out
#[negative_impl]
impl !Sync for ABC {}               // Opt out

/*
    negative_impl not support for non-auto traits.
    Sized is a special trait that the compiler automatically 
    assumes for most types unless explicitly opted out.
*/
// #[negative_impl]
// impl !Sized for ABC {}

// fn some_fn<T>(t: T) {}           // Implicit Sized Bound.
// fn some_fn<T: Sized>(t: &T) {}   // Explicit Sized Bound.

// This function can accept both `sized` and unsized types.
fn some_fn<T: ?Sized>(t: &T) {}

fn main() {
    let x: i32 =  Default::default();
}