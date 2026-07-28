"""Tests for the Maibuk Notes reference plugin."""

import json
import sqlite3
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import main


class TestMaibukPlugin(unittest.TestCase):

    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.db_path = Path(self.tmpdir.name) / "maibuk.db"
        self.conn = sqlite3.connect(str(self.db_path))
        self.conn.execute("""
            CREATE TABLE notes (
                id INTEGER PRIMARY KEY,
                book_id INTEGER NOT NULL DEFAULT 1,
                title TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                pinned INTEGER NOT NULL DEFAULT 0,
                "order" INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT '2025-01-01T00:00:00',
                updated_at TEXT NOT NULL DEFAULT '2025-01-01T00:00:00',
                content_updated_at TEXT
            )
        """)
        self.conn.commit()

    def tearDown(self):
        self.conn.close()
        self.tmpdir.cleanup()

    def _add_note(self, title="Note", content="", pinned=0, order=0):
        self.conn.execute(
            "INSERT INTO notes (title, content, pinned, `order`) VALUES (?, ?, ?, ?)",
            (title, content, pinned, order),
        )
        self.conn.commit()

    def _run(self, config=None):
        if config is None:
            config = {"db_path": str(self.db_path)}
        request = json.dumps({
            "protocol": 1,
            "instance_id": "test",
            "config": config,
        })
        with patch("sys.stdin.read", return_value=request):
            with patch("sys.stdout", new_callable=io.StringIO) as mock_stdout:
                main.main()
                return json.loads(mock_stdout.getvalue())

    def test_returns_notes(self):
        self._add_note("Hello", pinned=1)
        result = self._run()
        self.assertEqual(result["title"], "Maibuk Notes")
        self.assertEqual(len(result["rows"]), 1)
        self.assertEqual(result["rows"][0]["columns"][0], "Hello")

    def test_empty_database(self):
        result = self._run()
        self.assertEqual(len(result["rows"]), 0)

    def test_missing_db_path_returns_error(self):
        result = self._run(config={})
        self.assertIn("Error", result["summary"])

    def test_pinned_selection(self):
        self._add_note("A", pinned=1, order=1)
        self._add_note("B", pinned=1, order=2)
        self._add_note("C", pinned=0)
        result = self._run(config={"db_path": str(self.db_path), "selection": "pinned"})
        self.assertEqual(len(result["rows"]), 2)

    def test_incompatible_schema_returns_error(self):
        self.conn.execute("ALTER TABLE notes DROP COLUMN title")
        self.conn.commit()
        result = self._run()
        self.assertEqual(result["rows"][0]["status"], "error")

    def test_wal_visibility(self):
        self.conn.execute("PRAGMA journal_mode=WAL")
        self.conn.commit()
        self._add_note("WAL test")
        result = self._run()
        self.assertEqual(len(result["rows"]), 1)

    def test_read_only_enforced(self):
        self._add_note("Readonly")
        result = self._run()
        self.assertEqual(len(result["rows"]), 1)
        self.assertEqual(result["rows"][0]["columns"][0], "Readonly")

    def test_limit_clamping(self):
        for i in range(50):
            self._add_note(f"Note {i}")
        result = self._run(config={"db_path": str(self.db_path), "limit": 300})
        self.assertLessEqual(len(result["rows"]), 200)


import io

if __name__ == "__main__":
    unittest.main()
