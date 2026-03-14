#!/usr/bin/env python3
import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from collections import defaultdict
from pathlib import Path


LOCOMO_REPO = "https://github.com/snap-research/LoCoMo.git"
REPO_ROOT = Path(__file__).resolve().parents[2]


def run(cmd, cwd=None, env=None):
    subprocess.run(cmd, cwd=cwd, env=env, check=True)


def clone_or_update(repo_url: str, name: str) -> Path:
    base = Path(tempfile.gettempdir()) / "cortex-benchmark-sources"
    base.mkdir(parents=True, exist_ok=True)
    target = base / name
    if target.exists():
        run(["git", "-C", str(target), "fetch", "--all", "--prune"])
    else:
        run(["git", "clone", repo_url, str(target)])
    return target


def resolve_cortex_binary(raw_path: str) -> Path:
    if raw_path:
        candidate = Path(raw_path).expanduser()
        if not candidate.is_absolute():
            candidate = (REPO_ROOT / candidate).resolve()
        if candidate.is_file():
            return candidate
        raise FileNotFoundError(
            f"Cortex binary not found at '{candidate}'. "
            f"Build it first with `cargo build --release --bin cortex` "
            f"or pass --cortex-binary/ CORTEX_BINARY with the absolute path."
        )

    env_path = os.environ.get("CORTEX_BINARY", "").strip()
    if env_path:
        return resolve_cortex_binary(env_path)

    names = ["cortex.exe", "cortex"] if os.name == "nt" else ["cortex", "cortex.exe"]
    search_roots = [
        REPO_ROOT / "target" / "release",
        Path.home() / ".cargo" / "target_global" / "release",
    ]

    for root in search_roots:
        for name in names:
            candidate = root / name
            if candidate.is_file():
                return candidate.resolve()

    searched = ", ".join(str(root) for root in search_roots)
    raise FileNotFoundError(
        "Unable to locate Cortex binary automatically. "
        f"Searched: {searched}. "
        "Build it first with `cargo build --release --bin cortex` or set CORTEX_BINARY."
    )


def http_json(url: str, payload=None, method="GET", timeout=30):
    headers = {"Content-Type": "application/json"}
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(url, data=data, headers=headers, method=method)
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return json.loads(response.read().decode("utf-8"))


def wait_for_health(base_url: str, timeout_seconds: int = 60) -> None:
    deadline = time.time() + timeout_seconds
    last_error = None
    while time.time() < deadline:
        try:
            payload = http_json(f"{base_url}/health")
            if payload.get("status") == "ok":
                return
        except Exception as error:  # pragma: no cover - integration only
            last_error = error
        time.sleep(1)
    raise RuntimeError(f"Cortex health check failed: {last_error}")


def normalize_text(text) -> str:
    text = str(text)
    text = text.lower()
    text = re.sub(r"\b(a|an|the)\b", " ", text)
    text = re.sub(r"[^a-z0-9\s]", " ", text)
    return " ".join(text.split())


def token_f1(prediction: str, answer: str) -> float:
    pred_tokens = normalize_text(prediction).split()
    answer_tokens = normalize_text(answer).split()
    if not pred_tokens and not answer_tokens:
        return 1.0
    if not pred_tokens or not answer_tokens:
        return 0.0

    pred_counts = defaultdict(int)
    answer_counts = defaultdict(int)
    for token in pred_tokens:
        pred_counts[token] += 1
    for token in answer_tokens:
        answer_counts[token] += 1

    overlap = sum(min(pred_counts[token], answer_counts[token]) for token in pred_counts)
    if overlap == 0:
        return 0.0
    precision = overlap / len(pred_tokens)
    recall = overlap / len(answer_tokens)
    return 2 * precision * recall / (precision + recall)


def exact_match(prediction: str, answer: str) -> float:
    return float(normalize_text(prediction) == normalize_text(answer))


def session_keys(conversation: dict) -> list[str]:
    return sorted(
        [
            key
            for key in conversation
            if key.startswith("session_") and not key.endswith("_date_time")
        ],
        key=lambda key: int(key.split("_")[1]),
    )


def add_conversation(base_url: str, sample: dict) -> int:
    added = 0
    conversation = sample["conversation"]
    observations = sample.get("observation", {})
    session_summaries = sample.get("session_summary", {})

    for session_key in session_keys(conversation):
        session = conversation.get(session_key, [])
        session_time = conversation.get(f"{session_key}_date_time")
        for turn in session:
            dia_id = turn.get("dia_id", f"{session_key}-{added}")
            speaker = turn.get("speaker", "unknown")
            content = turn.get("text", "")
            path = f"locomo/{sample['sample_id']}/{session_key}/{dia_id}"
            metadata = {
                "benchmark": "locomo",
                "sample_id": sample["sample_id"],
                "session": session_key,
                "session_time": session_time,
                "speaker": speaker,
                "dia_id": dia_id,
                "category": "conversation",
            }
            if turn.get("img_url"):
                metadata["img_url"] = turn["img_url"]
            if turn.get("blip_caption"):
                metadata["blip_caption"] = turn["blip_caption"]
            http_json(
                f"{base_url}/memory/add",
                {
                    "path": path,
                    "content": f"{speaker}: {content}",
                    "metadata": metadata,
                },
                method="POST",
            )
            added += 1

        observation_key = f"{session_key}_observation"
        for index, observation in enumerate(observations.get(observation_key, [])):
            http_json(
                f"{base_url}/memory/add",
                {
                    "path": f"locomo/{sample['sample_id']}/{session_key}/observation/{index}",
                    "content": str(observation),
                    "metadata": {
                        "benchmark": "locomo",
                        "sample_id": sample["sample_id"],
                        "session": session_key,
                        "session_time": session_time,
                        "category": "observation",
                    },
                },
                method="POST",
            )
            added += 1

        summary_key = f"{session_key}_summary"
        summary = session_summaries.get(summary_key)
        if summary:
            http_json(
                f"{base_url}/memory/add",
                {
                    "path": f"locomo/{sample['sample_id']}/{session_key}/summary",
                    "content": str(summary),
                    "metadata": {
                        "benchmark": "locomo",
                        "sample_id": sample["sample_id"],
                        "session": session_key,
                        "session_time": session_time,
                        "category": "session_summary",
                    },
                },
                method="POST",
            )
            added += 1
    return added


def score_predictions(records: list[dict]) -> dict:
    categories = defaultdict(lambda: {"count": 0, "exact_match": 0.0, "token_f1": 0.0})
    summary = {"count": 0, "exact_match": 0.0, "token_f1": 0.0}

    for record in records:
        summary["count"] += 1
        summary["exact_match"] += record["exact_match"]
        summary["token_f1"] += record["token_f1"]

        category = record["category"]
        categories[category]["count"] += 1
        categories[category]["exact_match"] += record["exact_match"]
        categories[category]["token_f1"] += record["token_f1"]

    if summary["count"]:
        summary["exact_match"] /= summary["count"]
        summary["token_f1"] /= summary["count"]

    category_metrics = {}
    for category, metrics in categories.items():
        if metrics["count"]:
            category_metrics[category] = {
                "count": metrics["count"],
                "exact_match": metrics["exact_match"] / metrics["count"],
                "token_f1": metrics["token_f1"] / metrics["count"],
            }

    return {"overall": summary, "by_category": category_metrics}


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cortex-binary", default="")
    parser.add_argument("--base-url", default="http://127.0.0.1:8003")
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--sample-limit", type=int, default=10)
    parser.add_argument("--question-limit", type=int, default=0)
    parser.add_argument("--use-existing-server", action="store_true")
    return parser.parse_args()


def main():
    args = parse_args()
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    locomo_repo = clone_or_update(LOCOMO_REPO, "LoCoMo")
    dataset_path = locomo_repo / "data" / "locomo10.json"
    samples = json.loads(dataset_path.read_text(encoding="utf-8"))[: args.sample_limit]

    child = None
    if not args.use_existing_server:
        cortex_binary = resolve_cortex_binary(args.cortex_binary)
        env = os.environ.copy()
        env["CORTEX_DEV_MODE"] = "1"
        child = subprocess.Popen(
            [str(cortex_binary)],
            env=env,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )

    try:
        wait_for_health(args.base_url)

        records = []
        for sample in samples:
            http_json(f"{args.base_url}/memory/reset", {}, method="POST")
            added = add_conversation(args.base_url, sample)

            questions = sample.get("qa", [])
            if args.question_limit > 0:
                questions = questions[: args.question_limit]

            for index, qa in enumerate(questions):
                question = qa["question"]
                answer = qa["answer"]
                category = qa.get("category", "unknown")
                payload = http_json(
                    f"{args.base_url}/memory/query",
                    {"query": question, "limit": 10},
                    method="POST",
                )
                prediction = payload.get("response", "").strip()
                record = {
                    "sample_id": sample["sample_id"],
                    "question_index": index,
                    "question": question,
                    "answer": answer,
                    "prediction": prediction,
                    "category": category,
                    "evidence": qa.get("evidence", []),
                    "documents_ingested": added,
                    "exact_match": exact_match(prediction, answer),
                    "token_f1": token_f1(prediction, answer),
                }
                records.append(record)

        metrics = score_predictions(records)
        summary = {
            "benchmark": "locomo",
            "dataset": str(dataset_path),
            "samples_evaluated": len(samples),
            "questions_evaluated": len(records),
            "metrics": metrics,
        }

        (output_dir / "summary.json").write_text(
            json.dumps(summary, indent=2), encoding="utf-8"
        )
        (output_dir / "predictions.json").write_text(
            json.dumps(records, indent=2), encoding="utf-8"
        )
        print(json.dumps(summary, indent=2))
    finally:
        if child is not None:
            child.terminate()
            try:
                child.wait(timeout=15)
            except subprocess.TimeoutExpired:
                child.kill()
                child.wait(timeout=15)


if __name__ == "__main__":
    sys.exit(main())
