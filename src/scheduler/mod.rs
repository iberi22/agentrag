pub mod job;

pub use job::{RecoveryConfig, ScheduledJob};

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::fs;

const DEFAULT_SCHEDULER_STATE_PATH: &str = "scheduler/jobs.json";

#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub storage_path: PathBuf,
    pub recovery: RecoveryConfig,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from(DEFAULT_SCHEDULER_STATE_PATH),
            recovery: RecoveryConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SchedulerState {
    jobs: Vec<ScheduledJob>,
}

#[allow(dead_code)]
pub struct JobScheduler {
    #[allow(dead_code)]
    jobs: Vec<ScheduledJob>,
    #[allow(dead_code)]
    config: SchedulerConfig,
}

impl JobScheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            jobs: Vec::new(),
            config,
        }
    }

    pub async fn load(config: SchedulerConfig) -> Result<Self> {
        let state = load_state(&config.storage_path).await?;
        Ok(Self {
            jobs: state.jobs,
            config,
        })
    }

    pub async fn load_or_default(config: SchedulerConfig) -> Result<Self> {
        if fs::try_exists(&config.storage_path)
            .await
            .with_context(|| {
                format!(
                    "failed to check scheduler state file {}",
                    config.storage_path.display()
                )
            })?
        {
            Self::load(config).await
        } else {
            Ok(Self::new(config))
        }
    }

    pub fn jobs(&self) -> &[ScheduledJob] {
        &self.jobs
    }

    pub fn jobs_mut(&mut self) -> &mut [ScheduledJob] {
        &mut self.jobs
    }

    pub async fn add_job(&mut self, job: ScheduledJob) -> Result<()> {
        self.jobs.push(job);
        self.persist().await
    }

    pub async fn upsert_job(&mut self, job: ScheduledJob) -> Result<()> {
        match self.jobs.iter_mut().find(|existing| existing.id == job.id) {
            Some(existing) => *existing = job,
            None => self.jobs.push(job),
        }

        self.persist().await
    }

    pub async fn persist(&self) -> Result<()> {
        persist_state(&self.config.storage_path, &self.jobs).await
    }

    pub async fn detect_missed_jobs(&mut self) -> Result<usize> {
        let missed = job::detect_missed_jobs(
            &mut self.jobs,
            Utc::now(),
            self.config.recovery.missed_window,
        );
        self.persist().await?;
        Ok(missed)
    }

    pub async fn recover_missed_jobs<F, Fut>(&mut self, executor: F) -> Result<Vec<String>>
    where
        F: FnMut(ScheduledJob) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let recovered =
            job::recover_missed_jobs(&mut self.jobs, &self.config.recovery, executor).await?;
        self.persist().await?;
        Ok(recovered)
    }
}

async fn load_state(path: &Path) -> Result<SchedulerState> {
    let payload = fs::read_to_string(path)
        .await
        .with_context(|| format!("failed to read scheduler state {}", path.display()))?;

    serde_json::from_str(&payload)
        .with_context(|| format!("failed to deserialize scheduler state {}", path.display()))
}

async fn persist_state(path: &Path, jobs: &[ScheduledJob]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.with_context(|| {
            format!(
                "failed to create scheduler state directory {}",
                parent.display()
            )
        })?;
    }

    let payload = serde_json::to_vec_pretty(&SchedulerState {
        jobs: jobs.to_vec(),
    })
    .context("failed to serialize scheduler state")?;

    fs::write(path, payload)
        .await
        .with_context(|| format!("failed to write scheduler state {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::job::ScheduledJobStatus;
    use chrono::Duration;

    async fn temp_scheduler_path() -> PathBuf {
        let root = std::env::temp_dir()
            .join("cortex-scheduler-tests")
            .join(uuid::Uuid::new_v4().to_string());
        fs::create_dir_all(&root).await.unwrap();
        root.join("scheduler").join("jobs.json")
    }

    #[tokio::test]
    async fn persists_jobs_to_json_file() {
        let storage_path = temp_scheduler_path().await;
        let mut scheduler = JobScheduler::new(SchedulerConfig {
            storage_path: storage_path.clone(),
            recovery: RecoveryConfig::default(),
        });

        scheduler
            .add_job(ScheduledJob::from_schedule("job-a", "index", "0/30 * * * * * *").unwrap())
            .await
            .unwrap();

        assert!(fs::try_exists(&storage_path).await.unwrap());

        let restored = JobScheduler::load(SchedulerConfig {
            storage_path,
            recovery: RecoveryConfig::default(),
        })
        .await
        .unwrap();

        assert_eq!(restored.jobs().len(), 1);
        assert_eq!(restored.jobs()[0].id, "job-a");
    }

    #[tokio::test]
    async fn detects_and_recovers_missed_jobs_through_scheduler() {
        let storage_path = temp_scheduler_path().await;
        let mut scheduler = JobScheduler::new(SchedulerConfig {
            storage_path,
            recovery: RecoveryConfig {
                missed_window: Duration::minutes(5),
                max_per_restart: 2,
                stagger_ms: 0,
            },
        });

        scheduler
            .add_job(ScheduledJob {
                id: "job-a".to_string(),
                name: "job-a".to_string(),
                schedule: "0/15 * * * * * *".to_string(),
                last_run: None,
                next_run: Utc::now() - Duration::minutes(10),
                status: ScheduledJobStatus::Pending,
            })
            .await
            .unwrap();

        let missed = scheduler.detect_missed_jobs().await.unwrap();
        assert_eq!(missed, 1);
        assert_eq!(scheduler.jobs()[0].status, ScheduledJobStatus::Missed);

        let recovered = scheduler
            .recover_missed_jobs(|_| async { Ok(()) })
            .await
            .unwrap();

        assert_eq!(recovered, vec!["job-a".to_string()]);
        assert_eq!(scheduler.jobs()[0].status, ScheduledJobStatus::Completed);
        assert!(scheduler.jobs()[0].last_run.is_some());
    }
}
