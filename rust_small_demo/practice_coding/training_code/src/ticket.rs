use tokio::sync::{Semaphore, SemaphorePermit};

#[allow(dead_code)]
struct Museum {
    tickets_num: Semaphore,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

#[allow(dead_code)]
impl<'a> Ticket<'a> {
    fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

#[allow(dead_code)]
impl Museum {
    fn new(totals: usize) -> Self {
        Self {
            tickets_num: Semaphore::new(totals),
        }
    }

    fn get_tickets(&self) -> Option<Ticket> {
        match self.tickets_num.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    fn tickets(&self) -> usize {
        self.tickets_num.available_permits()
    }
}

impl Drop for Museum {
    fn drop(&mut self) {
        println!("ticket freed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m = Museum::new(50);
        let ticket = m.get_tickets().unwrap();
        assert_eq!(m.tickets(), 49);
        let _tickets: Vec<Ticket> = (0..49).map(|_| m.get_tickets().unwrap()).collect();
        assert_eq!(m.tickets(), 0);
        assert!(m.get_tickets().is_none());
        drop(ticket);
        assert!(m.get_tickets().is_some());
    }
}
