# SCM2 null Clip panic fix

SCM2 startup trace showed:

- `org.kwis.msp.media.Player.stop(Lorg/kwis/msp/media/Clip;)Z args=[0]`
- followed by RustJava `class_instance.rs` panic: `called Option::unwrap() on a None value`

The game passes a null `Clip` while stopping audio before any clip has been loaded. The existing WIE implementation called `Clip::player()` unconditionally, which dereferenced the null Java object and crashed RustJava.

This patch adds null guards to the `Clip` overloads of both `Player.stop()` and `Player.play()`. A null clip now returns `false` instead of dereferencing it.
