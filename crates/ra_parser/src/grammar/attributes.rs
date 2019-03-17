use super::*;

pub(super) fn inner_attributes(p: &mut Parser) {
    while p.current() == POUND && p.nth(1) == EXCL {
        attribute(p, true)
    }
}

pub(super) fn outer_attributes(p: &mut Parser) -> bool {
    let mut has_attrs = false;
    while p.at(POUND) {
        has_attrs = true;
        attribute(p, false)
    }
    has_attrs
}

fn attribute(p: &mut Parser, inner: bool) {
    let attr = p.start();
    assert!(p.at(POUND));
    p.bump();

    if inner {
        assert!(p.at(EXCL));
        p.bump();
    }

    if p.at(L_BRACK) {
        items::token_tree(p);
    } else {
        p.error("expected `[`");
    }
    attr.complete(p, ATTR);
}
