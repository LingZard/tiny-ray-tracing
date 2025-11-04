## Reference
https://raytracing.github.io/books/RayTracingInOneWeekend.html

## Note on design & style

 - A Rust take on “Ray Tracing in One Weekend”, aimed at learning rather than production.
 - Data structures are intentionally open and straightforward — minimal encapsulation and guardrails.
 - Examples may use `unwrap`/`expect`; production code should prefer structured error handling.
 - Passing by value vs by reference is handled pragmatically; readability takes priority.
 - Small `Copy` types often go by value; borrowing or cloning is used when it improves clarity.
 - Inputs are assumed to be well‑formed (e.g., UVs in [0,1], RGB in range).