use anyhow::Result;

mod witness_rep;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://0.0.0.0:14265";

/*     println!("Starting Examples");
    println!("---------------------------------------");
    println!("Single Publisher Examples");

    println!("\n---------------------------------------");
    println!("\nPublic - Single Branch - Single Publisher\n");
    examples::single_branch_public::example(url).await?; */

/*     println!("\n---------------------------------------");
    println!("\nPrivate - Single Branch - Single Publisher\n");
    examples::single_branch_private::example(url).await?; */

/*    println!("\n---------------------------------------");
    println!("\nPublic - Single Depth - Single Publisher\n");
    examples::single_depth_public::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nPrivate - Single Depth - Single Publisher\n");
    examples::single_depth_private::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nMixed - Multi Branch - Single Publisher\n");
    examples::multi_branch_mixed_privacy::example(url).await?;

    println!("\n---------------------------------------");
    println!("Multiple Publisher Examples");

    println!("\n---------------------------------------");
    println!("\nPrivate - Multi Branch - Single Publisher per Branch\n");
    examples::single_pub_per_branch::example(url).await?; */


    //let annoucement_msg = String::from("7ef5fbaa68868808b65154b311f0872cee627258ebca1ab90a9648849043034e0000000000000000:1d03be69bc0fbaee3b76e0d6");
    //let reliability = vec![1.0, 0.8, 1.0, 1.0, 1.0, 0.5, 0.5, 0.75, 0.5, 0.9, 1.0];
    let reliability = vec![1.0; 4];
    witness_rep::simulation::simulation(url, 4, 1.0, 2, 1, reliability).await?;


/* 
    println!("\n---------------------------------------");
    println!("Utility Examples");

    println!("\n---------------------------------------");
    println!("\nPrevious Message Retrieval\n");
    examples::fetch_prev::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nGranting and Revoking Access\n");
    examples::grant_and_revoke_access::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nUsing Public Keys for Keyload Generation\n");
    examples::pk_keyloads::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nUsing Pre Shared Keys for Keyload Generation\n");
    examples::psk_keyloads::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nState Recovery\n");
    examples::state_recovery::example(url).await?;

    println!("\n---------------------------------------");
    println!("\nStateless Recovery\n");
    examples::stateless_recovery::example(url).await?; */

    println!("\n---------------------------------------");
    println!("Examples Complete");

    Ok(())
}
