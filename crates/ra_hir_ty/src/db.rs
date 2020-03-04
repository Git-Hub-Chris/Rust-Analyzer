//! FIXME: write short doc here

use std::sync::Arc;

use hir_def::{
    db::DefDatabase, DefWithBodyId, FunctionId, GenericDefId, ImplId, LocalStructFieldId, TraitId,
    TypeParamId, VariantId,
};
use ra_arena::map::ArenaMap;
use ra_db::{impl_intern_key, salsa, CrateId};
use ra_prof::profile;

use crate::{
    method_resolution::CrateImplDefs,
    traits::{chalk, AssocTyValue, Impl},
    Binders, CallableDef, GenericPredicate, ImplTraitId, InferenceResult, PolyFnSig,
    ReturnTypeImplTraits, Substs, TraitRef, Ty, TyDefId, TypeCtor, ValueTyDefId,
};

#[salsa::query_group(HirDatabaseStorage)]
#[salsa::requires(salsa::Database)]
pub trait HirDatabase: DefDatabase {
    #[salsa::transparent]
    fn infer(&self, def: DefWithBodyId) -> Arc<InferenceResult>;

    #[salsa::invoke(crate::do_infer_query)]
    fn do_infer(&self, def: DefWithBodyId) -> Arc<InferenceResult>;

    #[salsa::invoke(crate::lower::ty_query)]
    #[salsa::cycle(crate::lower::ty_recover)]
    fn ty(&self, def: TyDefId) -> Binders<Ty>;

    #[salsa::invoke(crate::lower::value_ty_query)]
    fn value_ty(&self, def: ValueTyDefId) -> Binders<Ty>;

    #[salsa::invoke(crate::lower::impl_self_ty_query)]
    #[salsa::cycle(crate::lower::impl_self_ty_recover)]
    fn impl_self_ty(&self, def: ImplId) -> Binders<Ty>;

    #[salsa::invoke(crate::lower::impl_trait_query)]
    fn impl_trait(&self, def: ImplId) -> Option<Binders<TraitRef>>;

    #[salsa::invoke(crate::lower::field_types_query)]
    fn field_types(&self, var: VariantId) -> Arc<ArenaMap<LocalStructFieldId, Binders<Ty>>>;

    #[salsa::invoke(crate::callable_item_sig)]
    fn callable_item_signature(&self, def: CallableDef) -> PolyFnSig;

    #[salsa::invoke(crate::return_type_impl_traits)]
    fn return_type_impl_traits(&self, def: FunctionId) -> Option<Arc<ReturnTypeImplTraits>>;

    #[salsa::invoke(crate::lower::generic_predicates_for_param_query)]
    #[salsa::cycle(crate::lower::generic_predicates_for_param_recover)]
    fn generic_predicates_for_param(
        &self,
        param_id: TypeParamId,
    ) -> Arc<[Binders<GenericPredicate>]>;

    #[salsa::invoke(crate::lower::generic_predicates_query)]
    fn generic_predicates(&self, def: GenericDefId) -> Arc<[Binders<GenericPredicate>]>;

    #[salsa::invoke(crate::lower::generic_defaults_query)]
    fn generic_defaults(&self, def: GenericDefId) -> Substs;

    #[salsa::invoke(crate::method_resolution::CrateImplDefs::impls_in_crate_query)]
    fn impls_in_crate(&self, krate: CrateId) -> Arc<CrateImplDefs>;

    #[salsa::invoke(crate::traits::impls_for_trait_query)]
    fn impls_for_trait(&self, krate: CrateId, trait_: TraitId) -> Arc<[ImplId]>;

    /// This provides the Chalk trait solver instance. Because Chalk always
    /// works from a specific crate, this query is keyed on the crate; and
    /// because Chalk does its own internal caching, the solver is wrapped in a
    /// Mutex and the query does an untracked read internally, to make sure the
    /// cached state is thrown away when input facts change.
    #[salsa::invoke(crate::traits::trait_solver_query)]
    fn trait_solver(&self, krate: CrateId) -> crate::traits::TraitSolver;

    // Interned IDs for Chalk integration
    #[salsa::interned]
    fn intern_type_ctor(&self, type_ctor: TypeCtor) -> crate::TypeCtorId;
    #[salsa::interned]
    fn intern_type_param_id(&self, param_id: TypeParamId) -> GlobalTypeParamId;
    #[salsa::interned]
    fn intern_impl_trait_id(&self, id: ImplTraitId) -> ChalkImplTraitId;
    #[salsa::interned]
    fn intern_chalk_impl(&self, impl_: Impl) -> crate::traits::GlobalImplId;
    #[salsa::interned]
    fn intern_assoc_ty_value(&self, assoc_ty_value: AssocTyValue) -> crate::traits::AssocTyValueId;

    #[salsa::invoke(chalk::associated_ty_data_query)]
    fn associated_ty_data(&self, id: chalk::AssocTypeId) -> Arc<chalk::AssociatedTyDatum>;

    #[salsa::invoke(chalk::trait_datum_query)]
    fn trait_datum(&self, krate: CrateId, trait_id: chalk::TraitId) -> Arc<chalk::TraitDatum>;

    #[salsa::invoke(chalk::struct_datum_query)]
    fn struct_datum(&self, krate: CrateId, struct_id: chalk::StructId) -> Arc<chalk::StructDatum>;

    #[salsa::invoke(crate::traits::chalk::impl_datum_query)]
    fn impl_datum(&self, krate: CrateId, impl_id: chalk::ImplId) -> Arc<chalk::ImplDatum>;

    #[salsa::invoke(crate::traits::chalk::associated_ty_value_query)]
    fn associated_ty_value(
        &self,
        krate: CrateId,
        id: chalk::AssociatedTyValueId,
    ) -> Arc<chalk::AssociatedTyValue>;

    #[salsa::invoke(crate::traits::trait_solve_query)]
    fn trait_solve(
        &self,
        krate: CrateId,
        goal: crate::Canonical<crate::InEnvironment<crate::Obligation>>,
    ) -> Option<crate::traits::Solution>;
}

fn infer(db: &impl HirDatabase, def: DefWithBodyId) -> Arc<InferenceResult> {
    let _p = profile("infer");
    db.do_infer(def)
}

#[test]
fn hir_database_is_object_safe() {
    fn _assert_object_safe(_: &dyn HirDatabase) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalTypeParamId(salsa::InternId);
impl_intern_key!(GlobalTypeParamId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChalkImplTraitId(salsa::InternId);
impl_intern_key!(ChalkImplTraitId);
