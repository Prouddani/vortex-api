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
