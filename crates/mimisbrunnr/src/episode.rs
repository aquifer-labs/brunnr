// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

/// Greedy embedding-based episode clustering.
///
/// Records whose embeddings are within `threshold` cosine similarity of an existing episode
/// centroid are assigned to that episode. Otherwise, a new episode is started. Centroids are
/// updated as a running mean over all member embeddings.
#[derive(Debug, Default)]
pub struct EpisodeIndex {
    episodes: Vec<Episode>,
    by_node: HashMap<String, usize>,
}

#[derive(Debug)]
struct Episode {
    node_ids: Vec<String>,
    centroid: Vec<f32>,
}

impl EpisodeIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Assign `node_id` to the nearest episode above `threshold`, or start a new episode.
    pub fn add_record(&mut self, node_id: &str, embedding: &[f32], threshold: f32) {
        let best = self
            .episodes
            .iter()
            .enumerate()
            .map(|(i, ep)| (i, cosine_similarity(embedding, &ep.centroid)))
            .filter(|(_, sim)| *sim >= threshold)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let episode_idx = if let Some((i, _)) = best {
            let ep = &mut self.episodes[i];
            ep.node_ids.push(node_id.to_string());
            let new_count = ep.node_ids.len();
            update_centroid(&mut ep.centroid, embedding, new_count);
            i
        } else {
            let idx = self.episodes.len();
            self.episodes.push(Episode {
                node_ids: vec![node_id.to_string()],
                centroid: embedding.to_vec(),
            });
            idx
        };

        self.by_node.insert(node_id.to_string(), episode_idx);
    }

    /// Returns node_ids of other records in the same episode (excluding `node_id` itself).
    pub fn episode_mates(&self, node_id: &str) -> Vec<String> {
        let Some(&idx) = self.by_node.get(node_id) else {
            return Vec::new();
        };
        self.episodes[idx]
            .node_ids
            .iter()
            .filter(|id| id.as_str() != node_id)
            .cloned()
            .collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    (dot / (norm_a * norm_b)).clamp(-1.0, 1.0)
}

/// Weighted running mean: new_centroid = old * (n-1)/n + new_embedding * 1/n
fn update_centroid(centroid: &mut [f32], embedding: &[f32], new_count: usize) {
    if centroid.len() != embedding.len() || new_count == 0 {
        return;
    }
    let old_weight = (new_count - 1) as f32 / new_count as f32;
    let new_weight = 1.0 / new_count as f32;
    for (c, e) in centroid.iter_mut().zip(embedding.iter()) {
        *c = *c * old_weight + e * new_weight;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn norm(v: &[f32]) -> Vec<f32> {
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        v.iter().map(|x| x / norm).collect()
    }

    #[test]
    fn similar_records_land_in_same_episode() {
        let mut index = EpisodeIndex::new();
        let e1 = norm(&[1.0, 0.0, 0.0]);
        let e2 = norm(&[0.98, 0.14, 0.0]); // ~cos 0.98 to e1
        let e3 = norm(&[0.0, 0.0, 1.0]); // orthogonal

        index.add_record("node:1", &e1, 0.9);
        index.add_record("node:2", &e2, 0.9);
        index.add_record("node:3", &e3, 0.9);

        let mates = index.episode_mates("node:1");
        assert!(
            mates.contains(&"node:2".to_string()),
            "node:2 should be a mate of node:1"
        );
        assert!(
            !mates.contains(&"node:3".to_string()),
            "node:3 should be in a different episode"
        );
    }

    #[test]
    fn dissimilar_records_start_separate_episodes() {
        let mut index = EpisodeIndex::new();
        let e1 = norm(&[1.0, 0.0, 0.0]);
        let e2 = norm(&[0.0, 1.0, 0.0]);
        let e3 = norm(&[0.0, 0.0, 1.0]);

        index.add_record("node:a", &e1, 0.9);
        index.add_record("node:b", &e2, 0.9);
        index.add_record("node:c", &e3, 0.9);

        assert!(index.episode_mates("node:a").is_empty());
        assert!(index.episode_mates("node:b").is_empty());
        assert!(index.episode_mates("node:c").is_empty());
    }

    #[test]
    fn cosine_similarity_is_symmetric() {
        let a = vec![0.6, 0.8];
        let b = vec![0.8, 0.6];
        let sim_ab = cosine_similarity(&a, &b);
        let sim_ba = cosine_similarity(&b, &a);
        assert!((sim_ab - sim_ba).abs() < 1e-6);
    }
}
