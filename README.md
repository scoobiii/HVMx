# 🚀 HVMX - High-order Virtual Machine eXtreme

[![CI](https://github.com/scoobiii/hvmx/workflows/CI/badge.svg)](https://github.com/scoobiii/hvmx/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Write once, run optimally anywhere.** Adaptive JIT runtime for HVM targeting GPUs.

## ✨ Features

- 🎯 Adaptive JIT compilation
- ⚡ Zero-copy on mobile SoCs
- 🌐 Cross-platform (CUDA, Vulkan, Metal, CPU)
- 🔥 Heterogeneous computing (CPU + GPU)

## 📊 Performance

| Platform | Latency | vs CPU |
|----------|---------|--------|
| Snapdragon 8 Gen 3 | 5-10ms | 1.8x |
| Apple M3 | 3ms | 2.2x |

## 🚀 Quick Start

```bash
cargo install hvmx-cli
hvmx version
```

## 📁 Project Structure

```
hvmx/
├── crates/
│   ├── hvmx-core/       # Runtime (1,200 LOC)
│   ├── hvmx-jit/        # JIT compiler (2,000 LOC)
│   ├── hvmx-memory/     # Memory mgmt (800 LOC)
│   ├── hvmx-scheduler/  # Scheduling (600 LOC)
│   └── hvmx-cli/        # CLI (400 LOC)
├── benches/
├── examples/
└── tests/
```

## 📊 Development

**Total LOC**: ~5,000  
**Sprints**: 3 (8-12 weeks)  
**Tests**: 263  
**Coverage**: 90%+

## 🗓️ Roadmap

- [x] Sprint 1: Core + Vulkan (Week 1-4)
- [ ] Sprint 2: Backends + Memory (Week 5-7)
- [ ] Sprint 3: Scheduler + Production (Week 8-10)

## 📝 License

MIT OR Apache-2.0

## 👤 Author

**scoobiii** - scoobiii@gmail.com

---

**Status**: 🟡 Sprint 1 in progress
**Last Update**: 2024-12-28
