use std::iter;

use cairo_lang_defs::ids::{
    LanguageElementId, LookupItemId, ModuleItemId, NamedLanguageElementId,
    TopLevelLanguageElementId, TraitItemId,
};
use cairo_lang_doc::db::DocGroup;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_utils::smol_str::SmolStr;
use itertools::Itertools;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Information about the definition of an item (function, trait, impl, module, etc.).
#[derive(Eq, PartialEq, Debug)]
pub struct ItemDef {
    /// The [`LookupItemId`] associated with the item.
    lookup_item_id: LookupItemId,

    /// Parent item to use as context when building signatures, etc.
    ///
    /// Sometimes, a signature of an item, it might contain parts that are defined elsewhere.
    /// For example, for trait/impl items,
    /// signature may refer to generic params defined in the defining trait/impl.
    /// This reference allows including simplified signatures of such contexts alongside
    /// the signature of this item.
    context_items: Vec<LookupItemId>,

    definition_stable_ptr: SyntaxStablePtrId,
}

impl ItemDef {
    /// Constructs new [`ItemDef`] instance.
    pub(super) fn new(db: &AnalysisDatabase, definition_node: SyntaxNode) -> Option<Self> {
        let mut lookup_item_ids =
            db.collect_lookup_items_with_parent_files(definition_node)?.into_iter();

        // Pull the lookup item representing the defining item.
        let lookup_item_id = lookup_item_ids.next()?;

        // Collect context items.
        let context_items = lookup_item_ids
            .take_while(|item| {
                matches!(
                    item,
                    LookupItemId::ModuleItem(ModuleItemId::Struct(_))
                        | LookupItemId::ModuleItem(ModuleItemId::Enum(_))
                        | LookupItemId::ModuleItem(ModuleItemId::Trait(_))
                        | LookupItemId::ModuleItem(ModuleItemId::Impl(_))
                        | LookupItemId::TraitItem(TraitItemId::Impl(_))
                )
            })
            .collect();

        Some(Self {
            lookup_item_id,
            context_items,
            definition_stable_ptr: definition_node.stable_ptr(db),
        })
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId {
        self.definition_stable_ptr
    }

    /// Get item signature without its body including signatures of its contexts.
    pub fn signature(&self, db: &AnalysisDatabase) -> String {
        let contexts = self.context_items.iter().copied().rev();
        let this = iter::once(self.lookup_item_id);
        contexts.chain(this).filter_map(|item| db.get_item_signature(item.into())).join("\n")
    }

    /// Get item signature without its body including signatures of its contexts. Also adds text for this item only.
    pub fn signature_with_text(&self, db: &AnalysisDatabase, text: &str) -> String {
        let this = db.get_item_signature(self.lookup_item_id.into()).unwrap_or_else(|| "".into());

        let contexts = self.context_items.iter().copied().rev();
        let contexts = contexts.filter_map(|item| db.get_item_signature(item.into())).join("\n");

        format!("{this}{text}{contexts}")
    }

    /// Gets item documentation in a final form usable for display.
    pub fn documentation(&self, db: &AnalysisDatabase) -> Option<String> {
        db.get_item_documentation(self.lookup_item_id.into())
    }

    /// Gets the full path (including crate name and defining trait/impl if applicable)
    /// to the module containing the item.
    pub fn definition_path(&self, db: &AnalysisDatabase) -> String {
        match self.lookup_item_id {
            LookupItemId::ModuleItem(item) => item.parent_module(db).full_path(db),
            LookupItemId::TraitItem(item) => item.trait_id(db).full_path(db),
            LookupItemId::ImplItem(item) => item.impl_def_id(db).full_path(db),
        }
    }

    /// Gets the name of the item.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        match self.lookup_item_id {
            LookupItemId::ModuleItem(item) => item.name(db),
            LookupItemId::TraitItem(item) => item.name(db),
            LookupItemId::ImplItem(item) => item.name(db),
        }
    }
}
