//! Reference ONLY: deterministic *planning* allocation. This does NOT transfer
//! funds, sign envelopes, persist transactions, implement credential checks or
//! settle actual spend. `cargo run -- scenario.json` outputs an allocation plan
//! as JSON.
use std::{
	collections::{BTreeMap, BTreeSet},
	env,
	error::Error,
	fs,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Resources {
	input_tokens:u64,
	output_tokens:u64,
	cpu_core_seconds:u64,
	benchmark_seconds:u64,
	gpu_seconds:u64,
	storage_byte_days:u64,
	reference_requests:u64,
}
impl Resources {
	fn fits(&self, need:&Self) -> bool {
		self.input_tokens >= need.input_tokens
			&& self.output_tokens >= need.output_tokens
			&& self.cpu_core_seconds >= need.cpu_core_seconds
			&& self.benchmark_seconds >= need.benchmark_seconds
			&& self.gpu_seconds >= need.gpu_seconds
			&& self.storage_byte_days >= need.storage_byte_days
			&& self.reference_requests >= need.reference_requests
	}

	fn subtract(&mut self, n:&Self) {
		assert!(self.fits(n));
		self.input_tokens -= n.input_tokens;
		self.output_tokens -= n.output_tokens;
		self.cpu_core_seconds -= n.cpu_core_seconds;
		self.benchmark_seconds -= n.benchmark_seconds;
		self.gpu_seconds -= n.gpu_seconds;
		self.storage_byte_days -= n.storage_byte_days;
		self.reference_requests -= n.reference_requests;
	}

	fn norm_cost(&self, cap:&Self) -> f64 {
		fn portion(x:u64, n:u64) -> f64 {
			if n == 0 { if x == 0 { 0.0 } else { 1.0e9 } } else { (x as f64) / (n as f64) }
		}
		portion(self.input_tokens, cap.input_tokens)
			+ portion(self.output_tokens, cap.output_tokens)
			+ portion(self.cpu_core_seconds, cap.cpu_core_seconds)
			+ portion(self.benchmark_seconds, cap.benchmark_seconds)
			+ portion(self.gpu_seconds, cap.gpu_seconds)
			+ portion(self.storage_byte_days, cap.storage_byte_days)
			+ portion(self.reference_requests, cap.reference_requests)
	}
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Bid {
	bid_id:String,
	case_id:String,
	stance:String,
	model_cohort:String,
	mechanism_key:String,
	/// Policy-exogenous score from an independent estimator, not the bidder's
	/// self-report.
	externally_estimated_information_gain:f64,
	requested:Resources,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Scenario {
	capacity:Resources,
	bids:Vec<Bid>,
	mandatory_stances:Vec<String>,
	max_per_model_cohort:usize,
	max_per_mechanism:usize,
}
#[derive(Serialize)]
struct Decision {
	bid_id:String,
	accepted:bool,
	reason:String,
}
#[derive(Serialize)]
struct Plan {
	decisions:Vec<Decision>,
	remaining:Resources,
	used:Resources,
}

fn allocate(input:Scenario) -> Result<Plan, String> {
	let cap = input.capacity;
	if input.max_per_model_cohort == 0 || input.max_per_mechanism == 0 {
		return Err("caps must be positive".into());
	}
	let mut seen = BTreeSet::new();
	for b in &input.bids {
		if b.bid_id.is_empty() || b.case_id.is_empty() || b.model_cohort.is_empty() || b.mechanism_key.is_empty() {
			return Err("empty bid identifiers".into());
		}
		if !seen.insert(b.bid_id.clone()) {
			return Err(format!("repeated bid ID: {}", b.bid_id));
		}
		if !b.externally_estimated_information_gain.is_finite() || b.externally_estimated_information_gain < 0.0 {
			return Err("invalid information gain".into());
		}
	}
	let mut ordered = input.bids;
	let mandatory:BTreeSet<String> = input.mandatory_stances.into_iter().collect();
	// Priority tiers ensure dissent/replication bids are considered before
	// ordinary bids. This does not guarantee all stances are funded if capacity
	// is insufficient.
	ordered.sort_by(|a, b| {
		let a_tier = if mandatory.contains(&a.stance) { 0 } else { 1 };
		let b_tier = if mandatory.contains(&b.stance) { 0 } else { 1 };
		let a_score = a.externally_estimated_information_gain / (1.0 + a.requested.norm_cost(&cap));
		let b_score = b.externally_estimated_information_gain / (1.0 + b.requested.norm_cost(&cap));
		a_tier
			.cmp(&b_tier)
			.then_with(|| b_score.total_cmp(&a_score))
			.then_with(|| a.bid_id.cmp(&b.bid_id))
	});
	let mut remaining = cap;
	let mut models:BTreeMap<String, usize> = BTreeMap::new();
	let mut mechanisms:BTreeMap<String, usize> = BTreeMap::new();
	let mut decisions = Vec::new();
	for b in ordered {
		let reason = if *models.get(&b.model_cohort).unwrap_or(&0) >= input.max_per_model_cohort {
			"model_cohort_cap"
		} else if *mechanisms.get(&b.mechanism_key).unwrap_or(&0) >= input.max_per_mechanism {
			"mechanism_cap"
		} else if !remaining.fits(&b.requested) {
			"insufficient_resources"
		} else {
			"reserved_in_plan"
		};
		let accepted = reason == "reserved_in_plan";
		if accepted {
			remaining.subtract(&b.requested);
			*models.entry(b.model_cohort).or_insert(0) += 1;
			*mechanisms.entry(b.mechanism_key).or_insert(0) += 1;
		}
		decisions.push(Decision { bid_id:b.bid_id, accepted, reason:reason.into() });
	}
	let used = Resources {
		input_tokens:cap.input_tokens - remaining.input_tokens,
		output_tokens:cap.output_tokens - remaining.output_tokens,
		cpu_core_seconds:cap.cpu_core_seconds - remaining.cpu_core_seconds,
		benchmark_seconds:cap.benchmark_seconds - remaining.benchmark_seconds,
		gpu_seconds:cap.gpu_seconds - remaining.gpu_seconds,
		storage_byte_days:cap.storage_byte_days - remaining.storage_byte_days,
		reference_requests:cap.reference_requests - remaining.reference_requests,
	};
	Ok(Plan { decisions, remaining, used })
}
fn main() -> Result<(), Box<dyn Error>> {
	let file = env::args()
		.nth(1)
		.ok_or("usage: prun-research-market-prototype SCENARIO.json")?;
	let text = fs::read_to_string(file)?;
	let scenario:Scenario = serde_json::from_str(&text)?;
	let result = allocate(scenario).map_err(|e| format!("allocation rejected: {e}"))?;
	println!("{}", serde_json::to_string_pretty(&result)?);
	Ok(())
}
#[cfg(test)]
mod tests {
	use super::*;
	fn cap() -> Resources {
		Resources {
			input_tokens:100,
			output_tokens:100,
			cpu_core_seconds:100,
			benchmark_seconds:0,
			gpu_seconds:0,
			storage_byte_days:100,
			reference_requests:10,
		}
	}
	fn bid(name:&str, stance:&str, gain:f64) -> Bid {
		Bid {
			bid_id:name.into(),
			case_id:"case".into(),
			stance:stance.into(),
			model_cohort:name.into(),
			mechanism_key:name.into(),
			externally_estimated_information_gain:gain,
			requested:Resources { input_tokens:60, ..Resources::default() },
		}
	}
	#[test]
	fn protects_skeptic_capacity() {
		let result = allocate(Scenario {
			capacity:cap(),
			bids:vec![bid("fast-proponent", "proponent", 100.0), bid("skeptic", "skeptic", 1.0)],
			mandatory_stances:vec!["skeptic".into()],
			max_per_model_cohort:2,
			max_per_mechanism:2,
		})
		.unwrap();
		assert_eq!(result.decisions.iter().find(|d| d.bid_id == "skeptic").unwrap().accepted, true);
		assert_eq!(
			result.decisions.iter().find(|d| d.bid_id == "fast-proponent").unwrap().accepted,
			false
		);
		assert_eq!(result.used.input_tokens, 60);
	}
	#[test]
	fn does_not_overdraw() {
		let r = allocate(Scenario {
			capacity:cap(),
			bids:vec![bid("a", "proponent", 3.0), bid("b", "proponent", 2.0)],
			mandatory_stances:vec![],
			max_per_model_cohort:2,
			max_per_mechanism:2,
		})
		.unwrap();
		assert_eq!(r.used.input_tokens, 60);
	}
	#[test]
	fn rejects_nonfinite_gain() {
		let err = allocate(Scenario {
			capacity:cap(),
			bids:vec![bid("a", "proponent", f64::NAN)],
			mandatory_stances:vec![],
			max_per_model_cohort:2,
			max_per_mechanism:2,
		});
		assert!(err.is_err());
	}
}
