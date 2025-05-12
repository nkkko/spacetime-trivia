# TODO – Spacetime Trivia

> A living backlog of engineering work.  Keep it tidy ✨

---

## How to use this TODO

1. **Syntax** – Every line is a Markdown task list item.
   ```md
   - [ ] [P1] short imperative description _(owner) – notes_
   - [x] [P2] completed item
   ```
2. **Priorities**
   * **[P0]** – Fire-fight / blocker, fix within hours.
   * **[P1]** – Core sprint commitment (must ship this iteration).
   * **[P2]** – Nice-to-have, pull when bandwidth.
   * **[P3]** – Icebox / future idea.
3. **Ownership** – Append GitHub @handle in parentheses.  Single owner.
4. **Tags** – Add `#frontend`, `#backend`, `#infra`, `#design`, `#docs` inline for quick grep.
5. **Status** – Mark as `[x]` only after PR merged & deployed to prod.
6. **Ordering** – Keep list roughly priority-then-chronology within each section.
7. **Edits** – PRs that change this file must pass review; squash commits.

---

## Sprint S1 – MVP loop *(JTBD J-1 → J-4 focus)*

### Backend / Game Logic
- [x] [P1] Implement `player`, `lobby`, `active_round`, `answer` tables in STDB (@alice) #backend
- [x] [P1] Code reducers `join_lobby`, `start_game`, `submit_answer`, `score_round` (@alice) #backend
- [x] [P1] Bootstrap 10 canned questions into `question_bank` (@bob) #backend
- [x] [P1] Write unit tests for reducers with 3 bot clients (@alice) #backend #tests
- [x] [P2] Elo rating calculation helper in Rust util crate (@carol) #backend

### Frontend
- [x] [P1] Quick-Play landing page → lobby match-making flow (@dave) #frontend – UI screens merged
- [x] [P1] Question card component with timer & answers grid (@dave) #frontend – UI + TimerRing merged
- [ ] [P1] WebSocket integration via STDB JS SDK (@dave) #frontend
- [x] [P1] Score overlay after each round (@erin) #frontend – RoundResultScreen merged
- [ ] [P2] Basic avatar selector for guest players (@erin) #frontend

### Infra & DevOps
- [ ] [P1] Docker compose for local STDB instance + Next.js (@frank) #infra
- [ ] [P2] Vercel preview deployment hook with staging STDB URL (@frank) #infra

### Design / UX
- [ ] [P1] Mobile-first wireframes for landing, question, score screens (@grace) #design
- [ ] [P2] Sound FX for correct / wrong / combo (@heidi) #design

---

## Sprint S2 – Crowd Meter & Lightning Round *(JTBD J-3 & J-4)*

- [x] [P1] Wire `CrowdMeter` component to live STDB subscription (#frontend #backend)
      _bar-chart UI done; needs data hook_ (@dave)
- [x] [P1] Implement `lightning_tick` scheduled reducer + double-points logic (@alice) #backend
- [x] [P1] Neon banner CSS animation for Lightning Round (@grace) #design – LightningBanner merged
- [ ] [P2] Server-side metrics for CrowdMeter latency p95 in Prometheus (@frank) #infra

---

## Sprint S3 – LLM-generated Question Sets *(JTBD J-6)*

- [ ] [P1] Cloudflare Workflow script `LLMWorker` calling Workers AI (@bob) #backend #ai
- [x] [P1] Reducer `request_agent_work` + job queue tables (@alice) #backend
- [ ] [P2] Agent Studio chat UI for creators (@erin) #frontend

---

## Sprint S4 – Feedback Pipeline *(JTBD J-5 & J-11)*

- [ ] [P1] Thumb-up / down / flag buttons wired to `vote_question` reducer (@dave) #frontend
- [ ] [P1] Nightly Workflow `feedback-snapshot` → R2 JSONL dataset (@bob) #infra #ai
- [ ] [P2] Grafana dashboard for WorkersAI error_rate & R2 cost drift (@frank) #infra

---

## Sprint S5 – Agent Marketplace *(JTBD J-10)*

- [x] [P1] `agent_registry` table + upload flow (WASM hash, quota) (@alice) #backend
- [ ] [P1] AgentRunner service (could be Worker) sandboxing & metering (@bob) #backend #infra
- [ ] [P2] Marketplace web page listing rated agents (@erin) #frontend

---

## Sprint S6 – Battle-Royale & Global Leaderboard *(Stretch)*

- [ ] [P2] Scale lobby capacity to 10 k connections load-test (@frank) #infra
- [ ] [P2] Global leaderboard API & UI rail (@dave) #frontend #backend

---

## Icebox
- [ ] [P3] Replay highlight GIF generator *(JTBD J-8)* (@heidi) #backend #frontend
- [ ] [P3] Credibility score badge calculation algo *(JTBD J-9)* (@alice) #backend
- [ ] [P3] Refactor initial question bootstrapping to rely on agent population; minimize/remove hardcoded questions in lib.rs init function (@developer) #backend #architecture