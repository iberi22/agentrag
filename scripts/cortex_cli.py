#!/usr/bin/env python3
"""
Cortex CLI - Client for Cortex Memory Server

Usage:
    python cortex_cli.py create --path "project/doc" --content "Content here"
    python cortex_cli.py search --query "authentication"
    python cortex_cli.py list
    python cortex_cli.py get --id "memory_id"
    python cortex_cli.py sync-gitcore --project "E:/scripts-python/manteniapp"
"""

import os
import sys
import json
import argparse
import requests
from pathlib import Path

# Configuration
CORTEX_URL = os.environ.get("CORTEX_URL", "http://localhost:8003")
CORTEX_TOKEN = os.environ.get("CORTEX_TOKEN", "dev-token")


class CortexClient:
    """Client for Cortex Memory Server"""
    
    def __init__(self, url=CORTEX_URL, token=CORTEX_TOKEN):
        self.url = url
        self.token = token
        self.headers = {"X-Cortex-Token": token}
    
    def create_memory(self, path: str, content: str, metadata: dict = None):
        """Create a new memory"""
        data = {
            "path": path,
            "content": content,
            "metadata": metadata or {}
        }
        response = requests.post(
            f"{self.url}/memory/",
            json=data,
            headers=self.headers
        )
        return response.json()
    
    def search_memory(self, query: str, limit: int = 10):
        """Search memories"""
        data = {"query": query, "limit": limit}
        response = requests.post(
            f"{self.url}/memory/search",
            json=data,
            headers=self.headers
        )
        return response.json()
    
    def get_memory(self, mem_id: str):
        """Get memory by ID"""
        response = requests.get(
            f"{self.url}/memory/{mem_id}",
            headers=self.headers
        )
        return response.json()
    
    def list_memories(self, limit: int = 50):
        """List all memories"""
        response = requests.get(
            f"{self.url}/memory/",
            headers=self.headers,
            params={"limit": limit}
        )
        return response.json()
    
    def sync_gitcore_project(self, project_path: str):
        """Sync a GitCore project to Cortex"""
        project_path = Path(project_path)
        
        if not project_path.exists():
            return {"error": f"Project not found: {project_path}"}
        
        docs_path = project_path / "DOCS" / "SRC"
        if not docs_path.exists():
            docs_path = project_path / "docs" / "src"
        
        if not docs_path.exists():
            return {"error": f"DOCS/SRC not found in {project_path}"}
        
        synced = 0
        errors = []
        
        for md_file in docs_path.rglob("*.md"):
            if md_file.name.startswith('.'):
                continue
            
            try:
                content = md_file.read_text(encoding='utf-8')
                path = f"{project_path.name}/{md_file.relative_to(docs_path)}"
                
                metadata = {
                    "project": project_path.name,
                    "type": "src",
                    "file": str(md_file.relative_to(project_path))
                }
                
                result = self.create_memory(path, content, metadata)
                synced += 1
                print(f"  [OK] {path}")
                
            except Exception as e:
                errors.append(f"{md_file}: {e}")
                print(f"  [ERROR] {md_file}: {e}")
        
        return {
            "synced": synced,
            "errors": errors,
            "project": project_path.name
        }


def main():
    parser = argparse.ArgumentParser(description="Cortex CLI")
    subparsers = parser.add_subparsers(dest="command", help="Commands")
    
    # Create memory
    create_parser = subparsers.add_parser("create", help="Create memory")
    create_parser.add_argument("--path", required=True, help="Memory path")
    create_parser.add_argument("--content", required=True, help="Memory content")
    create_parser.add_argument("--metadata", help="Metadata JSON")
    
    # Search
    search_parser = subparsers.add_parser("search", help="Search memories")
    search_parser.add_argument("--query", required=True, help="Search query")
    search_parser.add_argument("--limit", type=int, default=10, help="Limit results")
    
    # Get
    get_parser = subparsers.add_parser("get", help="Get memory by ID")
    get_parser.add_argument("--id", required=True, help="Memory ID")
    
    # List
    list_parser = subparsers.add_parser("list", help="List memories")
    list_parser.add_argument("--limit", type=int, default=50, help="Limit results")
    
    # Sync GitCore
    sync_parser = subparsers.add_parser("sync-gitcore", help="Sync GitCore project")
    sync_parser.add_argument("--project", required=True, help="Project path")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    client = CortexClient()
    
    if args.command == "create":
        metadata = json.loads(args.metadata) if args.metadata else None
        result = client.create_memory(args.path, args.content, metadata)
        print(json.dumps(result, indent=2))
        
    elif args.command == "search":
        result = client.search_memory(args.query, args.limit)
        print(json.dumps(result, indent=2))
        
    elif args.command == "get":
        result = client.get_memory(args.id)
        print(json.dumps(result, indent=2))
        
    elif args.command == "list":
        result = client.list_memories(args.limit)
        print(json.dumps(result, indent=2))
        
    elif args.command == "sync-gitcore":
        print(f"Syncing GitCore project: {args.project}")
        result = client.sync_gitcore_project(args.project)
        print(f"\nResult: {json.dumps(result, indent=2)}")


if __name__ == "__main__":
    main()
