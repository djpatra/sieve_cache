use std::sync::{Arc, Condvar, Mutex};

use cache::SizeLimitedCache;
use cache::{ShareableCache, synchronized_cache};

use seive_cache::SeiveCache;

#[test]
fn should_evict_in_fifo_order() {
    let capacity = 10usize;
    let excess = 5;

    let mut cache = SeiveCache::<usize, String>::new(capacity);
    for i in 0..capacity+excess {
        cache.set(i, i.to_string() + "val");
    }

    assert_eq!(cache.cache().len(), capacity);

    for i in 0..excess {
        assert_eq!(cache.get(&i).is_none(), true);
    }

    for i in excess..excess+capacity {
        assert_eq!(cache.get(&i).is_some(), true);    
    }
}


#[test]
fn should_evict_the_unread_keys_in_fifo_order() {
    let capacity = 11usize;
    let excess = 4;

    let mut cache = SeiveCache::<usize, String>::new(capacity);
    for i in 0..capacity+excess {
        cache.set(i, i.to_string() + "val");
        if i >= 2 && (i-2) % 2 == 0 {
            let _ = cache.get(&(i-2));
        }
    }

    assert_eq!(cache.cache().len(), capacity);

    for i in 0..capacity+excess {
        if i % 2 == 0 {
            assert_eq!(cache.get(&i).is_some(), true);
        }
    }

    for i in 0..2*excess {
        if i % 2 == 1 {
            assert_eq!(cache.get(&i).is_none(), true);
        }
    }

    for i in 2*excess..capacity {
        if i % 2 == 1 {
            assert_eq!(cache.get(&i).is_some(), true);
        }
    }    
}

#[test]
fn should_evict_keys_in_fifo_order_over_multi_threads() {
    let capacity = 10;
    let excess = 4_usize;

    let cache = SeiveCache::<i32, String>::new(capacity);
    let threadsafe_cache = Arc::new(synchronized_cache(cache));
    let threadsafe_cache_cloned = Arc::clone(&threadsafe_cache);
    
    let pair = Arc::new((Mutex::new((0, true, true)), Condvar::new()));
    let pair_cloned = pair.clone();

    let join_thread = std::thread::spawn(move || {
        let (lock, cvar) = &*pair_cloned;
        
        loop {
            let mut i = lock.lock().unwrap();
            i.1 = false;

            if i.2 {
                if i.0 >= 2 && i.0 % capacity <= 2*excess && i.0 % 2 == 0 {
                    let _ = threadsafe_cache_cloned.get(&(i.0 as i32 - 2));
                }

                cvar.notify_one();
                while !i.1 {
                    i = cvar.wait(i).unwrap();
                }            
            } else {
                cvar.notify_one();
                break;
            }
       } 
    });

    let (lock, cvar) = &*pair;
    for c in 0..(excess + capacity) + 1 {
        let mut i = lock.lock().unwrap();
        i.1 = true;

        if c < capacity + excess {
            i.0 = c;
            threadsafe_cache.set(c as i32, c.to_string()+"val");               
        } else {
            i.2 = false;
        }

        cvar.notify_one();
        while i.1 {
             i = cvar.wait(i).unwrap();
        }
    }

    let _ = join_thread.join();

    for i in 0..2*excess {
        if i%2 == 0 {
            assert_eq!(threadsafe_cache.get(&(i as i32)), Some(i.to_string()+"val"));
        } else {
            assert_ne!(threadsafe_cache.get(&(i as i32)), Some(i.to_string()+"val"));
        }
    }

    assert_eq!(threadsafe_cache.get(&9), Some("9val".to_string()));
}
