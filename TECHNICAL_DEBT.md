# Technical Debt Analysis: Kenichi Project

## What is Technical Debt?

**Technical Debt** is the implied cost of future rework caused by choosing an easy/quick solution now instead of a better approach that would take longer.

### Types of Technical Debt

1. **Deliberate Debt** - Conscious shortcuts to meet deadlines
2. **Accidental Debt** - Unintentional issues from lack of knowledge
3. **Bit Rot** - Code degradation as dependencies/requirements change
4. **Design Debt** - Architecture that doesn't scale with new features

---

## Kenichi's Technical Debt Inventory

### 🔴 Critical (High Priority)

#### 1. Timeline Store Implementation (Deliberate Debt)
**Location**: `src/lib/stores/timeline.ts` (6 lines)

**Current State**:
```typescript
export const clips = writable([]);
export const tracks = writable([]);
```

**Problem**: Stub implementation. No clip management, no magnetic snapping, no ripple delete.

**Impact**: 
- Blocks Phase 5 (Multi-track Composition)
- Users can't edit timelines
- 60% of UI components are non-functional

**Solution Available**: Complete implementation in `guide/Magnetic Timeline Logic.md`

**Remediation Cost**: 4-6 hours  
**Risk if Ignored**: Cannot proceed to Phase 5

---

#### 2. Zero Test Coverage (Accidental Debt)
**Location**: Entire codebase

**Current State**: No unit tests, no integration tests, no E2E tests

**Impact**:
- High risk of regressions during Phase 5+
- No confidence in refactoring
- Bugs discovered in production

**Remediation**:
```rust
// Example: src-tauri/src/engine/decoding.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_seek_accuracy() {
        let mut decoder = VideoDecoder::new("test.mp4").unwrap();
        decoder.seek(5.0).unwrap();
        let (_, pts) = decoder.decode_next_frame().unwrap();
        assert!((pts - 5.0).abs() < 0.1); // Within 100ms
    }
}
```

**Remediation Cost**: 2-3 days for core coverage  
**Risk if Ignored**: Major bugs in Phase 5+

---

#### 3. No CI/CD Quality Gates (Deliberate Debt)
**Location**: `.github/workflows/`

**Current State**: Only release workflow, no PR checks

**Impact**:
- Code quality degradation
- Type errors slip through
- Linting violations accumulate

**Remediation**:
```yaml
# .github/workflows/ci.yml
name: CI
on: [pull_request, push]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: pnpm install
      - run: pnpm run check
      - run: cargo clippy -- -D warnings
      - run: cargo test
```

**Remediation Cost**: 2 hours  
**Risk if Ignored**: Code quality decline

---

### 🟡 Moderate (Medium Priority)

#### 4. Fixed Delta Time in Render Loop (Design Debt)
**Location**: `src-tauri/src/lib.rs:35`

**Current Code**:
```rust
engine_guard.tick(0.016); // Fixed delta for now
```

**Problem**: Assumes perfect 60fps. If loop is slow, time drifts.

**Impact**:
- Inaccurate playback speed
- Audio sync issues (Phase 4.5)

**Remediation**:
```rust
let mut last_time = Instant::now();
loop {
    interval.tick().await;
    let now = Instant::now();
    let dt = now.duration_since(last_time).as_secs_f64();
    last_time = now;
    engine_guard.tick(dt);
}
```

**Remediation Cost**: 30 minutes  
**Risk if Ignored**: Audio desync in Phase 4.5

---

#### 5. println! Instead of Logging Framework (Accidental Debt)
**Location**: Throughout codebase (50+ instances)

**Current Code**:
```rust
println!("Decoder: Seek Complete. Skipped {} frames.", frames_decoded);
```

**Problem**: No log levels, no filtering, clutters production output

**Impact**:
- Difficult debugging in production
- No performance profiling
- Can't disable verbose logs

**Remediation**:
```rust
// Add to Cargo.toml: tracing = "0.1"
use tracing::{info, debug, warn};

debug!("Decoder: Seek Complete. Skipped {} frames.", frames_decoded);
```

**Remediation Cost**: 3-4 hours  
**Risk if Ignored**: Poor production debugging

---

#### 6. Unsafe Send Implementation (Design Debt)
**Location**: `src-tauri/src/engine/decoding.rs:17`

**Current Code**:
```rust
unsafe impl Send for VideoDecoder {}
```

**Problem**: Bypasses Rust's thread safety guarantees

**Impact**:
- Potential data races if Mutex is removed
- Unsafe code requires extra scrutiny

**Justification**: FFmpeg's raw pointers aren't Send. We protect with Mutex.

**Remediation**: Document safety invariants thoroughly

**Remediation Cost**: 1 hour (documentation)  
**Risk if Ignored**: Future maintainers might violate safety

---

### 🟢 Low (Nice to Have)

#### 7. Magic Numbers (Code Smell)
**Location**: Various files

**Examples**:
```rust
let max_skip = 120; // What does 120 mean?
let frame_duration = 0.033; // Why 0.033?
```

**Remediation**:
```rust
const MAX_FRAMES_TO_SKIP: usize = 120; // 2 seconds at 60fps
const DEFAULT_FRAME_DURATION: f64 = 1.0 / 30.0; // 30fps fallback
```

**Remediation Cost**: 1 hour  
**Risk if Ignored**: Reduced code readability

---

#### 8. No API Documentation (Accidental Debt)
**Location**: All public functions

**Current State**: Inline comments, no rustdoc/TSDoc

**Remediation**:
```rust
/// Seeks to a specific timestamp in the video.
///
/// # Arguments
/// * `timestamp_seconds` - Target time in seconds
///
/// # Returns
/// * `Ok(())` if seek successful
/// * `Err` if seek fails or timestamp invalid
pub fn seek(&mut self, timestamp_seconds: f64) -> Result<()> {
```

**Remediation Cost**: 4-6 hours  
**Risk if Ignored**: Onboarding friction for contributors

---

## Technical Debt Metrics

### Debt Ratio
**Formula**: `(Remediation Cost) / (Total Development Time)`

**Current Debt**:
- Critical: 3-4 days
- Moderate: 1 day
- Low: 6-7 hours

**Total**: ~5-6 days

**Total Development Time**: ~30 days (Phases 1-4)

**Debt Ratio**: 16-20% (Acceptable for MVP stage)

**Industry Benchmark**: <20% is healthy for early-stage projects

---

### Debt Trend

```
Phase 1-3: Low debt (focused on foundation)
Phase 4: Moderate debt (frame pacing shortcuts)
Phase 5: Risk of HIGH debt (if timeline not implemented)
```

**Recommendation**: Address critical debt before Phase 5.

---

## Remediation Strategy

### Immediate (Before Phase 5)
1. ✅ **Implement Timeline Store** (4-6 hours)
   - Use code from `guide/Magnetic Timeline Logic.md`
   - Add magnetic snapping
   - Implement ripple delete

2. ✅ **Add CI Workflow** (2 hours)
   - Linting (clippy, svelte-check)
   - Type checking
   - Basic tests

3. ✅ **Fix Render Loop Delta** (30 minutes)
   - Use actual time delta
   - Prevents audio desync

### Short-term (Phase 5-6)
1. **Add Unit Tests** (2-3 days)
   - VideoDecoder tests
   - Timeline operation tests
   - Playback state tests

2. **Logging Framework** (3-4 hours)
   - Replace println! with tracing
   - Add log levels
   - Configure production logging

### Long-term (Phase 7+)
1. **API Documentation** (4-6 hours)
   - Rustdoc for all public APIs
   - TSDoc for Svelte components
   - Architecture diagrams

2. **Integration Tests** (1 week)
   - End-to-end timeline tests
   - Export pipeline tests
   - Performance benchmarks

---

## Cost-Benefit Analysis

### Paying Down Debt Now

**Benefits**:
- ✅ Faster Phase 5+ development
- ✅ Fewer bugs in production
- ✅ Easier onboarding for contributors
- ✅ Confidence in refactoring

**Costs**:
- ⏱️ 5-6 days of development time
- 📅 Delays Phase 5 start

### Ignoring Debt

**Benefits**:
- ⏱️ Faster feature delivery (short-term)

**Costs**:
- 🐛 High bug rate in Phase 5+
- 🔥 Potential rewrite of timeline (10+ days)
- 😰 Stress during debugging
- 📉 Code quality decline

---

## Recommendations

### Priority 1: Critical Debt (Before Phase 5)
```
Timeline Store Implementation → 6 hours
CI Workflow → 2 hours
Render Loop Fix → 30 minutes
---
Total: 1 day
```

**Impact**: Unblocks Phase 5, prevents major rework

### Priority 2: Moderate Debt (During Phase 5)
```
Unit Tests → 2 days
Logging Framework → 4 hours
---
Total: 2.5 days
```

**Impact**: Reduces bug rate, improves debugging

### Priority 3: Low Debt (Phase 7+)
```
API Documentation → 6 hours
Magic Number Cleanup → 1 hour
---
Total: 7 hours
```

**Impact**: Improves maintainability

---

## Conclusion

**Current Debt Level**: Moderate (16-20%)  
**Trend**: Increasing (risk of high debt in Phase 5)  
**Recommendation**: **Pay down critical debt NOW**

**Action Plan**:
1. Spend 1 day addressing critical debt
2. Proceed to Phase 5 with confidence
3. Address moderate debt during Phase 5-6
4. Tackle low-priority debt in Phase 7+

**ROI**: Investing 1 day now saves 10+ days of rework later.
