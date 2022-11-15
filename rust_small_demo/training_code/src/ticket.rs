use tokio::sync::{Semaphore, SemaphorePermit};

struct Museum {
    _tickets_num: Semaphore,
}

#[derive(Debug)]
struct Ticket<'a> {
    _permit: SemaphorePermit<'a>,
}

impl<'a> Ticket<'a> {
    fn _new(permit: SemaphorePermit<'a>) -> Self {
        Self { _permit: permit }
    }
}

impl Museum {
    fn _new(totals: usize) -> Self {
        Self {
            _tickets_num: Semaphore::new(totals),
        }
    }

    fn _get_tickets(&self) -> Option<Ticket> {
        match self._tickets_num.try_acquire() {
            Ok(permit) => Some(Ticket::_new(permit)),
            Err(_) => None,
        }
    }

    fn _tickets(&self) -> usize {
        self._tickets_num.available_permits()
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
        let tickets: Vec<Ticket> = (0..49).map(|_| m.get_tickets().unwrap()).collect();
        assert_eq!(m.tickets(), 0);
        assert!(m.get_tickets().is_none());
        drop(ticket);
        assert!(m.get_tickets().is_some());
    }
}
