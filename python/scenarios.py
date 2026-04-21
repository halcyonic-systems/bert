"""
Scenario presets for Bitcoin ABM via BERT.

Ported from bitcoin-abm-v2/backend/scenarios.py.
Each scenario overrides base params and states a testable hypothesis.
"""

SCENARIOS = {
    "baseline": {
        "name": "Baseline",
        "description": "Normal Bitcoin network operation with typical transaction rates and mining distribution.",
        "hypothesis": "Stable dynamics: mempool clears regularly, fees stay moderate, difficulty adjusts smoothly.",
        "params": {
            "tx_rate": 5,
            "base_hashrate": 100.0,
            "miner_count": 10,
            "block_reward": 6.25,
            "difficulty_adjustment_rate": 0.05,
            "mempool_limit": 100,
            "fee_sensitivity": 1.0,
        },
    },
    "fee_spike": {
        "name": "Fee Spike",
        "description": "High demand period with congested mempool, simulating inscription waves or major market events.",
        "hypothesis": "Mempool grows rapidly, fees spike as users compete for block space, miners profit.",
        "params": {
            "tx_rate": 15,
            "base_hashrate": 100.0,
            "miner_count": 10,
            "block_reward": 6.25,
            "difficulty_adjustment_rate": 0.05,
            "mempool_limit": 50,
            "fee_sensitivity": 2.0,
        },
    },
    "halving": {
        "name": "Halving Event",
        "description": "Block reward drops 50%, simulating a Bitcoin halving. Miners must rely more on fees.",
        "hypothesis": "Initial hashrate drop as marginal miners exit, fees become more important, difficulty adjusts down.",
        "params": {
            "tx_rate": 5,
            "base_hashrate": 100.0,
            "miner_count": 10,
            "block_reward": 3.125,
            "difficulty_adjustment_rate": 0.1,
            "mempool_limit": 100,
            "fee_sensitivity": 1.5,
        },
    },
    "hash_war": {
        "name": "Hash War",
        "description": "Rapid hashrate growth as new mining hardware comes online or competition intensifies.",
        "hypothesis": "Blocks found faster initially, difficulty increases, equilibrium restored at higher hashrate.",
        "params": {
            "tx_rate": 5,
            "base_hashrate": 150.0,
            "miner_count": 20,
            "block_reward": 6.25,
            "difficulty_adjustment_rate": 0.08,
            "mempool_limit": 100,
            "fee_sensitivity": 1.0,
            "hashrate_growth": 0.05,
        },
    },
    "contentious_fork": {
        "name": "Contentious Fork",
        "description": "Developer community split on protocol changes, simulating governance crisis.",
        "hypothesis": "BIP proposal rate increases, fewer reach consensus, development slows, uncertainty rises.",
        "params": {
            "tx_rate": 5,
            "base_hashrate": 100.0,
            "miner_count": 10,
            "block_reward": 6.25,
            "difficulty_adjustment_rate": 0.05,
            "mempool_limit": 100,
            "fee_sensitivity": 1.0,
            "bip_rate": 3,
            "consensus_threshold": 0.9,
        },
    },
    "attack_51": {
        "name": "51% Attack",
        "description": "Single entity controls majority hashrate, simulating centralization threat.",
        "hypothesis": "Dominant miner finds most blocks, network security compromised, other miners exit.",
        "params": {
            "tx_rate": 5,
            "base_hashrate": 100.0,
            "miner_count": 5,
            "block_reward": 6.25,
            "difficulty_adjustment_rate": 0.05,
            "mempool_limit": 100,
            "fee_sensitivity": 1.0,
            "dominant_miner_share": 0.51,
        },
    },
}


def get_scenario(scenario_id: str) -> dict:
    return SCENARIOS.get(scenario_id, SCENARIOS["baseline"])


def list_scenarios() -> list:
    return [
        {"id": k, "name": v["name"], "description": v["description"], "hypothesis": v["hypothesis"]}
        for k, v in SCENARIOS.items()
    ]
