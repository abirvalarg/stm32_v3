use core::{task::{Context, Poll}, pin::Pin};

use alloc::{collections::LinkedList, sync::Arc, boxed::Box};
use futures::{
    Future,
    task::{ArcWake, waker_ref}
};

use crate::sync::SyncCell;

static QUEUE: SyncCell<LinkedList<Arc<Task>>> = SyncCell::new(LinkedList::new());
static COUNT: SyncCell<usize> = SyncCell::new(0);

pub struct Task {
    future: SyncCell<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let mut queue = QUEUE.get();
        queue.push_back(arc_self.clone());
    }
}

pub fn add_task<F: Future<Output = ()> + 'static + Send>(future: F) {
    let task = Arc::new(Task {
        future: SyncCell::new(Some(Box::pin(future)))
    });
    QUEUE.get().push_back(task);
    *COUNT.get() += 1;
}

#[no_mangle]
fn executor_loop() -> ! {
    loop {
        let mut queue = QUEUE.get();
        let task = queue.pop_front();
        drop(queue);
        if let Some(task) = task {
            let mut future_slot = task.future.get();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&waker);
                match future.as_mut().poll(&mut context) {
                    Poll::Ready(()) => {
                        let mut cnt = COUNT.get();
                        *cnt -= 1;
                    }
                    Poll::Pending => *future_slot = Some(future)
                }
            }
        } else {
            unsafe {
                core::arch::asm!("wfi");
            }
        }
    }
}
