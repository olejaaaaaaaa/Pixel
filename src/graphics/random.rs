


static mut unique_ids: Vec<i64> = vec![];
use rand::*;


pub fn id() -> i64 {
    let mut rng = rand::thread_rng();
    let mut id = rng.gen_range(i64::MIN..i64::MAX);

    unsafe {
        loop {

            let mut is_unique_id = true;

            for i in &unique_ids {
                if id == *i {
                    is_unique_id = false;
                }
            }

            if is_unique_id {
                break;
            } else {
                id = rng.gen_range(i64::MIN..i64::MAX);
            }

        }
    }

    id
}