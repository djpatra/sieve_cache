use cache::SizeLimitedCache;
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
