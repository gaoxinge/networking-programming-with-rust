use futures::future;
use futures::join;
use futures::select;
use futures::executor::block_on;

async fn learn_song() -> &'static str {
    let song = "music";
    println!("learn song: {}", song);
    song
}

async fn sing_song(song: &str) {
    println!("sing song: {}", song);
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main1() {
    learn_and_sing().await;
    dance().await;
}

async fn async_main2() {
    join!(learn_and_sing(), dance());
}

fn main3() {
    let mut a = future::ready(4);
    let mut b = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            value = a => total += value,
            value = b => total += value,
            complete => break,
            default => unreachable!(),
        }
    }
    
    println!("total: {}", total);

}

fn main() {
    println!("======== 1 ========");
    block_on(async_main1());

    println!("======== 2 ========");
    block_on(async_main2());

    println!("======== 3 ========");
    main3();
}
