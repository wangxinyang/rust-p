use tokio::sync::{Semaphore, SemaphorePermit};

struct Museum {
    ticket: Semaphore,
}

#[allow(dead_code)]
struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("Ticket freed");
    }
}

impl Museum {
    fn new(count: usize) -> Self {
        Self {
            ticket: Semaphore::new(count),
        }
    }

    fn get_ticket(&self) -> Option<Ticket<'_>> {
        match self.ticket.try_acquire() {
            Ok(permit) => Some(Ticket { permit }),
            Err(_) => None,
        }
    }

    fn ticket(&self) -> usize {
        self.ticket.available_permits()
    }
}

fn main() {
    let museum = Museum::new(50);
    let _ticket = museum.get_ticket().unwrap();
    assert_eq!(49, museum.ticket());

    let _tickets: Vec<Ticket> = (0..49).map(|_| museum.get_ticket().unwrap()).collect();
    assert_eq!(0, museum.ticket());
    {
        assert!(museum.get_ticket().is_none());
        drop(_ticket);
        assert_eq!(1, museum.ticket());
    }
    println!("-------");
}
