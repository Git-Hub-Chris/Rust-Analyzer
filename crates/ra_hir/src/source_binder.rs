/// Lookup hir elements using positions in the source code. This is a lossy
/// transformation: in general, a single source might correspond to several
/// modules, functions, etc, due to macros, cfgs and `#[path=]` attributes on
/// modules.
///
/// So, this modules should not be used during hir construction, it exists
/// purely for "IDE needs".
use ra_db::{FileId, FilePosition};
use ra_syntax::{
    SmolStr, TextRange, SyntaxNode,
    ast::{self, AstNode, NameOwner},
    algo::find_node_at_offset,
};

use crate::{
    HirDatabase, Function, SourceItemId,
    DefKind, DefLoc, AsName, Module,
};

/// Locates the module by `FileId`. Picks topmost module in the file.
pub fn module_from_file_id(db: &impl HirDatabase, file_id: FileId) -> Option<Module> {
    let module_source = SourceItemId {
        file_id: file_id.into(),
        item_id: None,
    };
    module_from_source(db, module_source)
}

/// Locates the child module by `mod child;` declaration.
pub fn module_from_declaration(
    db: &impl HirDatabase,
    file_id: FileId,
    decl: &ast::Module,
) -> Option<Module> {
    let parent_module = module_from_file_id(db, file_id);
    let child_name = decl.name();
    match (parent_module, child_name) {
        (Some(parent_module), Some(child_name)) => parent_module.child(db, &child_name.as_name()),
        _ => None,
    }
}

/// Locates the module by position in the source code.
pub fn module_from_position(db: &impl HirDatabase, position: FilePosition) -> Option<Module> {
    let file = db.source_file(position.file_id);
    match find_node_at_offset::<ast::Module>(file.syntax(), position.offset) {
        Some(m) if !m.has_semi() => module_from_inline(db, position.file_id.into(), m),
        _ => module_from_file_id(db, position.file_id.into()),
    }
}

fn module_from_inline(
    db: &impl HirDatabase,
    file_id: FileId,
    module: &ast::Module,
) -> Option<Module> {
    assert!(!module.has_semi());
    let file_id = file_id.into();
    let file_items = db.file_items(file_id);
    let item_id = file_items.id_of(file_id, module.syntax());
    let source = SourceItemId {
        file_id,
        item_id: Some(item_id),
    };
    module_from_source(db, source)
}

/// Locates the module by child syntax element within the module
pub fn module_from_child_node(
    db: &impl HirDatabase,
    file_id: FileId,
    child: &SyntaxNode,
) -> Option<Module> {
    if let Some(m) = child
        .ancestors()
        .filter_map(ast::Module::cast)
        .find(|it| !it.has_semi())
    {
        module_from_inline(db, file_id.into(), m)
    } else {
        module_from_file_id(db, file_id.into())
    }
}

fn module_from_source(db: &impl HirDatabase, source: SourceItemId) -> Option<Module> {
    let source_root_id = db.file_source_root(source.file_id.as_original_file());
    let module_tree = db.module_tree(source_root_id);
    let module_id = module_tree.find_module_by_source(source)?;
    Some(Module::from_module_id(db, source_root_id, module_id))
}

pub fn function_from_position(db: &impl HirDatabase, position: FilePosition) -> Option<Function> {
    let file = db.source_file(position.file_id);
    let fn_def = find_node_at_offset::<ast::FnDef>(file.syntax(), position.offset)?;
    function_from_source(db, position.file_id, fn_def)
}

pub fn function_from_source(
    db: &impl HirDatabase,
    file_id: FileId,
    fn_def: &ast::FnDef,
) -> Option<Function> {
    let module = module_from_child_node(db, file_id, fn_def.syntax())?;
    let res = function_from_module(db, &module, fn_def);
    Some(res)
}

pub fn function_from_module(
    db: &impl HirDatabase,
    module: &Module,
    fn_def: &ast::FnDef,
) -> Function {
    let loc = module.def_id.loc(db);
    let file_id = loc.source_item_id.file_id;
    let file_items = db.file_items(file_id);
    let item_id = file_items.id_of(file_id, fn_def.syntax());
    let source_item_id = SourceItemId {
        file_id,
        item_id: Some(item_id),
    };
    let def_loc = DefLoc {
        kind: DefKind::Function,
        source_root_id: loc.source_root_id,
        module_id: loc.module_id,
        source_item_id,
    };
    Function::new(def_loc.id(db))
}

pub fn function_from_child_node(
    db: &impl HirDatabase,
    file_id: FileId,
    node: &SyntaxNode,
) -> Option<Function> {
    let fn_def = node.ancestors().find_map(ast::FnDef::cast)?;
    function_from_source(db, file_id, fn_def)
}

pub fn macro_symbols(db: &impl HirDatabase, file_id: FileId) -> Vec<(SmolStr, TextRange)> {
    let module = match module_from_file_id(db, file_id) {
        Some(it) => it,
        None => return Vec::new(),
    };
    let loc = module.def_id.loc(db);
    let items = db.lower_module_module(loc.source_root_id, loc.module_id);
    let mut res = Vec::new();

    for macro_call_id in items
        .items
        .iter()
        .filter_map(|it| it.id.file_id.as_macro_call_id())
    {
        if let Some(exp) = db.expand_macro_invocation(macro_call_id) {
            let loc = macro_call_id.loc(db);
            let syntax = db.file_item(loc.source_item_id);
            let macro_call = ast::MacroCall::cast(&syntax).unwrap();
            let off = macro_call.token_tree().unwrap().syntax().range().start();
            let file = exp.file();
            for trait_def in file.syntax().descendants().filter_map(ast::TraitDef::cast) {
                if let Some(name) = trait_def.name() {
                    let dst_range = name.syntax().range();
                    if let Some(src_range) = exp.map_range_back(dst_range) {
                        res.push((name.text().clone(), src_range + off))
                    }
                }
            }
        }
    }

    res
}
