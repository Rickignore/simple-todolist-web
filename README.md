# Todo List App — Rust Axum + Vanilla JS + Docker

Aplikasi Todo sederhana dengan backend **Rust (Axum)** dan frontend **Vanilla JavaScript**.  
Aplikasi ini **tanpa database** — data disimpan di memori, sehingga sangat ringan dan cepat.

Proyek ini dirancang untuk mudah di-*deploy* ke **Railway**, **Docker**, atau server Linux kecil.

---

## 🚀 Fitur

### Backend (Rust Axum)
- CRUD Todo:
  - Create Todo  
  - Read (List Todo)  
  - Update status  
  - Delete Todo  
- Penyimpanan in-memory (Mutex)
- Serve UI statis (ServeDir)
- Binary **super kecil** berkat optimasi:
  - LTO
  - opt-level z
  - strip = true

### Frontend (Vanilla JS)
- UI modern minimalis
- Animasi hover
- Mobile friendly
- Tanpa framework → cepat dan ringan
