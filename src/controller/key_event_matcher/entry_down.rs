use crate::controller::EventQueue;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_entry_down(&mut self) -> Option<()> {
        self.update_pager(1);
        Some(())
    }
}
