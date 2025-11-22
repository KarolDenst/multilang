use crate::grammar::Rule;

use crate::error::RuntimeError;
use crate::node::{Context, Node, ParsedChildren, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct MapNode {
    pub entries: Vec<(String, Box<dyn Node>)>, // Key (literal) -> Value Expr
                                               // Note: For now, keys in literals are strings or identifiers.
                                               // If we want dynamic keys in literals (e.g. { key_var: val }), we'd need Box<dyn Node> for keys too.
                                               // The grammar plan said "Key = String | Identifier".
                                               // Let's stick to string keys for now.
}

impl Node for MapNode {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let mut map = HashMap::new();
        for (key, value_node) in &self.entries {
            let value = value_node.run(ctx)?;
            map.insert(key.clone(), value);
        }
        Ok(Value::Map(Rc::new(RefCell::new(map))))
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // MapLiteral = "{" MapEntries "}"
        // MapLiteral = "{" "}"
        // MapEntries = MapEntry "," MapEntries
        // MapEntries = MapEntry
        // MapEntry = Key ":" Expr

        // The parser flattens recursive rules if we handle them right.
        // But here, MapEntries is recursive.
        // We need a helper node for MapEntries or handle it here.
        // Let's assume we have a MapEntriesNode that returns a list of entries,
        // OR we can just parse it manually if the parser structure allows.

        // Actually, looking at ListNode/ElementsNode, we used a separate node for the recursive part.
        // Let's do the same: MapEntriesNode.

        if let Some(entries_node) = children.take_child("MapEntries") {
            // This child should be a MapEntriesNode which we need to define.
            // But wait, `take_child` returns Box<dyn Node>. We can't easily downcast to concrete type in Rust without Any.
            // In ListNode, we used `into_list_elements()`.
            // We should add `into_map_entries()` to Node trait.
            let entries = entries_node.into_map_entries();
            Box::new(MapNode { entries })
        } else {
            // Empty map
            Box::new(MapNode { entries: vec![] })
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct MapEntriesNode {
    pub entries: Vec<(String, Box<dyn Node>)>,
}

impl Node for MapEntriesNode {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        // Should not be run directly
        Ok(Value::Void)
    }

    fn is_map_entries(&self) -> bool {
        true
    }

    fn into_map_entries(self: Box<Self>) -> Vec<(String, Box<dyn Node>)> {
        self.entries
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // MapEntries = MapEntry "," MapEntries
        // MapEntries = MapEntry

        let mut entries = Vec::new();

        // First child is MapEntry
        if let Some(entry_node) = children.take_child("MapEntry") {
            entries.extend(entry_node.into_map_entries());
        }

        // Second child (optional) is MapEntries
        if let Some(rest_node) = children.take_child("MapEntries") {
            entries.extend(rest_node.into_map_entries());
        }

        Box::new(MapEntriesNode { entries })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct MapEntryNode {
    pub key: String,
    pub value: Box<dyn Node>,
}

impl Node for MapEntryNode {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        Ok(Value::Void)
    }

    fn is_map_entries(&self) -> bool {
        true // It's a single entry, can be treated as a list of entries
    }

    fn into_map_entries(self: Box<Self>) -> Vec<(String, Box<dyn Node>)> {
        vec![(self.key, self.value)]
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // MapEntry = Key ":" Expr
        // Key = String | Identifier

        let key_node = children.take_child("Key").expect("MapEntry missing Key");
        // Key node is likely a wrapper around Literal (String) or Variable (Identifier).
        // We need to get the text from its child if it doesn't have text itself.
        // But wait, the parser returns "Key" node which is just the child returned by the rule.
        // In parser.rs, "Key" rule returns the child directly:
        // "Key" => parsed_children.remaining().into_iter().next().unwrap().1

        // So key_node IS the Literal or Variable node.
        // Literal and Variable nodes should implement text().
        // Let's check if they do.
        // Variable implements text(). Literal does too (inherited from RawTokenNode or implemented?).
        // Wait, Literal is a struct wrapping value. It might not implement text().
        // Let's check Literal implementation.

        // Assuming key_node.text() works, let's debug why it failed.
        // Maybe it's because "String" rule returns a Literal node, and Literal node's text() returns None?

        let key = key_node
            .text()
            .or_else(|| {
                // Fallback: if it's a Literal, maybe we can get text from it?
                // Or maybe we need to change how Literal/Variable are parsed to preserve text.
                None
            })
            .expect("Key must have text");

        let clean_key = if key.starts_with('"') && key.ends_with('"') {
            key[1..key.len() - 1].to_string()
        } else {
            key
        };

        let value_node = children.take_child("Expr").expect("MapEntry missing Expr");

        Box::new(MapEntryNode {
            key: clean_key,
            value: value_node,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
