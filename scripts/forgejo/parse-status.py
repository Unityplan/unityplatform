#!/usr/bin/env python3
"""
parse-status.py - Parse phase-1-status.md and extract incomplete tasks

This script parses the markdown status file and extracts all incomplete tasks,
mapping them to appropriate Forgejo labels based on context.

Usage:
    python3 parse-status.py [--stage N] [--format json|csv|table]
"""

import re
import json
import argparse
from pathlib import Path
from typing import List, Dict, Optional
from dataclasses import dataclass, asdict


@dataclass
class Task:
    """Represents a single task from the status file"""
    stage: int
    stage_name: str
    step: str
    task_text: str
    status: str  # '❌' incomplete, '⏸️' paused, '✅' complete
    priority: str
    type_label: str
    area: str
    
    def to_dict(self) -> Dict:
        return asdict(self)
    
    def to_issue_title(self) -> str:
        """Generate a concise issue title"""
        return f"Stage {self.stage}: {self.task_text}"
    
    def to_issue_body(self) -> str:
        """Generate issue body with context"""
        return f"""**Stage:** {self.stage} - {self.stage_name}
**Step:** {self.step}
**Task:** {self.task_text}

**Source:** `docs/status/current/phase-1-status.md`

---

This task was automatically converted from the markdown status tracking.
"""
    
    def get_labels(self) -> List[str]:
        """Get list of label names for this task"""
        labels = [
            f"priority:{self.priority}",
            f"type:{self.type_label}",
            f"area:{self.area}"
        ]
        return labels


class StatusParser:
    """Parser for phase-1-status.md"""
    
    # Regex patterns
    STAGE_PATTERN = re.compile(r'^###\s+Stage\s+(\d+):\s+(.+?)$')
    STEP_PATTERN = re.compile(r'^####\s+Step\s+[\d.]+:\s+(.+?)(?:\s+\([\d/]+\))?\s*(.*)$')
    TASK_PATTERN = re.compile(r'^-\s*([❌⏸️✅])\s+(.+)$')
    
    # Stage to area mapping
    STAGE_AREAS = {
        1: "infrastructure",
        2: "database",
        3: "backend",
        4: "backend",
        5: "frontend",
        6: "backend",
        7: "backend",
        8: "integration",
        9: "integration",
        10: "backend",
        11: "backend",
        12: "frontend",
        13: "testing",
        14: "backend",
    }
    
    # Stage to priority mapping (based on roadmap)
    STAGE_PRIORITIES = {
        5: "high",      # Frontend Auth & Profile - current focus
        7: "high",      # Course Service - core feature
        8: "medium",    # Matrix - important but complex
        9: "medium",    # IPFS - important but complex
        10: "medium",   # Forum - depends on Matrix
        11: "low",      # Translation - can come later
        12: "high",     # Frontend UI - needed for MVP
        13: "critical", # Testing & Deployment - essential for release
    }
    
    def __init__(self, status_file: Path):
        self.status_file = status_file
        self.tasks: List[Task] = []
        
    def parse(self) -> List[Task]:
        """Parse the status file and extract all tasks"""
        with open(self.status_file, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        current_stage = None
        current_stage_name = None
        current_step = None
        
        for line in lines:
            line = line.rstrip()
            
            # Check for stage header
            stage_match = self.STAGE_PATTERN.match(line)
            if stage_match:
                current_stage = int(stage_match.group(1))
                current_stage_name = stage_match.group(2).strip()
                current_step = None
                continue
            
            # Check for step header
            step_match = self.STEP_PATTERN.match(line)
            if step_match:
                current_step = step_match.group(1).strip()
                continue
            
            # Check for task
            task_match = self.TASK_PATTERN.match(line)
            if task_match and current_stage and current_step:
                status = task_match.group(1)
                task_text = task_match.group(2).strip()
                
                # Determine priority
                priority = self.STAGE_PRIORITIES.get(current_stage, "medium")
                
                # Determine area
                area = self.STAGE_AREAS.get(current_stage, "other")
                
                # Determine type (simple heuristic)
                type_label = self._determine_type(task_text)
                
                task = Task(
                    stage=current_stage,
                    stage_name=current_stage_name,
                    step=current_step,
                    task_text=task_text,
                    status=status,
                    priority=priority,
                    type_label=type_label,
                    area=area
                )
                
                self.tasks.append(task)
        
        return self.tasks
    
    def _determine_type(self, task_text: str) -> str:
        """Determine task type from text"""
        text_lower = task_text.lower()
        
        if any(word in text_lower for word in ['test', 'testing', 'verify']):
            return 'testing'
        elif any(word in text_lower for word in ['document', 'documentation', 'readme']):
            return 'docs'
        elif any(word in text_lower for word in ['fix', 'bug', 'error', 'issue']):
            return 'bug'
        elif any(word in text_lower for word in ['refactor', 'cleanup', 'improve']):
            return 'refactor'
        elif any(word in text_lower for word in ['deploy', 'deployment', 'ci/cd']):
            return 'deployment'
        else:
            return 'feature'
    
    def filter_incomplete(self) -> List[Task]:
        """Filter to only incomplete tasks (❌ or ⏸️)"""
        return [t for t in self.tasks if t.status in ['❌', '⏸️']]
    
    def filter_by_stage(self, stage: int) -> List[Task]:
        """Filter tasks by stage number"""
        return [t for t in self.tasks if t.stage == stage]
    
    def get_statistics(self) -> Dict:
        """Get statistics about tasks"""
        total = len(self.tasks)
        complete = len([t for t in self.tasks if t.status == '✅'])
        incomplete = len([t for t in self.tasks if t.status == '❌'])
        paused = len([t for t in self.tasks if t.status == '⏸️'])
        
        by_stage = {}
        for stage in range(1, 15):
            stage_tasks = self.filter_by_stage(stage)
            if stage_tasks:
                by_stage[stage] = {
                    'total': len(stage_tasks),
                    'complete': len([t for t in stage_tasks if t.status == '✅']),
                    'incomplete': len([t for t in stage_tasks if t.status == '❌']),
                    'paused': len([t for t in stage_tasks if t.status == '⏸️']),
                }
        
        return {
            'total_tasks': total,
            'complete': complete,
            'incomplete': incomplete,
            'paused': paused,
            'by_stage': by_stage
        }


def format_table(tasks: List[Task]) -> str:
    """Format tasks as ASCII table"""
    if not tasks:
        return "No tasks found"
    
    output = []
    output.append("\n{:<6} {:<8} {:<12} {:<10} {:<12} {:<50}".format(
        "Stage", "Status", "Priority", "Type", "Area", "Task"
    ))
    output.append("-" * 100)
    
    for task in tasks:
        output.append("{:<6} {:<8} {:<12} {:<10} {:<12} {:<50}".format(
            task.stage,
            task.status,
            task.priority,
            task.type_label,
            task.area,
            task.task_text[:47] + "..." if len(task.task_text) > 50 else task.task_text
        ))
    
    return "\n".join(output)


def format_csv(tasks: List[Task]) -> str:
    """Format tasks as CSV"""
    import csv
    import io
    
    output = io.StringIO()
    writer = csv.writer(output)
    
    # Header
    writer.writerow(['Stage', 'Stage Name', 'Step', 'Status', 'Priority', 'Type', 'Area', 'Task', 'Issue Title'])
    
    # Rows
    for task in tasks:
        writer.writerow([
            task.stage,
            task.stage_name,
            task.step,
            task.status,
            task.priority,
            task.type_label,
            task.area,
            task.task_text,
            task.to_issue_title()
        ])
    
    return output.getvalue()


def main():
    parser = argparse.ArgumentParser(description='Parse phase-1-status.md and extract tasks')
    parser.add_argument('--stage', type=int, help='Filter by stage number')
    parser.add_argument('--format', choices=['json', 'csv', 'table', 'stats'], default='table',
                       help='Output format (default: table)')
    parser.add_argument('--incomplete-only', action='store_true',
                       help='Show only incomplete tasks (❌ and ⏸️)')
    parser.add_argument('--file', type=Path,
                       default=Path(__file__).parent.parent.parent / 'docs/status/current/phase-1-status.md',
                       help='Path to status file')
    
    args = parser.parse_args()
    
    # Check file exists
    if not args.file.exists():
        print(f"❌ Error: Status file not found: {args.file}")
        return 1
    
    # Parse file
    status_parser = StatusParser(args.file)
    status_parser.parse()
    
    # Apply filters
    tasks = status_parser.tasks
    
    if args.incomplete_only:
        tasks = status_parser.filter_incomplete()
    
    if args.stage:
        tasks = [t for t in tasks if t.stage == args.stage]
    
    # Output
    if args.format == 'json':
        output = {
            'tasks': [t.to_dict() for t in tasks],
            'count': len(tasks)
        }
        print(json.dumps(output, indent=2))
    
    elif args.format == 'csv':
        print(format_csv(tasks))
    
    elif args.format == 'table':
        print(format_table(tasks))
        print(f"\nTotal: {len(tasks)} tasks")
    
    elif args.format == 'stats':
        stats = status_parser.get_statistics()
        print(json.dumps(stats, indent=2))
    
    return 0


if __name__ == '__main__':
    exit(main())
