## `vortex-api` Rust crate

Hello vortexians!
This is a crate used for accessing the Vortex API. For example, you could get the friends list or an user's bio!

> IMPORTANT: Webscraping is not allowed, according to Vortex's TOS.
> 
> Make sure you always have permission before accessing users' information!

```rust
use vortex_api::{get_friends_from_id, get_user_from_id};

#[tokio::main]
async fn main() -> Result<(), String> {
    let halo = get_user_from_id(1).await?;
    let halo_friends = get_friends_from_id(1).await?;

    // get halo's information
    println!("Halo information:");
    println!("{}", halo);

    println!("----------");

    // iterate through halo's friends
    for friend in halo_friends {
        println!("{}\n", friend)
    }
    Ok(())
}
```
