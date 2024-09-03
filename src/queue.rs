#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Queue(std::collections::VecDeque<crate::event::Event>);

impl Queue {
    pub fn new() -> Self {
        Queue(std::collections::VecDeque::new())
    }

    pub fn enqueue(&mut self, event: crate::event::Event) {
        self.0.push_front(event);
    }

    pub fn dequeue(&mut self) -> Option<crate::event::Event> {
        self.0.pop_back()
    }
}
