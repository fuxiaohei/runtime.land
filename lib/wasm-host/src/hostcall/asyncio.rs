use super::{
    host::land::asyncio::{asyncio, types::Handle},
    HostContext,
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, Arc},
};
use tokio::sync::{Mutex, Notify};
use tracing::debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Pending, // ready to run
    // Running,
    // Canceled,
    Finished, // run done
}

#[derive(Clone, Debug)]
struct Task {
    timing: Option<Status>,
    status: Status,
}

impl Task {
    pub fn is_runnable(&self) -> bool {
        if let Some(t) = &self.timing {
            if *t == Status::Pending {
                return false;
            }
        }
        self.status == Status::Pending
    }
}

#[derive(Debug)]
struct Inner {
    pub seq_id: AtomicU32,
    pub tasks: HashMap<u32, Task>,
    pub notify: Arc<Notify>,
}

impl Inner {
    pub fn new(notify: Arc<Notify>) -> Self {
        Self {
            seq_id: AtomicU32::new(1),
            tasks: HashMap::new(),
            notify,
        }
    }

    fn new_task(&mut self) -> Result<Handle, ()> {
        let seq_id = self
            .seq_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let task = Task {
            timing: None,
            status: Status::Pending,
        };
        self.tasks.insert(seq_id, task);
        // println!("asyncio->new_task: {}", seq_id);
        debug!("asyncio->new_task: {}", seq_id);
        Ok(seq_id)
    }

    async fn new_sleep(&mut self) -> Result<Handle, ()> {
        let seq_id = self
            .seq_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let task = Task {
            timing: Some(Status::Pending),
            status: Status::Pending,
        };
        self.tasks.insert(seq_id, task);
        Ok(seq_id)
    }

    async fn timeup(&mut self, handle: Handle) {
        let task = self.tasks.get_mut(&handle);
        if let Some(task) = task {
            // println!("asyncio->timeup: {}", handle);
            debug!("asyncio->timeup: {}", handle);
            task.timing = Some(Status::Finished);
            self.notify.notify_one();
        }
    }

    /// select_one select one task to run
    async fn select_one(&mut self) -> (Option<Handle>, bool) {
        // all tasks are exeucted
        if self.tasks.is_empty() {
            // println!("asyncio->select_one: all tasks are exeucted");
            debug!("asyncio->select_one: all tasks are exeucted");
            return (None, false);
        }
        let mut runnable_seq_id = 0;
        for (seq_id, task) in self.tasks.iter() {
            if task.is_runnable() {
                runnable_seq_id = *seq_id;
                // println!("asyncio->select_one: runnable_seq_id: {}", runnable_seq_id);
                debug!("asyncio->select_one: runnable_seq_id: {}", runnable_seq_id);
                break;
            }
        }
        // no runnable task, but some tasks are exists, need wait
        if runnable_seq_id == 0 && !self.tasks.is_empty() {
            // println!("asyncio->select_one: wait");
            debug!("asyncio->select_one: wait");
            return (None, true);
        }
        self.tasks.remove(&runnable_seq_id);
        (Some(runnable_seq_id), true)
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    notify: Arc<Notify>,
    inner: Arc<Mutex<Inner>>,
}

impl Context {
    pub fn new() -> Self {
        let notify = Arc::new(Notify::new());
        Self {
            inner: Arc::new(Mutex::new(Inner::new(notify.clone()))),
            notify,
        }
    }
}

#[async_trait::async_trait]
impl asyncio::Host for Context {
    async fn new(&mut self) -> Result<Handle, ()> {
        self.inner.lock().await.new_task()
    }
    async fn sleep(&mut self, ms: u32) -> Result<Handle, ()> {
        let self2 = self.clone();
        let seq_id = self
            .inner
            .lock()
            .await
            .new_sleep()
            .await
            .expect("new_sleep error");
        // println!("asyncio->new_sleep: {}, {}ms", seq_id, ms);
        debug!("asyncio->new_sleep: {}, {}ms", seq_id, ms);
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(ms as u64)).await;
            self2.inner.lock().await.timeup(seq_id).await;
        });
        Ok(seq_id)
    }
    async fn select(&mut self) -> (Option<Handle>, bool) {
        self.inner.lock().await.select_one().await
    }
    async fn ready(&mut self) {
        self.notify.notified().await
    }
}

#[async_trait::async_trait]
impl asyncio::Host for HostContext {
    async fn new(&mut self) -> Result<Handle, ()> {
        self.asyncio_ctx.new().await
    }
    async fn sleep(&mut self, ms: u32) -> Result<Handle, ()> {
        self.asyncio_ctx.sleep(ms).await
    }
    async fn select(&mut self) -> (Option<Handle>, bool) {
        self.asyncio_ctx.select().await
    }
    async fn ready(&mut self) {
        self.asyncio_ctx.ready().await
    }
}

#[cfg(test)]
mod asyncio_test {
    use crate::hostcall::{asyncio::Context, host::land::asyncio::asyncio::Host};

    #[tokio::test]
    async fn test_sleep() {
        let mut ctx = Context::new();
        let _ = ctx.sleep(1500).await.unwrap();
        let _ = ctx.sleep(1000).await.unwrap();
        loop {
            // println!("select");
            let (handle, is_wait) = ctx.select().await;
            // println!("handle: {:?}, is_wait: {:?}", handle, is_wait);
            if !is_wait {
                break;
            }
            if handle.is_none() {
                ctx.ready().await;
            }
        }
    }
}
