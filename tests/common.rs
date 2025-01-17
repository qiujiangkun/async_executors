#![allow(dead_code)]
//
pub use {
    async_executors::*,
    futures::{
        channel::{mpsc::Sender, oneshot},
        executor::block_on,
        FutureExt, SinkExt,
    },
    std::{rc::Rc, sync::Arc},
};

async fn sum(a: u8, b: u8, mut tx: Sender<u8>) {
    let res = tx.send(a + b).await;

    assert!(res.is_ok());
}

async fn sum_local(a: u8, b: u8, mut tx: Sender<u8>) {
    // make sure we have something !Send in here.
    //
    let a = Rc::new(a);

    let res = tx.send(*a + b).await;

    assert!(res.is_ok());
}

async fn sum_handle(a: u8, b: u8) -> u8 {
    a + b
}

async fn sum_handle_local(a: u8, b: u8) -> Rc<u8> {
    // Return something !Send
    //
    Rc::new(a + b)
}

// A function that takes a generic executor and spawns a task.
//
pub fn increment(a: u8, exec: impl Spawn, tx: Sender<u8>) {
    let res = exec.spawn(sum(a, 1, tx));

    assert!(res.is_ok());
}

// A function that takes a generic executor and spawns a task.
//
#[allow(dead_code)] // gives warning when testing all executors at once.
                    //
pub fn increment_local(a: u8, exec: impl LocalSpawn, tx: Sender<u8>) {
    let res = exec.spawn_local(sum(a, 1, tx));

    assert!(res.is_ok());
}

// A function that takes a generic executor and spawns a task.
//
pub fn increment_ref(a: u8, exec: &impl Spawn, tx: Sender<u8>) {
    let res = exec.spawn(sum(a, 1, tx));

    assert!(res.is_ok());
}

// A function that takes a generic executor and spawns a task.
//
#[allow(dead_code)] // gives warning when testing all executors at once.
                    //
pub fn increment_ref_local(a: u8, exec: &impl LocalSpawn, tx: Sender<u8>) {
    let res = exec.spawn_local(sum(a, 1, tx));

    assert!(res.is_ok());
}

// A function that takes a generic executor by value, clones it and spawns a task.
//
pub fn increment_clone(a: u8, exec: impl Spawn + Clone, tx: Sender<u8>) {
    let second = exec.clone();
    drop(exec);

    let res = second.spawn(sum(a, 1, tx));

    assert!(res.is_ok());
}

// A function that takes a generic executor by value, clones it and spawns a task.
//
#[allow(dead_code)] // gives warning when testing all executors at once.
                    //
pub fn increment_clone_local(a: u8, exec: impl LocalSpawn + Clone, tx: Sender<u8>) {
    let second = exec.clone();
    drop(exec);

    let res = second.spawn_local(sum(a, 1, tx));

    assert!(res.is_ok());
}

// A function that takes a generic executor and spawns a task.
//
#[allow(dead_code)]
//
pub async fn increment_spawn_handle(a: u8, exec: impl SpawnHandle<u8>) -> u8 {
    exec.spawn_handle(sum_handle(a, 1))
        .expect("spawn handle")
        .await
}

// A function that takes a generic executor and spawns a task.
//
#[allow(dead_code)] // gives warning when testing all executors at once.
                    //
pub async fn increment_spawn_handle_local(a: u8, exec: impl LocalSpawnHandle<Rc<u8>>) -> Rc<u8> {
    exec.spawn_handle_local(sum_handle_local(a, 1))
        .expect("spawn handle")
        .await
}

// A function that takes a trait object and spawns a task.
//
#[allow(dead_code)]
//
pub async fn increment_spawn_handle_os(a: u8, exec: &dyn SpawnHandle<u8>) -> u8 {
    exec.spawn_handle(sum_handle(a, 1).boxed())
        .expect("spawn handle")
        .await
}

// A function that takes a trait object and spawns a task.
//
#[allow(dead_code)] // gives warning when testing all executors at once.
                    //
pub async fn increment_spawn_handle_local_os(a: u8, exec: &dyn LocalSpawnHandle<Rc<u8>>) -> Rc<u8> {
    exec.spawn_handle_local(sum_handle_local(a, 1).boxed())
        .expect("spawn handle")
        .await
}
