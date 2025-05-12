# 🎨 Ultimate Front‑End Design Brief

*(for the visual/interaction designer)*

---

### 1. Project North Star

> **A massively‑multiplayer trivia “party” that feels as live & reactive as Twitch chat, yet is minimal enough to play on a phone with one thumb.**
> Tech stack gives us sub‑50 ms state updates (SpacetimeDB) + edge rendering (Next.js / Vercel). We want UI that celebrates that immediacy and social energy.

---

### 2. Player Flows & Key Screens

| # | Screen / Moment               | Core actions                                              | Real‑time hooks you must visualise                                                                                            |
| - | ----------------------------- | --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| 0 | **Landing**                   | Browse topic cards, see live head‑count, start new lobby. | Card badges animate as counts change.                                                                                         |
| 1 | **Create Lobby** (modal)      | Chip selection or “Ask Agent” chat.                       | Typing indicator from agent when auto‑generating a question set.                                                              |
| 2 | **Waiting Room**              | Avatars join, chat bubbles, host starts game.             | Entry fanfare animation fire on `client_connected`.                                                                           |
| 3 | **Question Round**            | Read prompt, tap answer, micro‑feedback (👍/👎/⚑).        | - Countdown ring (5–20 s).<br>- **Crowd Meter**: live bars showing hover/locked‑in counts.<br>- “First 5 correct” combo glow. |
| 4 | **Round Result**              | See correct answer, streak/latency banners.               | Confetti if correct; shake if wrong.                                                                                          |
| 5 | **Lightning Round** (special) | Same as Q‑round but red timer, double points banner.      | Scheduled reducer triggers this; need distinct colour & SFX.                                                                  |
| 6 | **Game Over**                 | Stats, badges, share GIF, “Play again”.                   | Global leaderboard diff blasting in behind the stats card.                                                                    |
| 7 | **Agent Studio**              | Chat UI, question list preview, publish.                  | Workers AI status chips (“thinking…”, “ready”).                                                                               |

---

### 3. Visual Language

| Element            | Spec                                                                                                                     |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| **Base palette**   | `#12141A` dark slate background. Topic accent colours (90s‑Pop → #FF2D9B electric pink, etc.) drive gradients & buttons. |
| **Typography**     | *Space Grotesk* (timers, numerals, headline), *Inter* (body).                                                            |
| **Cards & sheets** | Glass‑morphism (backdrop‑blur 20 px, rgba(255,255,255,0.08)), 16 px radius, shadow `0 8px 32px rgba(0,0,0,0.35)`.        |
| **Motion**         | Framer‑motion spring, 200 ms enter, 120 ms exit.<br>Elastic overshoot (1.05) on correct‑answer bounce.                   |
| **Emoji bursts**   | 12 px avatars or 🎉, ❤️‍🔥 flying on correct answer & 👍 votes.                                                          |
| **Accessibility**  | All accent/background pairs must hit WCAG AA.<br>Keyboard & reduced‑motion variants.                                     |

---

### 4. Interaction Details to Mock Up

| Component                  | States & Notes                                                                                                     |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| **Topic Card**             | Idle → hover tilt → press. Badge with live count pulses every 2 s.                                                 |
| **Answer Button**          | Default, hover, selected, correct (green glow), wrong (red shake), disabled.                                       |
| **Crowd Meter**            | Four vertical bars grow in real‑time; subtle “water‑fill” texture. 120 ms debounce so it feels fluid.              |
| **Combo Glow**             | If answer within 300 ms of first correct: avatar ring flashes gold for 1 s.                                        |
| **Timer Ring**             | SVG circle stroke animating; colour shifts from accent → orange → red last 3 s.                                    |
| **Lightning Round Banner** | Slides from top, neon streaks; persists 3 s.                                                                       |
| **Secret Role Hint**       | If player is Saboteur, wrong answers subtly highlighted (client sees, others don’t). Need a dedicated style sheet. |
| **Emoji Burst**            | 8–12 sprites, random 30° spread, 600 ms fade.                                                                      |

---

### 5. Responsive Breakpoints

* **Mobile (≤480 px)** – vertically stacked, answer buttons as bottom sheet.
* **Tablet (481‑1023)** – side leaderboard collapsible.
* **Desktop (≥1024)** – 2/3 question, 1/3 leaderboard. Agent Studio only on ≥1024.

---

### 6. Deliverables

1. **Figma file** with:

   * Full flows (frames 0–7 above) for mobile & desktop.
   * Component library: buttons, cards, timer, bars, emoji sprites.
   * Motion specs (use Figma Smart Animate or GIFs).
2. **Exportable assets**

   * SVG icons, sprite sheets, gradient definitions.
3. **Design tokens** (colour, spacing, radius) in JSON or Style Dictionary for Tailwind config.
4. **Accessibility annotation** (contrast matrix, focus order).

---

### 7. Reference Inspo

* **Wordle / Connections** — minimal yet delightful colour language.
* **Kahoot!** — answer buttons ergonomics on mobile.
* **Twitch emote rain** — social burst energy.
* **Among Us** — secret‑role UX hints.

---

### 8. Timeline & Check‑Ins

| Date       | Milestone                                  |
| ---------- | ------------------------------------------ |
| **May 12** | Wireframe walkthrough (low‑fi).            |
| **May 19** | High‑fidelity mockups + motion examples.   |
| **May 26** | Design‑dev hand‑off (tokens, components).  |
| **Jun 02** | UI polish pass after integrated play‑test. |

---

### 9. Technical Constraints & Handoff Notes

* **Next.js App Router, React‑server components.**
* **TailwindCSS** – please align spacing scales (4, 8, 12 px).
* Use **lucide‑react** icons; avoid custom fonts beyond specified.
* Framer‑motion anim definitions should map 1‑to‑1 from spec.
* Theme accent colour is dynamic per lobby (comes from DB). Provide design tokens accordingly.

---

### 10. Success Criteria

* First‑time user can join and answer within **<5 s**.
* Live social cues (crowd meter, emoji bursts) make the room feel “alive”.
* UI supports 1 → 10 000 players with no laggy spinners.
* Visual style is instantly recognisable & meme‑able (share GIF looks great).

---
