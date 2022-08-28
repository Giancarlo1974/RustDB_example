use rustdb::{
    AccessPagedData, AtomicFile, BuiltinMap,
    Database, GenTransaction, 
    SharedPagedData, SimpleFileStorage,
};
use std::{sync::Arc};

fn main() {
    println!("Hello, world!");

    // Construct an AtomicFile. This ensures that updates to the database are "all or nothing".
    let file = Box::new(SimpleFileStorage::new("rustweb.rustdb"));
    let upd = Box::new(SimpleFileStorage::new("rustweb.upd"));
    let stg = Box::new(AtomicFile::new(file, upd));

    // SharedPagedData allows for one writer and multiple readers.
    // Note that readers never have to wait, they get a "virtual" read-only copy of the database.
    let spd = Arc::new(SharedPagedData::new(stg));
    {
        let mut s = spd.stash.lock().unwrap();
        s.mem_limit = 10 * 1000000;
        s.trace = false;
    }
    // Construct map of "builtin" functions that can be called in SQL code.
    // Include extra functions ARGON, EMAILTX and SLEEP as well as the standard functions.
    let bmap = BuiltinMap::default();
    let bmap = Arc::new(bmap);

    // Get write-access to database ( there will only be one of these ).
    let wapd = AccessPagedData::new_writer(spd);

    println!("Init db");
    let db = Database::new(wapd, init::INITSQL, bmap);
    // let db = Database::new(wapd, "", bmap);
    println!("Hello, db!");
    let a: String = "SELECT PersonID, LastName, FirstName, Address, Country FROM dbo.Persons".to_owned();
    let sql: Arc<String> = Arc::from(a);
    let mut x = Box::new(GenTransaction::new());
    db.run(&sql, &mut *x);
    println!("End!");


}

/// Database initialisation string.
mod init;

