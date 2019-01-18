use super::*;

pub(super) const TYPE_FIRST: TokenSet = paths::PATH_FIRST.union(token_set![
    L_PAREN, EXCL, STAR, L_BRACK, AMP, UNDERSCORE, FN_KW, UNSAFE_KW, EXTERN_KW, FOR_KW, IMPL_KW,
    DYN_KW, L_ANGLE,
]);

const TYPE_RECOVERY_SET: TokenSet = token_set![R_PAREN, COMMA];

pub(super) fn type_(p: &mut Parser) {
    type_with_bounds_cond(p, true);
}

pub(super) fn type_no_bounds(p: &mut Parser) {
    type_with_bounds_cond(p, false);
}

fn type_with_bounds_cond(p: &mut Parser, allow_bounds: bool) {
    match p.current() {
        L_PAREN => paren_or_tuple_type(p),
        EXCL => never_type(p),
        STAR => pointer_type(p),
        L_BRACK => array_or_slice_type(p),
        AMP => reference_type(p),
        UNDERSCORE => placeholder_type(p),
        FN_KW | UNSAFE_KW | EXTERN_KW => fn_pointer_type(p),
        FOR_KW => for_type(p),
        IMPL_KW => impl_trait_type(p),
        DYN_KW => dyn_trait_type(p),
        // Some path types are not allowed to have bounds (no plus)
        L_ANGLE => path_type_(p, allow_bounds),
        _ if paths::is_path_start(p) => path_type_(p, allow_bounds),
        _ => {
            p.err_recover("expected type", TYPE_RECOVERY_SET);
        }
    }
}

pub(super) fn ascription(p: &mut Parser) {
    p.expect(COLON);
    type_(p)
}

fn paren_or_tuple_type(p: &mut Parser) {
    assert!(p.at(L_PAREN));
    let m = p.start();
    p.bump();
    let mut n_types: u32 = 0;
    let mut trailing_comma: bool = false;
    while !p.at(EOF) && !p.at(R_PAREN) {
        n_types += 1;
        type_(p);
        if p.eat(COMMA) {
            trailing_comma = true;
        } else {
            trailing_comma = false;
            break;
        }
    }
    p.expect(R_PAREN);

    let kind = if n_types == 1 && !trailing_comma {
        // test paren_type
        // type T = (i32);
        PAREN_TYPE
    } else {
        // test unit_type
        // type T = ();

        // test singleton_tuple_type
        // type T = (i32,);
        TUPLE_TYPE
    };
    m.complete(p, kind);
}

// test never_type
// type Never = !;
fn never_type(p: &mut Parser) {
    assert!(p.at(EXCL));
    let m = p.start();
    p.bump();
    m.complete(p, NEVER_TYPE);
}

fn pointer_type(p: &mut Parser) {
    assert!(p.at(STAR));
    let m = p.start();
    p.bump();

    match p.current() {
        // test pointer_type_mut
        // type M = *mut ();
        // type C = *mut ();
        MUT_KW | CONST_KW => p.bump(),
        _ => {
            // test_err pointer_type_no_mutability
            // type T = *();
            p.error(
                "expected mut or const in raw pointer type \
                 (use `*mut T` or `*const T` as appropriate)",
            );
        }
    };

    type_no_bounds(p);
    m.complete(p, POINTER_TYPE);
}

fn array_or_slice_type(p: &mut Parser) {
    assert!(p.at(L_BRACK));
    let m = p.start();
    p.bump();

    type_(p);
    let kind = match p.current() {
        // test slice_type
        // type T = [()];
        R_BRACK => {
            p.bump();
            SLICE_TYPE
        }

        // test array_type
        // type T = [(); 92];
        SEMI => {
            p.bump();
            expressions::expr(p);
            p.expect(R_BRACK);
            ARRAY_TYPE
        }
        // test_err array_type_missing_semi
        // type T = [() 92];
        _ => {
            p.error("expected `;` or `]`");
            SLICE_TYPE
        }
    };
    m.complete(p, kind);
}

// test reference_type;
// type A = &();
// type B = &'static ();
// type C = &mut ();
fn reference_type(p: &mut Parser) {
    assert!(p.at(AMP));
    let m = p.start();
    p.bump();
    p.eat(LIFETIME);
    p.eat(MUT_KW);
    type_no_bounds(p);
    m.complete(p, REFERENCE_TYPE);
}

// test placeholder_type
// type Placeholder = _;
fn placeholder_type(p: &mut Parser) {
    assert!(p.at(UNDERSCORE));
    let m = p.start();
    p.bump();
    m.complete(p, PLACEHOLDER_TYPE);
}

// test fn_pointer_type
// type A = fn();
// type B = unsafe fn();
// type C = unsafe extern "C" fn();
fn fn_pointer_type(p: &mut Parser) {
    let m = p.start();
    p.eat(UNSAFE_KW);
    if p.at(EXTERN_KW) {
        abi(p);
    }
    // test_err fn_pointer_type_missing_fn
    // type F = unsafe ();
    if !p.eat(FN_KW) {
        m.abandon(p);
        p.error("expected `fn`");
        return;
    }
    if p.at(L_PAREN) {
        params::param_list_opt_patterns(p);
    } else {
        p.error("expected parameters")
    }
    // test fn_pointer_type_with_ret
    // type F = fn() -> ();
    opt_fn_ret_type(p);
    m.complete(p, FN_POINTER_TYPE);
}

pub(super) fn for_binder(p: &mut Parser) {
    assert!(p.at(FOR_KW));
    p.bump();
    if p.at(L_ANGLE) {
        type_params::opt_type_param_list(p);
    } else {
        p.error("expected `<`");
    }
}

// test for_type
// type A = for<'a> fn() -> ();
pub(super) fn for_type(p: &mut Parser) {
    assert!(p.at(FOR_KW));
    let m = p.start();
    for_binder(p);
    match p.current() {
        FN_KW | UNSAFE_KW | EXTERN_KW => fn_pointer_type(p),
        _ if paths::is_path_start(p) => path_type_(p, false),
        _ => p.error("expected a path"),
    }
    m.complete(p, FOR_TYPE);
}

// test impl_trait_type
// type A = impl Iterator<Item=Foo<'a>> + 'a;
fn impl_trait_type(p: &mut Parser) {
    assert!(p.at(IMPL_KW));
    let m = p.start();
    p.bump();
    type_params::bounds_without_colon(p);
    m.complete(p, IMPL_TRAIT_TYPE);
}

// test dyn_trait_type
// type A = dyn Iterator<Item=Foo<'a>> + 'a;
fn dyn_trait_type(p: &mut Parser) {
    assert!(p.at(DYN_KW));
    let m = p.start();
    p.bump();
    type_params::bounds_without_colon(p);
    m.complete(p, DYN_TRAIT_TYPE);
}

// test path_type
// type A = Foo;
// type B = ::Foo;
// type C = self::Foo;
// type D = super::Foo;
pub(super) fn path_type(p: &mut Parser) {
    path_type_(p, true)
}

pub(super) fn path_type_(p: &mut Parser, allow_bounds: bool) {
    assert!(paths::is_path_start(p) || p.at(L_ANGLE));
    let m = p.start();
    paths::type_path(p);
    // test path_type_with_bounds
    // fn foo() -> Box<T + 'f> {}
    if allow_bounds && p.eat(PLUS) {
        type_params::bounds_without_colon(p);
    }
    m.complete(p, PATH_TYPE);
}
