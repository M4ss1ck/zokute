#!/usr/bin/env python3
"""Maibuk Notes plugin — reads note titles/preview from a Maibuk SQLite database."""

import json
import sqlite3
import sys
from pathlib import Path


def read_stdin() -> dict:
    raw = sys.stdin.read()
    return json.loads(raw)


def check_schema(cursor) -> list[str]:
    cursor.execute("PRAGMA table_info(notes)")
    columns = {row[1] for row in cursor.fetchall()}
    required = {"id", "book_id", "title", "content", "pinned", "order",
                "created_at", "updated_at", "content_updated_at"}
    missing = required - columns
    if missing:
        raise ValueError(
            f"notes table missing required columns: {', '.join(sorted(missing))}"
        )
    return list(columns)


def plain_text_from_tiptap(content: str, max_len: int = 200) -> str:
    import re
    text = re.sub(r'<[^>]+>', '', content)
    text = text.replace('&nbsp;', ' ').replace('&amp;', '&')
    text = re.sub(r'\s+', ' ', text).strip()
    if len(text) > max_len:
        text = text[:max_len].rstrip() + '…'
    return text


def query_notes(cursor, selection: str, limit: int) -> list[dict]:
    clamped_limit = min(max(limit, 1), 200)

    if selection == "pinned":
        cursor.execute(
            "SELECT id, title, content, pinned, content_updated_at, updated_at "
            "FROM notes WHERE pinned = 1 ORDER BY `order` ASC LIMIT ?",
            (clamped_limit,),
        )
    elif selection == "recent":
        cursor.execute(
            "SELECT id, title, content, pinned, content_updated_at, updated_at "
            "FROM notes ORDER BY COALESCE(content_updated_at, updated_at) DESC LIMIT ?",
            (clamped_limit,),
        )
    else:
        cursor.execute(
            "SELECT id, title, content, pinned, content_updated_at, updated_at "
            "FROM notes ORDER BY pinned DESC, `order` ASC LIMIT ?",
            (clamped_limit,),
        )

    rows = []
    for row_id, title, content, pinned, content_updated_at, updated_at in cursor.fetchall():
        preview = plain_text_from_tiptap(content or "")
        rows.append({
            "id": str(row_id),
            "columns": [title or "Untitled", preview[:100]] if preview else [title or "Untitled"],
            "status": "pinned" if pinned else None,
        })

    return rows


def main():
    request = read_stdin()
    instance_id = request.get("instance_id", "")
    config = request.get("config", {})

    db_path = config.get("db_path", "")
    if not db_path:
        result = {"title": "Maibuk Notes", "summary": "Error: db_path not configured", "rows": []}
        print(json.dumps(result))
        return

    selection = config.get("selection", "all")
    show_preview = config.get("show_preview", True)
    preview_length = int(config.get("preview_length", 100))
    limit = int(config.get("limit", 50))

    db = Path(db_path)
    if not db.exists():
        result = {"title": "Maibuk Notes", "summary": f"Database not found: {db_path}", "rows": []}
        print(json.dumps(result))
        return

    try:
        uri = f"file:{db.absolute()}?mode=ro"
        conn = sqlite3.connect(uri, uri=True, timeout=0.25)
        conn.execute("PRAGMA query_only=ON")
        conn.execute("PRAGMA busy_timeout=250")

        cursor = conn.cursor()
        check_schema(cursor)

        rows = query_notes(cursor, selection, limit)
        conn.close()

        result = {
            "title": "Maibuk Notes",
            "summary": f"{len(rows)} notes",
            "rows": rows,
        }
        print(json.dumps(result))

    except ValueError as e:
        result = {"title": "Maibuk Notes", "summary": f"Schema error", "rows": [
            {"id": "error", "columns": [str(e)], "status": "error"}
        ]}
        print(json.dumps(result))
    except sqlite3.Error as e:
        result = {"title": "Maibuk Notes", "summary": f"DB error", "rows": [
            {"id": "error", "columns": [str(e)], "status": "error"}
        ]}
        print(json.dumps(result))


if __name__ == "__main__":
    main()
