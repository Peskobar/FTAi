use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Rodzaj modyfikacji aplikacji
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatchType {
    Security,
    Performance,
    Feature,
    Compatibility,
    Custom(String),
}

/// Opcje dodatkowe dla zadania patchowania
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOptions {
    pub enable_ai_analysis: bool,
}

impl Default for PatchOptions {
    fn default() -> Self {
        Self { enable_ai_analysis: false }
    }
}

/// Żądanie patchowania
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequest {
    pub apk_path: String,
    pub patch_type: PatchType,
    pub options: PatchOptions,
}

/// Wynik patchowania
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchResult {
    pub output_path: String,
    pub patch_summary: String,
    pub applied_patches: Vec<String>,
    pub warnings: Vec<String>,
    pub ai_analysis: Option<AiAnalysisResult>,
}

/// Poziom ryzyka wykryty przez analizę AI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Wynik analizy AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysisResult {
    pub recommendations: Vec<String>,
    pub risk_assessment: RiskLevel,
    pub confidence_score: f64,
}

/// Status zadania w kolejce
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Zadanie patchowania przechowywane w kolejce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchJob {
    pub id: Uuid,
    pub request: PatchRequest,
    pub status: JobStatus,
    pub result: Option<PatchResult>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Prosta kolejka zadań patchowania
#[derive(Clone)]
pub struct QueueManager {
    queue: Arc<Mutex<VecDeque<PatchJob>>>,
    pub max_concurrent_jobs: usize,
}

impl QueueManager {
    pub fn new(max_concurrent_jobs: usize) -> Self {
        Self { queue: Arc::new(Mutex::new(VecDeque::new())), max_concurrent_jobs }
    }

    /// Dodaje nowe zadanie do kolejki i zwraca jego identyfikator
    pub async fn enqueue(&self, request: PatchRequest) -> Uuid {
        let job = PatchJob {
            id: Uuid::new_v4(),
            request,
            status: JobStatus::Pending,
            result: None,
            created_at: chrono::Utc::now(),
        };
        let mut guard = self.queue.lock().await;
        guard.push_back(job.clone());
        job.id
    }

    /// Pobiera z kolejki najstarsze zadanie
    pub async fn dequeue(&self) -> Option<PatchJob> {
        let mut guard = self.queue.lock().await;
        guard.pop_front()
    }

    /// Obecna wielkość kolejki
    pub async fn len(&self) -> usize {
        let guard = self.queue.lock().await;
        guard.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let queue = QueueManager::new(2);
        let req = PatchRequest {
            apk_path: "app.apk".into(),
            patch_type: PatchType::Security,
            options: PatchOptions::default(),
        };
        let id = queue.enqueue(req.clone()).await;
        assert_eq!(queue.len().await, 1);
        let job = queue.dequeue().await.unwrap();
        assert_eq!(job.id, id);
        assert_eq!(job.request.apk_path, req.apk_path);
    }
}
