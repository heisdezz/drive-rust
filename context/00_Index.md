---
title: Rust-Drive Project Hub (MOC)
tags: [moc, rust, gpui, drive-manager, architecture]
created: 2026-10-31
---

# Rust-Drive Project Hub (Map of Content)

Welcome to the **Rust-Drive** documentation and context base. This vault contains the design, architectural specifications, database models, UI component blueprints, and incremental implementation steps for porting [[heisdezz/drive|D-Stream Drive Desktop]] from Electrobun (TypeScript + SolidJS + Bun) to a high-performance native Rust application using **GPUI** and **[[gpui-kit]]**.

---

## 📌 Quick Links & Knowledge Base

- **[[01_Original_Codebase_Analysis]]** — Comprehensive analysis of the reference Electrobun application, RPC interfaces, store designs, and data flow.
- **[[02_Architecture]]** — System architecture, threading model, and unified single-process design in Rust.
- **[[03_Database_Design]]** — Portable SQLite schema, simple `rusqlite` query layer, triggers, and count caching.
- **[[04_Drive_Detection]]** — Cross-platform removable storage and disk volume discovery using `sysinfo`.
- **[[05_File_Operations_and_Scanner]]** — Directory traversal, ignore filters, safe atomic file relocation, and album auto-classification.
- **[[06_UI_Components]]** — GPUI and `gpui-kit` component specifications (Sidebar, Drive Cards, Media Cards, Modals, Players).
- **[[07_State_Management]]** — GPUI Model-View architecture, reactive app state, and asynchronous background worker channels.
- **[[08_Implementation_Steps]]** — Detailed, phase-by-phase implementation plan designed for incremental component-by-component development.

---

## 🎯 Porting Goals & Core Principles

1. **Portable Drive-Centric Storage**
   - Media metadata resides in `.media_library.db` directly on the designated external drive folder.
   - Moving the hard drive to another machine preserves all albums, tags, and catalog records.

2. **Explicit User Control & Non-Destructive Operations**
   - No surprise background file relocations.
   - Files are moved explicitly during scans or consolidation actions.
   - Rollback tracking and safe collision renaming (`filename_1.jpg`) prevent data loss.

3. **High Performance & Low Footprint (Native Rust + GPUI)**
   - Replaces WebKit/Blink and Bun runtimes with a single native binary compiled with Rust.
   - GPU-accelerated rendering with GPUI and `gpui-kit` provides sub-millisecond frame times and smooth animations even with tens of thousands of catalog items.

4. **Simple & Direct SQLite Logic**
   - Avoid bloated ORMs. Use straightforward `rusqlite` queries with prepared statements and transactions.

5. **Component-by-Component Development**
   - Develop and test the app piece by piece: foundational models -> drive selector -> media card -> album gallery -> scanner -> player.

---

## 🛠 Tech Stack Overview

| Layer | Original Stack | Rust Port Stack |
| :--- | :--- | :--- |
| **GUI Framework** | Electrobun (WebKit) + SolidJS + DaisyUI | **GPUI** + **gpui-kit** (Zed GPU-accelerated framework) |
| **Component Primitives**| Kobalte + Tailwind CSS | **gpui-kit** (Buttons, Dialogs, Inputs, Themes, Icons) |
| **Runtime & Language** | Bun (TypeScript) + V8/JavaScriptCore | **Rust 1.99+ (Native Binary)** |
| **Database** | `bun:sqlite` | **`rusqlite`** (Bundled SQLite with FTS5) |
| **Drive Detection** | `systeminformation` (Node/Bun) | **`sysinfo`** crate |
| **Thumbnails** | FFmpeg CLI + Bun child process | `image` crate (photos) + `ffmpeg` CLI (video frames) |
| **Async Tasks** | JavaScript Promises + Event Loop | GPUI Background Executor (`cx.background_executor()`) |
