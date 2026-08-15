SCM2 Android build fix

Fixes Rust borrow-checker error introduced in MC_knlGetProgramName patch.
The AID is copied into an owned String before mutably borrowing context.
