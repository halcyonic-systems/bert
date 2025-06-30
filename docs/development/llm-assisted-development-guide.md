# LLM-Assisted Development Guide for BERT

## Executive Summary

This guide synthesizes proven patterns for accelerating feature development in the BERT project using LLM assistance. Based on successful implementations of complex features like controls menus and theme toggles, these practices enable **3-5x faster development** while maintaining **high code quality** and **architectural consistency**.

## Core Principles

### 1. Constraint-Driven Development
**Well-structured constraints enable faster, higher-quality development.**

- **Cursor Rules as Architecture**: Your `.mdc` files serve as living architecture documentation that both humans and AI can follow
- **Patterns Over Invention**: Established patterns eliminate decision fatigue and ensure consistency
- **Guardrails, Not Restrictions**: Constraints channel creativity productively rather than limiting it

### 2. Incremental Validation
**Test early, test often, fail fast.**

- **Compilation-Driven Development**: Test compilation after each significant change
- **Phase-Gate Approach**: Complete each phase before moving to the next
- **Error-Driven Learning**: Use compilation errors as immediate feedback loops

### 3. Documentation as Code
**Your cursor rules effectively become your system architecture.**

- **Self-Documenting Patterns**: Consistent naming and structure reduce cognitive load
- **API as Documentation**: Well-designed component interfaces serve as usage guides
- **Living Standards**: Cursor rules evolve with the codebase

## The Proven Workflow

### Phase 1: Discovery & Planning (10-15 minutes)
```
1. Search existing patterns in codebase
2. Identify integration points and file modifications
3. Propose specific technical approach
4. Get user confirmation before proceeding
```

**Key Success Factor**: Never start coding without understanding existing patterns.

### Phase 2: Incremental Implementation (60-80% of time)
```
1. Create/modify one file at a time
2. Test compilation after each change
3. Follow established patterns religiously
4. Fix errors immediately before proceeding
```

**Key Success Factor**: Maintain working state at all times.

### Phase 3: Integration & Polish (15-25% of time)
```
1. Apply consistent styling and interactions
2. Add proper documentation
3. Test complete user workflow
4. Prepare clean commit
```

**Key Success Factor**: Don't skip the polish—it's what makes features feel integrated.

## Critical Success Patterns

### Pattern 1: Modal Component Implementation
```rust
// Standard modal pattern that works every time
#[component]
pub fn MyModal(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-4xl max-h-[90vh] overflow-y-auto m-4">
                    // Content here
                </div>
            </div>
        </Show>
    }
}
```

### Pattern 2: Component Integration
```rust
// In App component - signal management
let (feature_visible, set_feature_visible) = signal(false);

// Component usage with proper callback
<MyComponent 
    visible=feature_visible 
    on_close=Callback::new(move |_| set_feature_visible.set(false))
/>
```

### Pattern 3: File Structure
```
1. Create: src/leptos_app/components/my_component.rs
2. Export: Add to src/leptos_app/components/mod.rs
3. Import: Use in src/leptos_app/mod.rs
4. Integrate: Add to App component
```

## Common Pitfalls and Solutions

### Pitfall 1: API Version Mismatches
**Problem**: Using outdated API patterns (e.g., Leptos callback syntax)
**Solution**: Check version-specific documentation and test compilation frequently

### Pitfall 2: Pattern Deviation
**Problem**: Inventing new patterns instead of following established ones
**Solution**: Always research existing implementations first

### Pitfall 3: Integration Complexity
**Problem**: Underestimating how components integrate with existing systems
**Solution**: Map integration points before implementation

## Quality Metrics

### High-Quality Implementation Indicators
- ✅ **Compilation**: No errors, minimal warnings
- ✅ **Integration**: Seamless with existing features
- ✅ **Consistency**: Follows established patterns
- ✅ **Performance**: No noticeable impact
- ✅ **Maintainability**: Well-documented and extensible

### Process Efficiency Indicators
- ✅ **Speed**: Minimal back-and-forth during implementation
- ✅ **Accuracy**: No major rework required
- ✅ **Reusability**: Patterns documented for future use
- ✅ **Learning**: Insights captured for continuous improvement

## Technology-Specific Considerations

### Leptos (Frontend Framework)
- **Signals**: Use `Signal<T>` for reactive props, `move ||` for reactive reads
- **Callbacks**: Use `.run()` method in Leptos 0.7+
- **Components**: Follow `#[component]` and `#[prop(into)]` patterns

### Bevy (Game Engine)
- **Entities**: Use established query patterns and component bundles
- **Resources**: Ensure proper initialization and access patterns
- **Systems**: Follow existing system organization and event handling

### Tauri (Desktop Framework)
- **Build Process**: Be aware of frontend/backend compilation dependencies
- **Port Conflicts**: Kill existing processes when encountering port issues
- **Development**: Use `cargo tauri dev` for integrated development experience

## Optimization Strategies

### For Simple Features (< 2 hours)
- Combine discovery and implementation phases
- Focus on pattern reuse over customization
- Minimal documentation beyond code comments

### For Complex Features (> 4 hours)
- Add intermediate checkpoints within phases
- Create detailed implementation plan
- Consider breaking into smaller features

### For Team Development
- Document new patterns immediately
- Share successful implementations as templates
- Maintain pattern library for common use cases

## Measuring Success

### Development Velocity
- **Target**: 3-5x faster implementation than traditional development
- **Measure**: Time from requirement to working feature
- **Optimize**: Reduce discovery and debugging time

### Code Quality
- **Target**: Zero regressions, minimal technical debt
- **Measure**: Compilation success rate, integration smoothness
- **Optimize**: Improve pattern documentation and error prevention

### Knowledge Transfer
- **Target**: Patterns reusable by other developers
- **Measure**: Documentation completeness, pattern adoption
- **Optimize**: Capture and share successful implementations

## Future Evolution

### Continuous Improvement
- **Pattern Documentation**: Capture new successful patterns immediately
- **Error Prevention**: Document common mistakes and solutions
- **Process Refinement**: Evolve workflow based on experience

### Scaling Considerations
- **Team Adoption**: Train team members on cursor rules and patterns
- **Pattern Library**: Build comprehensive library of reusable components
- **Automation**: Consider automating repetitive pattern implementations

## Conclusion

LLM-assisted development with well-structured cursor rules transforms feature implementation from a time-consuming, error-prone process into a fast, predictable, and high-quality workflow. The key is treating cursor rules not as restrictions, but as **architectural guardrails** that enable both human and AI decision-making to converge on consistent, maintainable solutions.

**The result**: Features that feel like they've always been part of the application, implemented in a fraction of the traditional time, with quality that meets or exceeds hand-crafted code.

---

*This guide is a living document. Update it as new patterns emerge and the development process evolves.* 