---
description: "Task list for feature SPEC-XXX — [FEATURE NAME]"
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `.specs/features/SPEC-XXX-<feature-name>/`
**Prerequisites**: `plan.md` (required), `spec.md` (required for user stories)
**Tests**: Test tasks are OPTIONAL — include them only if the feature spec explicitly requests them.
**Organization**: Tasks are grouped by user story so each story can be implemented, tested, and delivered independently (Spec Kit convention).

## Format: `[ID] [P?] [USn] Description`

- **[P]**: can run in parallel (different files, no dependencies)
- **[USn]**: which user story this task belongs to (US1, US2, US3 …)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for this feature.

- [ ] T001 Create feature scaffolding per `plan.md` structure decision
- [ ] T002 Add feature-specific dependencies to `Cargo.toml` (justify against cli-spec §5 / D11)
- [ ] T003 [P] Configure/extend linting and formatting config (NFR4)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T004 [P] Shared types/errors for this feature (thiserror, E-code envelopes per cli-spec §7)
- [ ] T005 [P] Deterministic output plumbing (NFR2 — no timestamps by default)
- [ ] T006 [P] Test harness / golden-test fixtures (D8)

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel.

---

## Phase 3: User Story 1 — [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of what this story delivers]
**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 1 (OPTIONAL — only if tests requested) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation.**

- [ ] T010 [P] [US1] Unit test for [module] in tests/…/test_[name].rs
- [ ] T011 [P] [US1] Integration test for [user journey]

### Implementation for User Story 1

- [ ] T012 [P] [US1] Create [component] in src/<module>/[component].rs
- [ ] T013 [P] [US1] Create [component] in src/<module>/[component].rs
- [ ] T014 [US1] Wire [service/command] in src/<module>/mod.rs (depends on T012, T013)
- [ ] T015 [US1] Add validation and error handling (E-code envelope + rule ID)
- [ ] T016 [US1] Add `list`/`validate` surface if applicable (cli-spec command surface)

**Checkpoint**: User Story 1 should be fully functional and testable independently.

---

## Phase 4: User Story 2 — [Title] (Priority: P2)

**Goal**: [Brief description of what this story delivers]
**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 2 (OPTIONAL) ⚠️

- [ ] T017 [P] [US2] Unit test for [module]
- [ ] T018 [P] [US2] Integration test for [user journey]

### Implementation for User Story 2

- [ ] T019 [P] [US2] Create [component] in src/<module>/[component].rs
- [ ] T020 [US2] Implement [service] in src/<module>/[service].rs
- [ ] T021 [US2] Integrate with User Story 1 components (if needed)

**Checkpoint**: User Stories 1 AND 2 should both work independently.

---

## Phase 5: User Story 3 — [Title] (Priority: P3)

**Goal**: [Brief description of what this story delivers]
**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 3 (OPTIONAL) ⚠️

- [ ] T022 [P] [US3] Unit test for [module]
- [ ] T023 [P] [US3] Integration test for [user journey]

### Implementation for User Story 3

- [ ] T024 [P] [US3] Create [component] in src/<module>/[component].rs
- [ ] T025 [US3] Implement [service] in src/<module>/[service].rs

**Checkpoint**: All user stories should now be independently functional.

---

[Add more user story phases as needed, following the same pattern.]

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories.

- [ ] TXXX [P] Documentation updates (`.specs/` + README)
- [ ] TXXX Code cleanup and refactoring
- [ ] TXXX [P] Performance optimization (NFR3 budget)
- [ ] TXXX [P] Golden tests for the full reference tree (D8)
- [ ] TXXX Security hardening
- [ ] TXXX Update spec status → `released` once all ACs met

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story phases (3+)**: Depends on Foundational; each story phase is otherwise independent and can run in parallel

### Task Dependencies

| Task | Depends on |
|------|-----------|
| T014 | T012, T013 |
| T021 | T019, T020 (+ US1 components) |
| … | … |

### Final Checkpoint

- [ ] All acceptance criteria from `spec.md` pass (Given/When/Then, per C10)
- [ ] `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all green (C12)
- [ ] Spec status updated → `released`; review with the user before closing
