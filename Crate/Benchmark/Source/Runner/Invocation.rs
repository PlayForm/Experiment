//! A `docker run` benchmark invocation held as argv data.
//!
//! Grounded in `prun_lab.py::docker_limits` (501-518) and `run_group`
//! (568-586): network none, fixed memory and swap, no-new-privileges,
//! optional cpuset, injected environment, three mounts (fixtures read-only,
//! variants read-only, results writable) and a `/checks` tmpfs.

use crate::config::Invariant;
use crate::fixture::Volume;

use super::Group;

/// The container configuration for one runner group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invocation {
    /// Owning runner group.
    pub group: Group,
    /// Runner image ID.
    pub image: String,
    /// Docker resource and affinity flags.
    pub limits: Vec<String>,
    /// Docker mount arguments.
    pub mounts: Vec<String>,
    /// Checks tmpfs specification.
    pub tmpfs: String,
    /// JSON driver payload, held as data.
    pub payload: String,
}

impl Invocation {
    /// Compose container arguments (prun_lab.py:568-586).
    #[must_use]
    pub fn compose(
        invariant: &Invariant,
        group: Group,
        image: &str,
        volume: &Volume,
        artifacts: &str,
        results: &str,
        payload: impl Into<String>,
    ) -> Self {
        let mut limits = vec![
            "--network".to_owned(),
            "none".to_owned(),
            "--memory".to_owned(),
            invariant.memory.clone(),
            "--memory-swap".to_owned(),
            invariant.memory.clone(),
            "--security-opt".to_owned(),
            "no-new-privileges".to_owned(),
        ];
        invariant.affinity(group.tag()).into_iter().for_each(|value| {
            limits.push("--cpuset-cpus".to_owned());
            limits.push(value.to_owned());
        });
        invariant.environment.iter().for_each(|(key, value)| {
            limits.push("-e".to_owned());
            limits.push(format!("{key}={value}"));
        });
        let mounts = vec![
            format!("type=volume,src={},dst=/data,readonly", volume.name()),
            format!("type=bind,src={artifacts},dst=/variants,readonly"),
            format!("type=bind,src={results},dst=/results"),
        ];
        Self {
            group,
            image: image.to_owned(),
            limits,
            mounts,
            tmpfs: "/checks:rw,nosuid,nodev".to_owned(),
            payload: payload.into(),
        }
    }

    /// Complete docker argv (prun_lab.py:568-586).
    #[must_use]
    pub fn argv(&self) -> Vec<String> {
        let mut argv = vec![
            "docker".to_owned(),
            "run".to_owned(),
            "--rm".to_owned(),
        ];
        argv.extend(self.limits.iter().cloned());
        self.mounts.iter().for_each(|mount| {
            argv.push("--mount".to_owned());
            argv.push(mount.clone());
        });
        argv.extend([
            "--tmpfs".to_owned(),
            self.tmpfs.clone(),
            self.image.clone(),
            "python3".to_owned(),
            "-c".to_owned(),
            self.payload.clone(),
        ]);
        argv
    }
}
