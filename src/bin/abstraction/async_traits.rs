// --------------------------------------------
//       Using Async Functions in Traits
// --------------------------------------------

/*
    Before the 2024 update, we had to use an `associated type` to specify
    the type of Future would return. This required using `Pin<Box<dyn Future>>`
    to handle the Future inside the async block, making the code
    more complex than using `async fn` directly within traits.
*/

use std::future::Future;

// New update
use async_trait::async_trait;

struct Authenticator;

// Trait for user authentication, using an associated type for Future
trait Authentication {
    type AuthFuture: Future<Output = bool>; // Define associated type for Future
    fn authenticate(&self, username: &str, password: &str) -> Self::AuthFuture;
}

// New update
#[async_trait]
trait AuthenticationAsync {
    async fn authenticate(&self, username: &str, password: &str) -> bool;
}

impl Authentication for Authenticator {
    type AuthFuture = std::pin::Pin<Box<dyn Future<Output = bool> + Send>>;

    fn authenticate(&self, username: &str, password: &str) -> Self::AuthFuture {
        // Use Box to wrap the Future and use async block to create the Future
        let username = username.to_string();
        let password = password.to_string();

        Box::pin(async move {
            println!("Authenticating user '{}' with database...", username);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            // Return true or false based on the specified conditions
            username == "admin" && password == "password123"
        })
    }
}

// New update
#[async_trait]
impl AuthenticationAsync for Authenticator {
    async fn authenticate(&self, username: &str, password: &str) -> bool {
        println!("Authenticating user '{}' with database...", username);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        username == "admin" && password == "password123"
    }
}

async fn authen<A: Authentication>(authenticator: &A, username: &str, password: &str)
where
    A::AuthFuture: Send + 'static,
{
    let result = authenticator.authenticate(username, password).await;
    if result {
        println!("User '{}' authenticated successfully!", username);
    } else {
        println!("Authentication failed for user '{}'", username);
    }
    println!()
}

// New update
async fn authen_async<A: AuthenticationAsync>(authenticator: &A, username: &str, password: &str) {
    let result = authenticator.authenticate(username, password).await;
    if result {
        println!("User '{}' authenticated successfully!", username);
    } else {
        println!("Authentication failed for user '{}'", username);
    }
}

#[tokio::main]
async fn main() {
    let authenticator = Authenticator;
    authen(&authenticator, "admin", "password123").await;
    authen(&authenticator, "user", "wrong_password").await;

    // After applying async_trait
    authen_async(&authenticator, "admin", "password123").await;
    authen_async(&authenticator, "user", "wrong_password").await;
}
