#![cfg(feature = "std")]
#![cfg(feature = "derive")]

//! Regression tests for the bugs fixed by the audit pass. Each section is
//! introduced by the bug it covers.

use mem_dbg::*;

#[derive(MemSize, MemDbg)]
struct Inner {
    heavy: Vec<u8>,
}

fn render<T: MemDbg>(value: &T) -> String {
    let mut out = String::new();
    value
        .mem_dbg_on(&mut out, DbgFlags::default())
        .expect("mem_dbg_on");
    out
}

// ---------------------------------------------------------------------------
// Bug 1: `OnceCell<T>::_mem_dbg_rec_on` was a no-op because it called
// `Option::<&T>::_mem_dbg_rec_on` (the empty default) instead of unwrapping
// `self.get()`.
// ---------------------------------------------------------------------------

#[test]
fn oncecell_renders_inner_children() {
    use std::cell::OnceCell;

    let cell: OnceCell<Inner> = OnceCell::new();
    cell.set(Inner {
        heavy: vec![0u8; 64],
    })
    .ok()
    .expect("oncecell set");

    let out = render(&cell);
    assert!(out.contains("OnceCell"), "missing OnceCell line:\n{out}");
    assert!(
        out.contains("heavy"),
        "OnceCell did not recurse into Inner:\n{out}"
    );
}

#[test]
fn oncecell_empty_renders_only_self() {
    use std::cell::OnceCell;

    let cell: OnceCell<Inner> = OnceCell::new();
    let out = render(&cell);
    assert!(out.contains("OnceCell"));
    assert!(!out.contains("heavy"));
}
